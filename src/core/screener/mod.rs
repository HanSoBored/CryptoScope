//! Price screener module for detecting significant price changes.
//!
//! Fetches open prices (cached by date) and current tickers, then calculates
//! price changes with filtering, sorting, and formatted output.

use crate::core::db::Database;
use crate::core::error::Result;
use crate::core::exchange::Exchange;
use crate::core::models::PriceChange;
use std::sync::{Arc, Mutex};

pub mod calculator;
pub mod fetcher;
pub mod mode;
pub mod output;

pub use mode::ScreenerMode;

use fetcher::OpenPriceFetcherShared;

pub struct Screener {
    db: Arc<Mutex<Database>>,
    exchange: Arc<dyn Exchange>,
    mode: ScreenerMode,
    categories: Vec<String>,
}

impl Screener {
    pub fn new(
        db: Database,
        exchange: Arc<dyn Exchange>,
        mode: ScreenerMode,
        categories: Vec<String>,
    ) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            exchange,
            mode,
            categories,
        }
    }

    /// Run the screener: fetch open prices, current tickers, and calculate price changes.
    ///
    /// Checks if open prices need refreshing based on the stored date, fetches and saves
    /// them if needed (for each category), then fetches current tickers and calculates
    /// the price changes between open and current prices.
    pub async fn run(&mut self) -> Result<Vec<PriceChange>> {
        // Maybe refresh open prices (uses OpenPriceFetcherShared for date-check and fetch)
        self.maybe_refresh_open_prices().await?;

        // Load cached open prices from database
        let open_prices = self.load_open_prices()?;

        // Fetch current tickers for all categories
        let current_tickers = self.fetch_current_tickers().await?;

        // Calculate price changes
        Ok(calculator::calculate_all(open_prices, current_tickers))
    }

    /// Refresh open prices if cache is stale.
    ///
    /// Uses `OpenPriceFetcherShared::maybe_refresh()` which combines the date-check
    /// and fetch operations into a single method.
    async fn maybe_refresh_open_prices(&mut self) -> Result<()> {
        let exchange = self.exchange.clone();
        let mode = self.mode;
        let categories = self.categories.clone();
        let db = self.db.clone();

        let fetcher = OpenPriceFetcherShared::new(db, exchange);
        for category in &categories {
            fetcher.maybe_refresh(mode, category).await?;
        }

        Ok(())
    }

    /// Load cached open prices from database.
    fn load_open_prices(&self) -> Result<Vec<(String, f64)>> {
        let db = self.db.lock().unwrap();
        db.get_all_open_prices()
    }

    /// Fetch current tickers for all categories.
    async fn fetch_current_tickers(&self) -> Result<Vec<crate::core::models::Ticker>> {
        let mut all_tickers = Vec::new();
        for category in &self.categories {
            let tickers = self.exchange.fetch_tickers(category).await?;
            all_tickers.extend(tickers);
        }
        Ok(all_tickers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::db::OpenPriceRow;
    use crate::core::models::{DailyKline, Ticker};
    use crate::core::test_utils::create_test_db;
    use async_trait::async_trait;

    /// Mock exchange for testing the full screener flow.
    struct MockExchange {
        tickers: Vec<Ticker>,
        kline: DailyKline,
    }

    impl MockExchange {
        fn new(tickers: Vec<Ticker>, kline: DailyKline) -> Self {
            Self { tickers, kline }
        }
    }

    #[async_trait]
    impl Exchange for MockExchange {
        fn name(&self) -> &'static str {
            "mock"
        }

        async fn fetch_instruments(
            &self,
            _category: &str,
        ) -> Result<Vec<crate::core::models::Symbol>> {
            Ok(vec![])
        }

        async fn fetch_tickers(&self, _category: &str) -> Result<Vec<Ticker>> {
            Ok(self.tickers.clone())
        }

        async fn fetch_daily_kline(&self, _symbol: &str, _category: &str) -> Result<DailyKline> {
            Ok(self.kline.clone())
        }
    }

    #[tokio::test]
    async fn test_screener_cache_miss() {
        // Setup: Create a mock exchange with known data
        let tickers = vec![
            Ticker {
                symbol: "BTCUSDT".to_string(),
                category: "linear".to_string(),
                last_price: 52000.0,
                prev_price_24h: 49000.0,
                volume_24h: 1000.0,
            },
            Ticker {
                symbol: "ETHUSDT".to_string(),
                category: "linear".to_string(),
                last_price: 3100.0,
                prev_price_24h: 2900.0,
                volume_24h: 5000.0,
            },
        ];
        let kline = DailyKline {
            open_price: 50000.0,
        };

        let mock = MockExchange::new(tickers, kline);

        // Start with empty DB (cache miss — should fetch open prices)
        let db = create_test_db();
        let mut screener = Screener::new(
            db,
            Arc::new(mock),
            ScreenerMode::Kline,
            vec!["linear".to_string()],
        );

        // Run the screener
        let changes = screener.run().await.unwrap();

        // Verify: BTCUSDT should have price change calculated
        let btc_change = changes.iter().find(|c| c.symbol == "BTCUSDT").unwrap();
        assert!((btc_change.open_price - 50000.0).abs() < 0.01);
        assert!((btc_change.current_price - 52000.0).abs() < 0.01);
        assert!(btc_change.change_percent > 0.0);
    }

    #[tokio::test]
    async fn test_screener_cache_hit() {
        // Setup: DB with today's cached data (cache hit — should NOT re-fetch klines)
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let row = OpenPriceRow {
            symbol: "BTCUSDT".to_string(),
            open_price: 50000.0,
            fetch_date: today,
            fetch_timestamp: chrono::Utc::now().timestamp(),
            source: "mock".to_string(),
        };
        let mut db = create_test_db();
        db.save_open_prices(vec![row]).unwrap();

        let tickers = vec![Ticker {
            symbol: "BTCUSDT".to_string(),
            category: "linear".to_string(),
            last_price: 52000.0,
            prev_price_24h: 49000.0,
            volume_24h: 1000.0,
        }];
        let kline = DailyKline {
            open_price: 50000.0,
        };
        let mock = MockExchange::new(tickers, kline);

        let mut screener = Screener::new(
            db,
            Arc::new(mock),
            ScreenerMode::Kline,
            vec!["linear".to_string()],
        );

        // Run the screener — should use cached open prices
        let changes = screener.run().await.unwrap();

        // Verify: Should still get results (from cached open prices + fresh tickers)
        assert!(!changes.is_empty());
        let btc_change = changes.iter().find(|c| c.symbol == "BTCUSDT").unwrap();
        assert!((btc_change.open_price - 50000.0).abs() < 0.01);
    }

    #[test]
    fn test_screener_mode_default() {
        let mode = ScreenerMode::default();
        assert!(matches!(mode, ScreenerMode::Kline));
    }

    #[test]
    fn test_screener_mode_equality() {
        assert_eq!(ScreenerMode::Ticker, ScreenerMode::Ticker);
        assert_eq!(ScreenerMode::Kline, ScreenerMode::Kline);
        assert_ne!(ScreenerMode::Ticker, ScreenerMode::Kline);
    }
}
