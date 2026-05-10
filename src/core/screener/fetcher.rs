use chrono::Utc;
use futures::stream::{self, StreamExt};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::{sleep, timeout};
use tracing::{info, warn};

use crate::core::db::{Database, OpenPriceRow};
use crate::core::error::{CryptoScopeError, Result};
use crate::core::exchange::Exchange;
use crate::core::screener::mode::ScreenerMode;

/// Maximum concurrent kline requests to respect rate limits.
const MAX_CONCURRENT_KLINE_REQUESTS: usize = 5;

/// Timeout for individual kline fetch tasks.
const KLINE_FETCH_TIMEOUT: Duration = Duration::from_secs(60);

/// Maximum retry attempts for rate-limited requests.
const MAX_KLINE_RETRIES: u32 = 3;

/// Initial backoff delay for retries (doubles each attempt).
const KLINE_RETRY_BASE_DELAY_MS: u64 = 1000;

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

    Err(last_err.unwrap_or_else(|| CryptoScopeError::ApiError {
        code: -1,
        message: format!(
            "All {} retries exhausted for symbol '{}'",
            MAX_KLINE_RETRIES, symbol
        ),
    }))
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

/// Fetch a single kline for a ticker with timeout and retry logic.
/// Logs progress every 50 items.
async fn fetch_single_kline_with_progress(
    exchange: Arc<dyn Exchange>,
    ticker: crate::core::models::Ticker,
    index: usize,
    total: usize,
) -> Option<(String, f64)> {
    if index.is_multiple_of(50) {
        info!("Kline progress: {}/{}", index + 1, total);
    }

    let result = timeout(
        KLINE_FETCH_TIMEOUT,
        retry_with_backoff(
            || exchange.fetch_daily_kline(&ticker.symbol, &ticker.category),
            &ticker.symbol,
        ),
    )
    .await;

    match result {
        Ok(Ok(kline)) => Some((ticker.symbol, kline.open_price)),
        Ok(Err(e)) => {
            warn!("Failed kline for '{}': {}", ticker.symbol, e);
            None
        }
        Err(_) => {
            warn!("Timeout for '{}'", ticker.symbol);
            None
        }
    }
}

/// Shared state for fetcher operations with thread-safe database access.
///
/// Note: Uses `std::sync::Mutex` intentionally, not `tokio::sync::Mutex`.
/// Per Tokio best practices, std::sync::Mutex is preferred when the lock is not
/// held across `.await` points (which is the case here - locks are short-lived
/// and only used for database operations that don't involve async I/O).
pub struct OpenPriceFetcherShared {
    db: Arc<Mutex<Database>>,
    exchange: Arc<dyn Exchange>,
}

impl OpenPriceFetcherShared {
    pub fn new(db: Arc<Mutex<Database>>, exchange: Arc<dyn Exchange>) -> Self {
        Self { db, exchange }
    }

    /// Get the fetch date as YYYY-MM-DD format
    fn fetch_date(&self) -> String {
        Utc::now().format("%Y-%m-%d").to_string()
    }

    /// Get the current Unix timestamp
    fn fetch_timestamp(&self) -> i64 {
        Utc::now().timestamp()
    }

    /// Check whether open prices need to be fetched.
    pub fn should_fetch_open_prices(&self) -> Result<bool> {
        let db = self.db.lock().unwrap();
        let stored_date = db.get_stored_date()?;
        match stored_date {
            None => Ok(true),
            Some(date) => Ok(date != self.fetch_date()),
        }
    }

    /// Check if refresh is needed and fetch open prices if so.
    ///
    /// Combines the check-then-act pattern into a single method for convenience.
    /// Returns `true` if a refresh was performed, `false` if cache was still valid.
    pub async fn maybe_refresh(&self, mode: ScreenerMode, category: &str) -> Result<bool> {
        if self.should_fetch_open_prices()? {
            self.fetch_and_save_open_prices(mode, category).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Fetch open prices for the given category and save them to the database.
    ///
    /// Uses either ticker mode (fast, rolling 24h) or kline mode (accurate, true
    /// daily open) depending on the `ScreenerMode`. Handles database locking internally.
    pub async fn fetch_and_save_open_prices(
        &self,
        mode: ScreenerMode,
        category: &str,
    ) -> Result<()> {
        let prices = match mode {
            ScreenerMode::Ticker => self.fetch_ticker_mode(category).await?,
            ScreenerMode::Kline => self.fetch_kline_mode(category).await?,
        };

        self.save_open_prices(prices)
    }

    async fn fetch_ticker_mode(&self, category: &str) -> Result<Vec<(String, f64)>> {
        let tickers = self.exchange.fetch_tickers(category).await?;
        Ok(tickers
            .iter()
            .filter(|t| t.prev_price_24h > 0.0)
            .map(|t| (t.symbol.clone(), t.prev_price_24h))
            .collect())
    }

    async fn fetch_kline_mode(&self, category: &str) -> Result<Vec<(String, f64)>> {
        let tickers = self.exchange.fetch_tickers(category).await?;
        let total = tickers.len();

        info!(
            "Kline mode: Fetching daily k-lines for {} symbols (max {} concurrent)...",
            total, MAX_CONCURRENT_KLINE_REQUESTS
        );

        let exchange = self.exchange.clone();
        let results: Vec<_> = stream::iter(tickers.into_iter().enumerate())
            .map(|(i, ticker)| fetch_single_kline_with_progress(exchange.clone(), ticker, i, total))
            .buffer_unordered(MAX_CONCURRENT_KLINE_REQUESTS)
            .filter_map(|r| async move { r })
            .collect()
            .await;

        let success = results.len();
        let failed = total - success;
        log_fetch_summary(success, failed, total);

        Ok(results)
    }

    fn save_open_prices(&self, prices: Vec<(String, f64)>) -> Result<()> {
        let mut db = self.db.lock().unwrap();
        let rows: Vec<OpenPriceRow> = prices
            .into_iter()
            .map(|(symbol, open_price)| OpenPriceRow {
                symbol,
                open_price,
                fetch_date: self.fetch_date(),
                fetch_timestamp: self.fetch_timestamp(),
                source: crate::core::exchange::bybit::EXCHANGE_NAME.to_string(),
            })
            .collect();
        db.save_open_prices(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::exchange::bybit::BybitClient;
    use crate::core::test_utils::create_test_db;

    #[test]
    fn test_should_fetch_when_no_data() {
        let db = create_test_db();
        let exchange = Arc::new(BybitClient::new().unwrap());
        let fetcher = OpenPriceFetcherShared::new(Arc::new(Mutex::new(db)), exchange);
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
            source: crate::core::exchange::bybit::EXCHANGE_NAME.to_string(),
        };
        db.save_open_prices(vec![row]).unwrap();

        let exchange = Arc::new(BybitClient::new().unwrap());
        let fetcher = OpenPriceFetcherShared::new(Arc::new(Mutex::new(db)), exchange);
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
            source: crate::core::exchange::bybit::EXCHANGE_NAME.to_string(),
        };
        db.save_open_prices(vec![row]).unwrap();

        let exchange = Arc::new(BybitClient::new().unwrap());
        let fetcher = OpenPriceFetcherShared::new(Arc::new(Mutex::new(db)), exchange);
        // Stored date is today, should NOT fetch
        assert!(!fetcher.should_fetch_open_prices().unwrap());
    }
}
