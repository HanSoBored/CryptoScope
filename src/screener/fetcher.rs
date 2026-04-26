use chrono::Utc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{sleep, timeout};
use tracing::{info, warn};

use crate::cli::ScreenerMode;
use crate::db::{Database, OpenPriceRow};
use crate::error::Result;
use crate::exchange::Exchange;
use crate::models::{DailyKline, Ticker};

/// Maximum concurrent kline requests to respect rate limits.
const MAX_CONCURRENT_KLINE_REQUESTS: usize = 2;

/// Per-request delay to respect Bybit rate limits (120 req/min = 500ms/request).
const PER_REQUEST_DELAY_MS: u64 = 500;

/// Timeout for individual kline fetch tasks.
const KLINE_FETCH_TIMEOUT: Duration = Duration::from_secs(30);

/// Maximum retry attempts for rate-limited requests.
const MAX_KLINE_RETRIES: u32 = 3;

/// Initial backoff delay for retries (doubles each attempt).
const KLINE_RETRY_BASE_DELAY_MS: u64 = 1000;

/// Async task that fetches a single kline with rate-limit delay, semaphore, and retry logic.
async fn fetch_single_kline(
    symbol: String,
    category: String,
    sem: Arc<Semaphore>,
    exch: Arc<dyn Exchange>,
) -> Result<DailyKline> {
    // Acquire semaphore permit first, then apply per-request delay
    let _permit = sem
        .acquire()
        .await
        .map_err(|e| crate::error::CryptoScopeError::ApiError {
            code: -1,
            message: format!("Semaphore closed while acquiring permit for {symbol}: {e}"),
        })?;

    // Per-request delay to stay within rate limits (applied to active requests)
    sleep(Duration::from_millis(PER_REQUEST_DELAY_MS)).await;

    // Retry with exponential backoff on failure
    let fetch = || async { exch.fetch_daily_kline(&symbol, &category).await };
    retry_with_backoff(fetch, &symbol).await
}

/// Retry an async operation with exponential backoff.
async fn retry_with_backoff<F, Fut, T>(mut operation: F, symbol: &str) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut last_err = None;
    for attempt in 0..=MAX_KLINE_RETRIES {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(e) => {
                last_err = Some(e);
                if attempt < MAX_KLINE_RETRIES {
                    let backoff_ms = KLINE_RETRY_BASE_DELAY_MS * 2u64.pow(attempt);
                    warn!(
                        "Retry {}/{} for symbol '{}' after {}ms: {}",
                        attempt + 1,
                        MAX_KLINE_RETRIES,
                        symbol,
                        backoff_ms,
                        last_err.as_ref().expect("retry error should exist")
                    );
                    sleep(Duration::from_millis(backoff_ms)).await;
                }
            }
        }
    }

    Err(
        last_err.unwrap_or_else(|| crate::error::CryptoScopeError::ApiError {
            code: -1,
            message: format!(
                "All {} retries exhausted for symbol '{}'",
                MAX_KLINE_RETRIES, symbol
            ),
        }),
    )
}

/// Log progress every 50 symbols.
fn log_fetch_progress(processed: usize, total_symbols: usize) {
    if processed.is_multiple_of(50) {
        info!(
            "Kline fetch progress: {}/{} symbols ({:.1}%)",
            processed,
            total_symbols,
            ((processed as f64) / (total_symbols as f64) * 100.0)
        );
    }
}

/// Log final summary and warn if failure rate exceeds 10%.
fn log_fetch_summary(success_count: usize, failed_count: usize, total_symbols: usize) {
    info!(
        "Kline fetch completed: {} successful, {} failed out of {} total symbols",
        success_count, failed_count, total_symbols
    );

    if total_symbols > 0 && (failed_count as f64 / total_symbols as f64) > 0.1 {
        warn!(
            "High failure rate: {:.1}% of k-line fetches failed ({} out of {})",
            (failed_count as f64 / total_symbols as f64) * 100.0,
            failed_count,
            total_symbols
        );
    }
}

