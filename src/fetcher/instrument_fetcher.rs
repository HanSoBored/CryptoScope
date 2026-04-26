use crate::error::Result;
use crate::exchange::Exchange;
use crate::models::Symbol;
use tracing::info;

/// Fetch instruments for a single category.
#[allow(dead_code)]
pub async fn fetch_category(exchange: &dyn Exchange, category: &str) -> Result<Vec<Symbol>> {
    exchange.fetch_instruments(category).await
}

/// Fetch instruments for multiple categories.
pub async fn fetch_categories(exchange: &dyn Exchange, categories: &[&str]) -> Result<Vec<Symbol>> {
    let mut all_symbols = Vec::new();

    for category in categories {
        info!("Fetching category: {}", category);
        let symbols = exchange.fetch_instruments(category).await?;
        info!("  → Got {} symbols from {}", symbols.len(), category);
        all_symbols.extend(symbols);
    }

    Ok(all_symbols)
}
