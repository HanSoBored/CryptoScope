use serde::{Deserialize, Serialize};

/// Represents a cryptocurrency trading instrument/symbol.
///
/// Contains metadata about a trading pair including contract type,
/// and other trading parameters from the Bybit exchange.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Symbol {
    /// The symbol/ticker name (e.g., "BTCUSDT")
    pub symbol: String,
    /// Category of the instrument (e.g., "linear", "inverse")
    #[serde(rename = "category", default)]
    pub category: Option<String>,
    /// Type of contract (e.g., "Linear", "InversePerpetual")
    #[serde(rename = "contractType", default)]
    pub contract_type: Option<String>,
    /// Base currency code (e.g., "BTC")
    #[serde(rename = "baseCoin", default)]
    pub base_coin: Option<String>,
    /// Quote currency code (e.g., "USDT")
    #[serde(rename = "quoteCoin", default)]
    pub quote_coin: Option<String>,
    /// Unix timestamp when the symbol was launched
    #[serde(rename = "launchTime", default)]
    pub launch_time: Option<String>,
    /// Unix timestamp for delivery (futures contracts)
    #[serde(rename = "deliveryTime", default)]
    pub delivery_time: Option<String>,
    /// Delivery fee rate for futures contracts
    #[serde(rename = "deliveryFeeRate", default)]
    pub delivery_fee_rate: Option<String>,
}

impl Symbol {
    /// Get category with fallback to "unknown".
    ///
    /// Returns the instrument category (e.g., "linear", "inverse") or "unknown"
    /// if not specified.
    pub fn category(&self) -> &str {
        self.category.as_deref().unwrap_or("unknown")
    }

    /// Get contract type with fallback
    ///
    /// Returns the contract type (e.g., "Linear", "InversePerpetual") or "Unknown"
    /// if not specified.
    pub fn contract_type(&self) -> &str {
        self.contract_type.as_deref().unwrap_or("Unknown")
    }

    /// Get base coin with fallback
    ///
    /// Returns the base currency code (e.g., "BTC") or "UNKNOWN" if not specified.
    pub fn base_coin(&self) -> &str {
        self.base_coin.as_deref().unwrap_or("UNKNOWN")
    }

    /// Get quote coin with fallback
    ///
    /// Returns the quote currency code (e.g., "USDT") or "UNKNOWN" if not specified.
    pub fn quote_coin(&self) -> &str {
        self.quote_coin.as_deref().unwrap_or("UNKNOWN")
    }
}
