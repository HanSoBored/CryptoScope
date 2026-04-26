//! Price screener module for detecting significant price changes.
//!
//! Fetches open prices (cached by date) and current tickers, then calculates
//! price changes with filtering, sorting, and formatted output.

use crate::cli::ScreenerMode;
use crate::db::Database;
use crate::error::Result;
use crate::exchange::Exchange;
use crate::models::PriceChange;
use crate::screener::fetcher::OpenPriceFetcher;
use std::sync::Arc;

pub mod calculator;
pub mod fetcher;
pub mod output;

pub use output::{display as display_output, display_stats as display_output_stats};

pub struct Screener {
    db: Database,
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
            db,
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
        let exchange = self.exchange.clone();
        let mut fetcher = OpenPriceFetcher::new(&mut self.db, exchange);

        if fetcher.should_fetch_open_prices()? {
            for category in &self.categories {
                fetcher
                    .fetch_and_save_open_prices(self.mode, category)
                    .await?;
            }
        }

        let open_prices = self.db.get_all_open_prices()?;

        let mut all_tickers = Vec::new();
        for category in &self.categories {
            let tickers = self.exchange.fetch_tickers(category).await?;
            all_tickers.extend(tickers);
        }

        let changes = calculator::calculate_all(open_prices, all_tickers);

        Ok(changes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::OpenPriceRow;
    use crate::models::{DailyKline, Ticker};
    use crate::test_utils::create_test_db;
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

        async fn fetch_instruments(&self, _category: &str) -> Result<Vec<crate::models::Symbol>> {
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
