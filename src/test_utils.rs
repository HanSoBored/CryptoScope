use crate::db::{Database, OpenPriceRow, init_schema};
use crate::models::{DailyKline, PriceChange, Symbol, Ticker};
use rusqlite::Connection;

pub fn create_test_db() -> Database {
    let conn = Connection::open_in_memory().expect("Failed to create in-memory connection");
    init_schema(&conn).expect("Failed to initialize test schema");
    Database::new(conn)
}

pub fn create_test_symbol(symbol: &str, category: &str) -> Symbol {
    Symbol {
        symbol: symbol.to_string(),
        category: Some(category.to_string()),
        contract_type: if category == "linear" {
            Some("Linear".to_string())
        } else {
            Some("InversePerpetual".to_string())
        },
        base_coin: Some("BTC".to_string()),
        quote_coin: Some("USDT".to_string()),
        launch_time: None,
        delivery_time: None,
        delivery_fee_rate: None,
    }
}

#[allow(dead_code)]
pub fn create_test_ticker(symbol: &str) -> Ticker {
    Ticker {
        symbol: symbol.to_string(),
        category: "linear".to_string(),
        last_price: 50000.0,
        prev_price_24h: 49000.0,
        volume_24h: 1000.0,
    }
}

#[allow(dead_code)]
pub fn create_test_price_change(symbol: &str) -> PriceChange {
    PriceChange {
        symbol: symbol.to_string(),
        category: "linear".to_string(),
        open_price: 49000.0,
        current_price: 50000.0,
        change_value: 1000.0,
        change_percent: 2.04,
        volume_24h: 1000.0,
    }
}

pub fn create_test_open_price_row(symbol: &str) -> OpenPriceRow {
    OpenPriceRow {
        symbol: symbol.to_string(),
        open_price: 50000.0,
        fetch_date: "2026-04-25".to_string(),
        fetch_timestamp: 1745539200,
        source: "bybit".to_string(),
    }
}

#[allow(dead_code)]
pub fn create_test_kline(_symbol: &str) -> DailyKline {
    DailyKline {
        open_price: 49000.0,
    }
}
