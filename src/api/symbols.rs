//! Symbol/instrument query endpoints.

use axum::{routing::get, Json, Router};
use tracing::info;

use super::AppState;
use super::error::{HandlerResult, ValidatedQuery};
use super::types::{SymbolQuery, SymbolResponse};
use super::utils::{resolve_exchange, fetch_symbols};

/// Get symbols from an exchange
///
/// Fetches trading instruments from the specified exchange, optionally filtered
/// by category and search term.
#[utoipa::path(
    get,
    path = "/api/v1/symbols",
    tag = "Symbols",
    params(
        ("exchange" = String, Query, description = "Exchange name (e.g., 'bybit')"),
        ("category" = Option<String>, Query, description = "Category filter (e.g., 'linear', 'inverse')"),
        ("search" = Option<String>, Query, description = "Search term to filter symbols"),
    ),
    responses(
        (status = 200, description = "List of symbols", body = SymbolResponse),
        (status = 400, description = "Invalid parameters", body = crate::api::types::ValidationErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::types::ErrorResponse),
    ),
)]
#[axum::debug_handler]
pub async fn get_symbols(
    ValidatedQuery(query): ValidatedQuery<SymbolQuery>,
) -> HandlerResult<SymbolResponse> {
    info!(
        "Fetching symbols: exchange={}, category={:?}, search={:?}",
        query.exchange, query.category, query.search
    );

    // Create exchange client
    let exchange = resolve_exchange(&query.exchange)?;

    // Fetch instruments
    let mut symbols = fetch_symbols(&exchange, query.category.as_deref()).await?;

    // Apply search filter if provided
    if let Some(search) = &query.search {
        let search_lower = search.to_lowercase();
        symbols.retain(|s| s.symbol.to_lowercase().contains(&search_lower));
    }

    info!("Returning {} symbols", symbols.len());
    Ok(Json(SymbolResponse { symbols }))
}

/// Build the symbols router
pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/symbols", get(get_symbols))
}
