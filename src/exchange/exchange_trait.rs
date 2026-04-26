use crate::error::Result;
use crate::models::{DailyKline, Symbol, Ticker};
use async_trait::async_trait;

/// Trait that all exchange clients must implement
#[async_trait]
pub trait Exchange: Send + Sync {
    /// Get the exchange name
    fn name(&self) -> &'static str;

    /// Fetch instruments for a specific category
    /// This method handles pagination internally
    async fn fetch_instruments(&self, category: &str) -> Result<Vec<Symbol>>;

    /// Fetch instruments for multiple categories
    #[allow(dead_code)]
    async fn fetch_all_instruments(&self, categories: &[&str]) -> Result<Vec<Symbol>> {
        let mut all_symbols = Vec::new();
        for category in categories {
            let symbols = self.fetch_instruments(category).await?;
            all_symbols.extend(symbols);
        }
        Ok(all_symbols)
    }

    /// Fetch all tickers for a category
    async fn fetch_tickers(&self, category: &str) -> Result<Vec<Ticker>>;

    /// Fetch daily k-line for a specific symbol and category
    async fn fetch_daily_kline(&self, symbol: &str, category: &str) -> Result<DailyKline>;
}
