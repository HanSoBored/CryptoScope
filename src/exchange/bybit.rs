use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, info, warn};

use super::exchange_trait::Exchange;
use crate::error::{CryptoScopeError, Result};
use crate::models::{
    BybitApiResponse, DailyKline, Symbol, Ticker,
    response::{KlineApiResponse, TickerApiResponse},
};
use crate::utils::parse_f64;

const BYBIT_BASE_URL: &str = "https://api.bybit.com";
const INSTRUMENTS_ENDPOINT: &str = "/v5/market/instruments-info";
const TICKERS_ENDPOINT: &str = "/v5/market/tickers";
const KLINE_ENDPOINT: &str = "/v5/market/kline";

// Bybit kline array column indices
// See: https://bybit-exchange.github.io/docs/v5/market/kline
const KLINE_OPEN_IDX: usize = 1;

fn build_kline_url(base_url: &str, symbol: &str, category: &str) -> String {
    format!(
        "{}{}?category={}&symbol={}&interval=D&limit=1",
        base_url, KLINE_ENDPOINT, category, symbol
    )
}

fn check_api_response(ret_code: i32, ret_msg: &str) -> Result<()> {
    if ret_code != 0 {
        return Err(CryptoScopeError::ApiError {
            code: ret_code,
            message: ret_msg.to_string(),
        });
    }
    Ok(())
}

/// Parse a single kline field by index with descriptive error messages.
fn parse_kline_field(data: &[String], idx: usize, field_name: &str, symbol: &str) -> Result<f64> {
    let value = data.get(idx).ok_or_else(|| CryptoScopeError::ApiError {
        code: -1,
        message: format!(
            "Kline data too short: expected index {idx} \
             but data has {} elements for symbol '{symbol}'",
            data.len()
        ),
    })?;
    parse_f64(field_name, value, symbol).ok_or_else(|| CryptoScopeError::ApiError {
        code: -1,
        message: format!("Failed to parse {field_name} for symbol '{symbol}'"),
    })
}

fn parse_kline_fields(data: &[Vec<String>], symbol: &str) -> Result<DailyKline> {
    let kline_data = data.first().ok_or_else(|| CryptoScopeError::ApiError {
        code: -1,
        message: format!("No k-line data found for symbol '{}'", symbol),
    })?;

    if kline_data.len() < 6 {
        return Err(CryptoScopeError::ApiError {
            code: -1,
            message: format!("Invalid k-line data format for symbol '{}'", symbol),
        });
    }

    let open_price = parse_kline_field(kline_data, KLINE_OPEN_IDX, "open_price", symbol)?;

    debug!("Parsed kline for {}: open={}", symbol, open_price);

    Ok(DailyKline { open_price })
}

/// Bybit exchange client
pub struct BybitClient {
    client: Client,
    base_url: String,
}

impl BybitClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: BYBIT_BASE_URL.to_string(),
        }
    }

    /// Create a new Bybit client with custom base URL (useful for testing)
    #[allow(dead_code)]
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Fetch a single page of instruments
    async fn fetch_page(&self, category: &str, cursor: &str) -> Result<BybitApiResponse> {
        let mut url = format!("{}{}", self.base_url, INSTRUMENTS_ENDPOINT);
        url.push_str(&format!("?category={}", category));

        if !cursor.is_empty() {
            url.push_str(&format!("&cursor={}", cursor));
        }

        debug!("Fetching: {}", url);

        let response = self.client.get(&url).send().await?;
        let api_response: BybitApiResponse = response.json().await?;

        check_api_response(api_response.ret_code, &api_response.ret_msg)?;

        Ok(api_response)
    }

    /// Fetch a single page, assign category, and return symbols with next cursor.
    async fn process_page(
        &self,
        category: &str,
        cursor: &str,
        page_count: &mut u32,
    ) -> Result<(Vec<Symbol>, Option<String>)> {
        *page_count += 1;
        let response = self.fetch_page(category, cursor).await?;

        let mut symbols = response.result.list;
        for symbol in &mut symbols {
            if symbol.category.is_none() {
                symbol.category = Some(category.to_string());
            }
        }
        let count = symbols.len();
        debug!("Page {}: fetched {} symbols", page_count, count);

        Ok((symbols, response.result.next_page_cursor))
    }
}

#[async_trait]
impl Exchange for BybitClient {
    fn name(&self) -> &'static str {
        "bybit"
    }

    async fn fetch_instruments(&self, category: &str) -> Result<Vec<Symbol>> {
        info!("Fetching {} instruments from Bybit...", category);

        let mut all_symbols = Vec::new();
        let mut cursor = String::new();
        let mut page_count = 0;

        loop {
            let (symbols, next_cursor) = self
                .process_page(category, &cursor, &mut page_count)
                .await?;
            all_symbols.extend(symbols);

            match next_cursor {
                Some(c) if !c.is_empty() => cursor = c,
                _ => {
                    info!(
                        "Completed fetching {} symbols from {} pages for category '{}'",
                        all_symbols.len(),
                        page_count,
                        category
                    );
                    break;
                }
            }

            if page_count >= 100 {
                warn!("Reached maximum page limit (100). Stopping pagination.");
                break;
            }
        }

        Ok(all_symbols)
    }

    async fn fetch_tickers(&self, category: &str) -> Result<Vec<Ticker>> {
        info!("Fetching tickers for category '{}' from Bybit...", category);

        let url = format!(
            "{}{}?category={}",
            self.base_url, TICKERS_ENDPOINT, category
        );
        debug!("Fetching: {}", url);

        let response = self.client.get(&url).send().await?;
        let api_response: TickerApiResponse = response.json().await?;

        check_api_response(api_response.ret_code, &api_response.ret_msg)?;

        let tickers: Vec<Ticker> = api_response
            .result
            .list
            .iter()
            .map(|raw| Ticker::from_raw(raw, category))
            .collect();

        info!(
            "Fetched {} tickers for category '{}'",
            tickers.len(),
            category
        );
        Ok(tickers)
    }

    async fn fetch_daily_kline(&self, symbol: &str, category: &str) -> Result<DailyKline> {
        debug!(
            "Fetching daily k-line for symbol: {} (category: {})",
            symbol, category
        );

        let url = build_kline_url(&self.base_url, symbol, category);
        debug!("Kline fetch URL: {}", url);

        let response = self.client.get(&url).send().await?;
        let api_response: KlineApiResponse = response.json().await?;

        check_api_response(api_response.ret_code, &api_response.ret_msg)?;

        debug!("Raw kline data array: {:?}", api_response.result.list);

        parse_kline_fields(&api_response.result.list, symbol)
    }
}

impl Default for BybitClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bybit_client_creation() {
        let client = BybitClient::new();
        assert_eq!(client.name(), "bybit");
    }

    #[test]
    fn test_parse_f64_valid() {
        let result = parse_f64("open_price", "60000.5", "BTCUSDT");
        assert_eq!(result, Some(60000.5));
    }

    #[test]
    fn test_parse_f64_invalid_number() {
        let result = parse_f64("open_price", "invalid", "BTCUSDT");
        assert!(result.is_none());
    }

    #[test]
    fn test_parse_kline_fields_insufficient_data() {
        let data = vec![vec!["1714003200000".to_string()]];
        let result = parse_kline_fields(&data, "BTCUSDT");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_kline_fields_empty_data() {
        let data: Vec<Vec<String>> = vec![];
        let result = parse_kline_fields(&data, "BTCUSDT");
        assert!(result.is_err());
    }
}