pub struct OpenPriceFetcher<'a> {
    db: &'a mut Database,
    exchange: Arc<dyn Exchange>,
    /// Captured at fetcher creation to avoid midnight race conditions
    /// between date check and save operations.
    fetch_date: String,
    fetch_timestamp: i64,
}

impl<'a> OpenPriceFetcher<'a> {
    pub fn new(db: &'a mut Database, exchange: Arc<dyn Exchange>) -> Self {
        let now = Utc::now();
        Self {
            db,
            exchange,
            fetch_date: now.format("%Y-%m-%d").to_string(),
            fetch_timestamp: now.timestamp(),
        }
    }

    /// Check whether open prices need to be fetched.
    ///
    /// Returns `true` if no prices are stored or if the stored data is from a
    /// different date than today. Call this before `fetch_and_save_open_prices`
    /// to avoid redundant API calls.
    pub fn should_fetch_open_prices(&self) -> Result<bool> {
        let stored_date = self.db.get_stored_date()?;

        match stored_date {
            None => Ok(true),
            Some(date) => Ok(date != self.fetch_date),
        }
    }

    /// Fetch open prices for the given category and save them to the database.
    ///
    /// Uses either ticker mode (fast, rolling 24h) or kline mode (accurate, true
    /// daily open) depending on the `ScreenerMode`. Call this after
    /// `should_fetch_open_prices` returns `true`.
    pub async fn fetch_and_save_open_prices(
        &mut self,
        mode: ScreenerMode,
        category: &str,
    ) -> Result<()> {
        let prices = match mode {
            ScreenerMode::Ticker => self.fetch_ticker_mode(category).await?,
            ScreenerMode::Kline => self.fetch_kline_mode(category).await?,
        };

        self.save_open_prices(prices)?;
        Ok(())
    }

    async fn fetch_ticker_mode(&self, category: &str) -> Result<Vec<(String, f64)>> {
        let tickers = self.exchange.fetch_tickers(category).await?;
        Ok(self.extract_open_prices_from_tickers(&tickers))
    }

