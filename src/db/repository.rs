use rusqlite::{Connection, OptionalExtension};

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct OpenPriceRow {
    pub symbol: String,
    pub open_price: f64,
    pub fetch_date: String,
    pub fetch_timestamp: i64,
    pub source: String,
}

/// Database repository for price data operations
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create a new Database instance with an existing connection
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    /// Get the stored fetch date from the database
    ///
    /// Returns the most recent fetch_date if any data exists.
    pub fn get_stored_date(&self) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row(
                "SELECT fetch_date FROM daily_open_prices ORDER BY fetch_timestamp DESC LIMIT 1",
                [],
                |row| row.get(0),
            )
            .optional()?)
    }

    /// Get the cached open price for a specific symbol
    #[allow(dead_code)]
    pub fn get_open_price(&self, symbol: &str) -> Result<Option<f64>> {
        Ok(self
            .conn
            .query_row(
                "SELECT open_price FROM daily_open_prices WHERE symbol = ?",
                [symbol],
                |row| row.get(0),
            )
            .optional()?)
    }

    /// Get all cached open prices as (symbol, price) pairs
    pub fn get_all_open_prices(&self) -> Result<Vec<(String, f64)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT symbol, open_price FROM daily_open_prices")?;

        let price_iter = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })?;

        let mut prices = Vec::new();
        for price_result in price_iter {
            prices.push(price_result?);
        }

        Ok(prices)
    }

    /// Save open prices using UPSERT (ON CONFLICT DO UPDATE) strategy.
    ///
    /// Uses INSERT ... ON CONFLICT(symbol) DO UPDATE to atomically update existing
    /// rows or insert new ones. If a row with the same `symbol` already exists,
    /// all columns are overwritten with the new values.
    /// The transaction ensures atomicity: either all rows are upserted or none.
    /// On any error, the transaction is dropped without calling `commit()`,
    /// which triggers an automatic rollback (rusqlite behavior).
    pub fn save_open_prices(&mut self, prices: Vec<OpenPriceRow>) -> Result<()> {
        let tx = self.conn.transaction()?;

        for price in prices {
            tx.execute(
                "INSERT INTO daily_open_prices
                    (symbol, open_price, fetch_date, fetch_timestamp, source)
                    VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(symbol) DO UPDATE SET
                    open_price = excluded.open_price,
                    fetch_date = excluded.fetch_date,
                    fetch_timestamp = excluded.fetch_timestamp,
                    source = excluded.source",
                rusqlite::params![
                    price.symbol,
                    price.open_price,
                    price.fetch_date,
                    price.fetch_timestamp,
                    price.source,
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    /// Clear all cached price data from the database.
    ///
    /// Note: This only removes data from `daily_open_prices`. The `schema_version`
    /// table is preserved to maintain migration tracking.
    pub fn clear_price_data(&mut self) -> Result<()> {
        self.conn.execute("DELETE FROM daily_open_prices", [])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_test_db, create_test_open_price_row};

    #[test]
    fn test_save_and_get_open_price() {
        let mut db = create_test_db();

        let row = create_test_open_price_row("BTCUSDT");
        db.save_open_prices(vec![row]).unwrap();

        let price = db.get_open_price("BTCUSDT").unwrap();
        assert_eq!(price, Some(50000.0));
    }

    #[test]
    fn test_save_replaces_existing_price() {
        let mut db = create_test_db();

        // Insert initial price
        let row1 = OpenPriceRow {
            symbol: "ETHUSDT".to_string(),
            open_price: 3000.0,
            fetch_date: "2026-04-24".to_string(),
            fetch_timestamp: 1745452800,
            source: "bybit".to_string(),
        };
        db.save_open_prices(vec![row1]).unwrap();

        // Replace with new price
        let row2 = OpenPriceRow {
            symbol: "ETHUSDT".to_string(),
            open_price: 3100.0,
            fetch_date: "2026-04-25".to_string(),
            fetch_timestamp: 1745539200,
            source: "bybit".to_string(),
        };
        db.save_open_prices(vec![row2]).unwrap();

        let price = db.get_open_price("ETHUSDT").unwrap();
        assert_eq!(price, Some(3100.0));
    }

    #[test]
    fn test_get_all_open_prices() {
        let mut db = create_test_db();

        let rows = vec![
            create_test_open_price_row("BTCUSDT"),
            OpenPriceRow {
                symbol: "ETHUSDT".to_string(),
                open_price: 3000.0,
                fetch_date: "2026-04-25".to_string(),
                fetch_timestamp: 1745539200,
                source: "bybit".to_string(),
            },
        ];
        db.save_open_prices(rows).unwrap();

        let prices = db.get_all_open_prices().unwrap();
        assert_eq!(prices.len(), 2);
    }

    #[test]
    fn test_get_stored_date() {
        let mut db = create_test_db();

        let row = create_test_open_price_row("BTCUSDT");
        db.save_open_prices(vec![row]).unwrap();

        let date = db.get_stored_date().unwrap();
        assert_eq!(date, Some("2026-04-25".to_string()));
    }

    #[test]
    fn test_get_stored_date_empty() {
        let db = create_test_db();
        let date = db.get_stored_date().unwrap();
        assert!(date.is_none());
    }

    #[test]
    fn test_clear_price_data() {
        let mut db = create_test_db();

        let row = create_test_open_price_row("BTCUSDT");
        db.save_open_prices(vec![row]).unwrap();

        db.clear_price_data().unwrap();

        let prices = db.get_all_open_prices().unwrap();
        assert!(prices.is_empty());
    }

    #[test]
    fn test_get_open_price_not_found() {
        let db = create_test_db();
        let price = db.get_open_price("NONEXISTENT").unwrap();
        assert!(price.is_none());
    }
}
