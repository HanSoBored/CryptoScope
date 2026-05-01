//! Shared utility functions for API handlers.
//!
//! This module contains common helper functions used across multiple
//! API handlers to reduce code duplication.

use std::sync::Arc;
use tracing::{warn, error};
use super::error::AppError;
use crate::core::db::{create_connection, init_schema, Database};
use crate::core::exchange::Exchange;
use crate::core::models::Symbol;

/// Initialize database connection and schema.
///
/// Creates a new database connection and initializes the schema if needed.
/// Returns an error if connection or schema initialization fails.
pub fn init_database() -> Result<Database, AppError> {
    let conn = create_connection()
        .map_err(|e| AppError::internal_error(format!("Failed to create DB connection: {}", e)))?;
    init_schema(&conn)
        .map_err(|e| AppError::internal_error(format!("Failed to initialize DB schema: {}", e)))?;
    Ok(Database::new(conn))
}

/// Resolve exchange name to exchange client.
///
/// Creates an exchange client for the given exchange name.
/// Returns an error if the exchange is not supported.
pub fn resolve_exchange(name: &str) -> Result<Arc<dyn Exchange>, AppError> {
    crate::core::exchange::create_exchange(name)
        .map(Arc::from)
        .map_err(|e| AppError::bad_request(format!("Unknown exchange: {}", e)))
}

/// Fetch symbols from exchange, either for a specific category or all categories.
///
/// If a category is provided, fetches instruments for that category only.
/// Otherwise, fetches instruments from all categories (linear, inverse, spot).
pub async fn fetch_symbols(
    exchange: &Arc<dyn Exchange>,
    category: Option<&str>,
) -> Result<Vec<Symbol>, AppError> {
    if let Some(category) = category {
        exchange.fetch_instruments(category).await.map_err(|e| {
            error!("Failed to fetch instruments: {}", e);
            AppError::internal_error(format!("Failed to fetch symbols: {}", e))
        })
    } else {
        // Fetch from all standard categories, continuing on errors
        let categories = ["linear", "inverse", "spot"];
        let mut symbols = Vec::new();
        for cat in &categories {
            match exchange.fetch_instruments(cat).await {
                Ok(mut syms) => symbols.append(&mut syms),
                Err(e) => warn!("Failed to fetch {} instruments: {}", cat, e),
            }
        }
        Ok(symbols)
    }
}
