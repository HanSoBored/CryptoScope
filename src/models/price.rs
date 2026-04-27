use crate::models::response::TickerRawResponse;
use crate::models::ContractType;
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
    /// Create a ticker from raw response with explicit category.
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
    pub contract_type: ContractType,
    pub open_price: f64,
    pub current_price: f64,
    pub change_value: f64,
    pub change_percent: f64,
    pub volume_24h: f64,
}

/// Format a numeric value with a `+` prefix for non-negative values.
fn format_with_sign(value: f64, decimals: usize) -> String {
    if value >= 0.0 {
        format!("+{value:.decimals$}")
    } else {
        format!("{value:.decimals$}")
    }
}

impl PriceChange {
    pub fn change_percent_formatted(&self) -> String {
        format!("{}%", format_with_sign(self.change_percent, 2))
    }

    pub fn change_value_formatted(&self) -> String {
        format_with_sign(self.change_value, 2)
    }

    /// Returns true if this is a derivative contract (perpetual or futures).
    /// Spot and other non-derivative contract types return false.
    pub fn is_derivative(&self) -> bool {
        matches!(
            self.contract_type,
            ContractType::LinearPerpetual
                | ContractType::LinearFutures
                | ContractType::InversePerpetual
                | ContractType::InverseFutures
        )
    }

    /// Compute volume in USDT terms.
    /// For derivatives (linear/inverse), volume is in contracts so multiply by price.
    /// For spot, volume is already in quote currency (USDT).
    pub fn volume_usdt(&self) -> f64 {
        if self.is_derivative() {
            self.current_price * self.volume_24h
        } else {
            self.volume_24h
        }
    }
}
