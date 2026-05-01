//! Legacy CLI output formatting (deprecated - web API uses JSON responses).

#![allow(dead_code)]

use crate::core::error::Result;
use crate::core::models::{Statistics, Symbol};
use serde::Serialize;

/// JSON output structure for machine-readable output.
///
/// Contains all the data needed for a complete JSON report including
/// exchange info, categories, timestamp, statistics, and full symbol list.
#[derive(Serialize)]
pub struct JsonOutput {
    /// Name of the exchange (e.g., "bybit")
    pub exchange: String,
    /// List of categories included in the output
    pub categories: Vec<String>,
    /// ISO 8601 timestamp of when the report was generated
    pub timestamp: String,
    /// Aggregated statistics about the symbols
    pub statistics: Statistics,
    /// List of all symbols with their details
    pub symbols: Vec<JsonSymbol>,
}

/// Symbol information in JSON-serializable format
#[derive(Serialize)]
pub struct JsonSymbol {
    /// The symbol/ticker name (e.g., "BTCUSDT")
    pub symbol: String,
    /// Category of the instrument
    pub category: String,
    /// Type of contract
    pub contract_type: String,
    /// Base currency code
    pub base_coin: String,
    /// Quote currency code
    pub quote_coin: String,
}

impl JsonOutput {
    /// Create JSON output from fetched data
    ///
    /// Converts symbols and statistics into JSON-serializable format
    /// with exchange info and timestamp.
    pub fn new(
        exchange: &str,
        categories: &[&str],
        symbols: &[Symbol],
        stats: &Statistics,
    ) -> Self {
        let json_symbols: Vec<JsonSymbol> = symbols
            .iter()
            .map(|s| JsonSymbol {
                symbol: s.symbol.clone(),
                category: s.category().to_string(),
                contract_type: s.contract_type().to_string(),
                base_coin: s.base_coin().to_string(),
                quote_coin: s.quote_coin().to_string(),
            })
            .collect();

        Self {
            exchange: exchange.to_string(),
            categories: categories.iter().map(ToString::to_string).collect(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            statistics: stats.clone(),
            symbols: json_symbols,
        }
    }

    /// Serialize to JSON string
    ///
    /// Returns a pretty-printed JSON string representation of the output.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

/// Print JSON output to stdout
pub fn print_json(
    exchange: &str,
    categories: &[&str],
    symbols: &[Symbol],
    stats: &Statistics,
) -> Result<()> {
    let output = JsonOutput::new(exchange, categories, symbols, stats);
    println!("{}", output.to_json()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::test_utils::create_test_symbol;

    #[test]
    fn test_json_output_creation() {
        let symbols = vec![create_test_symbol("BTCUSDT", "linear")];
        let stats = Statistics::from_symbols(&symbols);

        let output = JsonOutput::new("bybit", &["linear"], &symbols, &stats);

        assert_eq!(output.exchange, "bybit");
        assert_eq!(output.statistics.total_count, 1);
        assert_eq!(output.symbols.len(), 1);
    }

    #[test]
    fn test_json_serialization() {
        let symbols = vec![create_test_symbol("BTCUSDT", "linear")];
        let stats = Statistics::from_symbols(&symbols);

        let output = JsonOutput::new("bybit", &["linear"], &symbols, &stats);
        let json = output.to_json();

        assert!(json.is_ok());
        assert!(json.unwrap().contains("BTCUSDT"));
    }
}