    async fn fetch_kline_mode(&self, category: &str) -> Result<Vec<(String, f64)>> {
        let tickers = self.exchange.fetch_tickers(category).await?;
        let total_symbols = tickers.len();

        info!(
            "Kline mode: Fetching daily k-lines for {} symbols (streaming, max {} concurrent)...",
            total_symbols, MAX_CONCURRENT_KLINE_REQUESTS
        );

        let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_KLINE_REQUESTS));

        // Stream kline fetches using a bounded channel to avoid storing all JoinHandles in memory.
        // We spawn tasks in chunks and collect results as they complete.
        Self::stream_kline_results(&tickers, &semaphore, &self.exchange, total_symbols).await
    }

    /// Stream kline fetches with true concurrency using bounded semaphore and channel.
    /// Spawns all tasks concurrently but limits active requests via semaphore.
    /// Results are collected through a channel as they complete.
    async fn stream_kline_results(
        tickers: &[Ticker],
        semaphore: &Arc<Semaphore>,
        exchange: &Arc<dyn Exchange>,
        total_symbols: usize,
    ) -> Result<Vec<(String, f64)>> {
        let (tx, mut rx) = tokio::sync::mpsc::channel(total_symbols);
        let results = Arc::new(Mutex::new(Vec::with_capacity(total_symbols)));
        let failed_count = AtomicUsize::new(0);
        let processed = AtomicUsize::new(0);

        // Spawn all tasks concurrently — semaphore bounds active requests
        for ticker in tickers {
            let symbol = ticker.symbol.clone();
            let ticker_category = ticker.category.clone();
            let sem = semaphore.clone();
            let exch = exchange.clone();
            let tx = tx.clone();

            tokio::spawn(async move {
                let result = timeout(
                    KLINE_FETCH_TIMEOUT,
                    fetch_single_kline(symbol.clone(), ticker_category, sem, exch),
                )
                .await;

                let open_price = match result {
                    Ok(Ok(kline)) => Some(kline.open_price),
                    Ok(Err(e)) => {
                        warn!("Failed to fetch k-line for symbol '{}': {}", symbol, e);
                        None
                    }
                    Err(_) => {
                        warn!(
                            "Timeout ({:?}) fetching k-line for symbol '{}'",
                            KLINE_FETCH_TIMEOUT, symbol
                        );
                        None
                    }
                };

                let _ = tx.send((symbol, open_price)).await;
            });
        }

        // Drop the original sender so the channel closes when all tasks complete
        drop(tx);

        // Collect results as they arrive
        while let Some((symbol, price)) = rx.recv().await {
            if let Some(open_price) = price {
                results.lock().await.push((symbol, open_price));
            } else {
                failed_count.fetch_add(1, Ordering::Relaxed);
            }

            let processed_val = processed.fetch_add(1, Ordering::Relaxed) + 1;
            log_fetch_progress(processed_val, total_symbols);
        }

        let failed = failed_count.load(Ordering::Relaxed);
        let success = results.lock().await.len();
        log_fetch_summary(success, failed, total_symbols);

        let final_results = Arc::try_unwrap(results)
            .expect("results Arc should have single owner after channel drain")
            .into_inner();

        Ok(final_results)
    }

    fn extract_open_prices_from_tickers(&self, tickers: &[Ticker]) -> Vec<(String, f64)> {
        tickers
            .iter()
            .filter(|t| t.prev_price_24h > 0.0)
            .map(|t| (t.symbol.clone(), t.prev_price_24h))
            .collect()
    }

    fn save_open_prices(&mut self, prices: Vec<(String, f64)>) -> Result<()> {
        let rows: Vec<OpenPriceRow> = prices
            .into_iter()
            .map(|(symbol, open_price)| OpenPriceRow {
                symbol,
                open_price,
                fetch_date: self.fetch_date.clone(),
                fetch_timestamp: self.fetch_timestamp,
                source: "bybit".to_string(),
            })
            .collect();

        self.db.save_open_prices(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_test_db;

    #[test]
    fn test_should_fetch_when_no_data() {
        let mut db = create_test_db();
        let exchange = Arc::new(crate::exchange::bybit::BybitClient::new());
        let fetcher = OpenPriceFetcher::new(&mut db, exchange);
        // No data stored, should fetch
        assert!(fetcher.should_fetch_open_prices().unwrap());
    }

    #[test]
    fn test_should_fetch_when_stale_date() {
        let mut db = create_test_db();
        // Insert stale data (yesterday)
        let yesterday = (Utc::now() - chrono::Duration::days(1))
            .format("%Y-%m-%d")
            .to_string();
        let row = OpenPriceRow {
            symbol: "BTCUSDT".to_string(),
            open_price: 50000.0,
            fetch_date: yesterday,
            fetch_timestamp: Utc::now().timestamp() - 86400,
            source: "bybit".to_string(),
        };
        db.save_open_prices(vec![row]).unwrap();

        let exchange = Arc::new(crate::exchange::bybit::BybitClient::new());
        let fetcher = OpenPriceFetcher::new(&mut db, exchange);
        // Stored date is yesterday, should fetch
        assert!(fetcher.should_fetch_open_prices().unwrap());
    }

    #[test]
    fn test_should_not_fetch_when_today() {
        let mut db = create_test_db();
        // Insert today's data
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let row = OpenPriceRow {
            symbol: "BTCUSDT".to_string(),
            open_price: 50000.0,
            fetch_date: today,
            fetch_timestamp: Utc::now().timestamp(),
            source: "bybit".to_string(),
        };
        db.save_open_prices(vec![row]).unwrap();

        let exchange = Arc::new(crate::exchange::bybit::BybitClient::new());
        let fetcher = OpenPriceFetcher::new(&mut db, exchange);
        // Stored date is today, should NOT fetch
        assert!(!fetcher.should_fetch_open_prices().unwrap());
    }
}
