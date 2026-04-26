use super::symbol::Symbol;
use serde::{Deserialize, Serialize};

/// Bybit API response structure for instruments-info endpoint
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BybitApiResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: BybitInstrumentsResult,
    #[serde(rename = "retExtInfo", default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

/// Result section containing the instruments list and pagination info.
///
/// This struct wraps the list of symbols returned by the Bybit API
/// along with category information and pagination cursor for fetching
/// additional pages of results.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BybitInstrumentsResult {
    /// The category of instruments (e.g., "linear", "inverse")
    #[serde(rename = "category")]
    pub category: String,
    /// List of trading symbols/instruments in this category
    pub list: Vec<Symbol>,
    /// Cursor for fetching the next page of results (if available)
    #[serde(rename = "nextPageCursor", default)]
    pub next_page_cursor: Option<String>,
}

impl BybitApiResponse {
    /// Check if API call was successful
    ///
    /// Returns `true` if the response code is 0, indicating success.
    pub fn is_success(&self) -> bool {
        self.ret_code == 0
    }

    /// Get error message if request failed
    ///
    /// Returns `None` if the request was successful, otherwise returns
    /// a formatted error message with the response code.
    #[allow(dead_code)]
    pub fn error_message(&self) -> Option<String> {
        if self.is_success() {
            None
        } else {
            Some(format!("{} (code: {})", self.ret_msg, self.ret_code))
        }
    }
}

/// Ticker endpoint response structure
#[derive(Debug, Clone, Deserialize)]
pub struct TickerApiResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: TickerApiResult,
}

/// Result section of the ticker API response.
///
/// Contains the category and list of raw ticker data returned by
/// the Bybit `/v5/market/tickers` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct TickerApiResult {
    /// The category of instruments (e.g., "linear", "inverse").
    #[allow(dead_code)]
    pub category: String,
    /// List of raw ticker responses with string-encoded numeric fields.
    pub list: Vec<TickerRawResponse>,
}

/// Raw ticker response with string fields (API returns strings).
///
/// Numeric fields are stored as strings by the Bybit API and must be
/// parsed separately.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerRawResponse {
    /// The symbol/ticker name (e.g., "BTCUSDT").
    pub symbol: String,
    /// Last traded price as a string.
    pub last_price: String,
    /// Price 24 hours ago as a string.
    pub prev_price_24h: String,
    /// Price change percentage over 24h as a string.
    #[allow(dead_code)]
    pub price_24h_pcnt: String,
    /// Highest price in the last 24h as a string.
    #[allow(dead_code)]
    pub high_price_24h: String,
    /// Lowest price in the last 24h as a string.
    #[allow(dead_code)]
    pub low_price_24h: String,
    /// Trading volume in the last 24h as a string.
    pub volume_24h: String,
}

/// K-line endpoint response structure.
#[derive(Debug, Clone, Deserialize)]
pub struct KlineApiResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: KlineApiResult,
}

/// Result section of the K-line API response.
///
/// Contains the symbol, category, and list of K-line data arrays returned
/// by the Bybit `/v5/market/kline` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct KlineApiResult {
    /// The symbol identifier.
    #[allow(dead_code)]
    pub symbol: String,
    /// The category of instruments (e.g., "linear", "inverse").
    #[allow(dead_code)]
    pub category: String,
    /// List of K-line data arrays. Each inner array contains:
    /// [timestamp, open, high, low, close, volume, turnover].
    pub list: Vec<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sample_response() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "category": "linear",
                "list": [
                    {
                        "symbol": "BTCUSDT",
                        "status": "Trading",
                        "category": "linear",
                        "contractType": "Linear",
                        "baseCoin": "BTC",
                        "quoteCoin": "USDT"
                    }
                ],
                "nextPageCursor": ""
            },
            "time": 1234567890
        }"#;

        let response: BybitApiResponse = serde_json::from_str(json).unwrap();
        assert!(response.is_success());
        assert_eq!(response.result.list.len(), 1);
        assert_eq!(response.result.list[0].symbol, "BTCUSDT");
    }
}
