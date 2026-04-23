use crate::error::Result;
use crate::models::{Statistics, Symbol};
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
    /// Unix timestamp of when the report was generated
    pub timestamp: String,
    /// Aggregated statistics about the symbols
    pub statistics: JsonStatistics,
    /// List of all symbols with their details
    pub symbols: Vec<JsonSymbol>,
}

/// Statistics in JSON-serializable format
///
/// Contains counts organized by category for machine-readable output.
#[derive(Serialize)]
pub struct JsonStatistics {
    /// Total number of symbols
    pub total_count: usize,
    /// Count of symbols by category
    pub by_category: Vec<CategoryCount>,
}

/// Category count pair for JSON output
#[derive(Serialize)]
pub struct CategoryCount {
    /// The category name (e.g., "linear", "inverse")
    pub category: String,
    /// Number of symbols in this category
    pub count: usize,
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

        let by_category: Vec<CategoryCount> = stats
            .by_category
            .iter()
            .map(|(category, count)| CategoryCount {
                category: category.clone(),
                count: *count,
            })
            .collect();

        Self {
            exchange: exchange.to_string(),
            categories: categories.iter().map(ToString::to_string).collect(),
            timestamp: chrono_timestamp(),
            statistics: JsonStatistics {
                total_count: stats.total_count,
                by_category,
            },
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

fn chrono_timestamp() -> String {
    use std::time::SystemTime;

    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("SystemTime before UNIX EPOCH");

    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();

    // Simple ISO-like format without external dependency
    format!("{}.{}", secs, nanos)
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

    fn create_test_symbol() -> Symbol {
        Symbol {
            symbol: "BTCUSDT".to_string(),
            category: Some("linear".to_string()),
            contract_type: Some("Linear".to_string()),
            base_coin: Some("BTC".to_string()),
            quote_coin: Some("USDT".to_string()),
            launch_time: None,
            delivery_time: None,
            delivery_fee_rate: None,
        }
    }

    #[test]
    fn test_json_output_creation() {
        let symbols = vec![create_test_symbol()];
        let stats = Statistics::from_symbols(&symbols);

        let output = JsonOutput::new("bybit", &["linear"], &symbols, &stats);

        assert_eq!(output.exchange, "bybit");
        assert_eq!(output.statistics.total_count, 1);
        assert_eq!(output.symbols.len(), 1);
    }

    #[test]
    fn test_json_serialization() {
        let symbols = vec![create_test_symbol()];
        let stats = Statistics::from_symbols(&symbols);

        let output = JsonOutput::new("bybit", &["linear"], &symbols, &stats);
        let json = output.to_json();

        assert!(json.is_ok());
        assert!(json.unwrap().contains("BTCUSDT"));
    }
}
