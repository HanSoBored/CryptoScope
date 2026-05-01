//! Market statistics endpoints.

use axum::{routing::get, Json, Router};
use tracing::info;

use super::AppState;
use super::error::{HandlerResult, ValidatedQuery};
use super::types::{StatsQuery, StatsResponse};
use super::utils::{resolve_exchange, fetch_symbols};

/// Get market statistics
///
/// Returns aggregated statistics about symbols for the specified exchange
/// and category.
#[utoipa::path(
    get,
    path = "/api/v1/stats",
    tag = "Statistics",
    params(
        ("exchange" = String, Query, description = "Exchange name"),
        ("category" = Option<String>, Query, description = "Category filter"),
    ),
    responses(
        (status = 200, description = "Market statistics", body = StatsResponse),
        (status = 400, description = "Invalid parameters", body = crate::api::types::ValidationErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::types::ErrorResponse),
    ),
)]
#[axum::debug_handler]
pub async fn get_stats(
    ValidatedQuery(query): ValidatedQuery<StatsQuery>,
) -> HandlerResult<StatsResponse> {
    info!(
        "Fetching stats: exchange={}, category={:?}",
        query.exchange, query.category
    );

    // Create exchange client
    let exchange = resolve_exchange(&query.exchange)?;

    // Fetch symbols for statistics
    let symbols = fetch_symbols(&exchange, query.category.as_deref()).await?;

    let statistics = crate::core::models::Statistics::from_symbols(&symbols);

    info!(
        "Stats: {} total symbols across {} categories",
        statistics.total_count,
        statistics.by_category.len()
    );

    Ok(Json(StatsResponse { statistics }))
}

/// Build the stats router
pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/stats", get(get_stats))
}
