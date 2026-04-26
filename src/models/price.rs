use crate::models::response::TickerRawResponse;
use crate::utils::parse_f64_or_zero;

/// Ticker from /v5/market/tickers endpoint
#[derive(Debug, Clone)]
pub struct Ticker {
    pub symbol: String,
    pub category: String,
    pub last_price: f64,
    pub prev_price_24h: f64,
    pub volume_24h: f64,
}

impl Ticker {
    /// Create a ticker from raw response with explicit category
    pub fn from_raw(raw: &TickerRawResponse, category: &str) -> Self {
        Self {
            symbol: raw.symbol.clone(),
            category: category.to_string(),
            last_price: parse_f64_or_zero("last_price", &raw.last_price, &raw.symbol),
            prev_price_24h: parse_f64_or_zero("prev_price_24h", &raw.prev_price_24h, &raw.symbol),
            volume_24h: parse_f64_or_zero("volume_24h", &raw.volume_24h, &raw.symbol),
        }
    }
}

/// Daily K-line from /v5/market/kline?interval=D
#[derive(Debug, Clone)]
pub struct DailyKline {
    pub open_price: f64,
}

/// Price change calculation result
#[derive(Debug, Clone)]
pub struct PriceChange {
    pub symbol: String,
    pub category: String,
    pub open_price: f64,
    pub current_price: f64,
    pub change_value: f64,
    pub change_percent: f64,
    pub volume_24h: f64,
}

impl PriceChange {
    pub fn change_percent_formatted(&self) -> String {
        if self.change_percent >= 0.0 {
            format!("+{:.2}%", self.change_percent)
        } else {
            format!("{:.2}%", self.change_percent)
        }
    }

    pub fn change_value_formatted(&self) -> String {
        if self.change_value >= 0.0 {
            format!("+{:.2}", self.change_value)
        } else {
            format!("{:.2}", self.change_value)
        }
    }

    /// Returns true if this is a perpetual contract (linear or inverse).
    pub fn is_perpetual(&self) -> bool {
        matches!(self.category.as_str(), "linear" | "inverse")
    }

    /// Compute volume in USDT terms.
    /// For perpetuals (linear/inverse), volume is in contracts so multiply by price.
    /// For spot, volume is already in quote currency (USDT).
    pub fn volume_usdt(&self) -> f64 {
        if self.is_perpetual() {
            self.current_price * self.volume_24h
        } else {
            self.volume_24h
        }
    }
}
