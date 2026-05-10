//! Price screener endpoints.

use axum::{Json, Router, routing::get};
use std::sync::Arc;
use tracing::{error, info};

use super::AppState;
use super::error::{AppError, HandlerResult, ValidatedQuery};
use super::types::{ScreenerModeQuery, ScreenerQuery, ScreenerResponse};
use super::utils::{init_database, resolve_exchange};
use crate::core::db::Database;
use crate::core::exchange::Exchange;
use crate::core::models::Statistics;
use crate::core::screener::{Screener, ScreenerMode};

/// Run the price screener
///
/// Analyzes price changes between daily open and current prices, returning
/// top movers filtered by minimum change percentage.
#[utoipa::path(
    get,
    path = "/api/v1/screener",
    tag = "Screener",
    params(
        ("exchange" = String, Query, description = "Exchange name"),
        ("category" = Option<String>, Query, description = "Category filter"),
        ("mode" = Option<ScreenerModeQuery>, Query, description = "Screener mode: kline or mark"),
        ("top" = usize, Query, description = "Number of top results"),
        ("min_change" = Option<f64>, Query, description = "Minimum change percent"),
    ),
    responses(
        (status = 200, description = "Screener results", body = ScreenerResponse),
        (status = 400, description = "Invalid parameters", body = crate::api::types::ValidationErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::types::ErrorResponse),
    ),
)]
#[axum::debug_handler]
pub async fn run_screener(
    ValidatedQuery(query): ValidatedQuery<ScreenerQuery>,
) -> HandlerResult<ScreenerResponse> {
    info!(
        "Running screener: exchange={}, category={:?}, mode={:?}, top={:?}, min_change={:?}",
        query.exchange, query.category, query.mode, query.top, query.min_change
    );

    // Create exchange client
    let exchange = resolve_exchange(&query.exchange)?;

    // Determine categories to scan
    let categories = resolve_categories(query.category.as_deref());

    // Convert mode - Mark mode returns 501 Not Implemented
    let mode = resolve_screener_mode(query.mode)?;

    // Create database connection
    let db = init_database()?;

    // Execute screener (create, run, filter, compute stats)
    let (results, statistics) =
        execute_screener(exchange, db, mode, categories, query.top, query.min_change).await?;

    info!(
        "Screener complete: {} results (filtered from {})",
        results.len(),
        statistics.total_count
    );

    Ok(Json(ScreenerResponse {
        results,
        statistics,
    }))
}

/// Convert screener results to Statistics by building temporary Symbol structs
fn results_to_statistics(results: &[crate::core::models::PriceChange]) -> Statistics {
    Statistics::from_price_changes(results.iter())
}

/// Convert ScreenerModeQuery to domain ScreenerMode
fn resolve_screener_mode(mode: ScreenerModeQuery) -> Result<ScreenerMode, AppError> {
    match mode {
        ScreenerModeQuery::Kline => Ok(ScreenerMode::Kline),
        ScreenerModeQuery::Mark => Err(AppError::not_implemented(
            "Mark price mode is not yet implemented. Use mode=kline instead.".to_string(),
        )),
    }
}

/// Resolve categories to scan from query parameter or defaults
fn resolve_categories(category: Option<&str>) -> Vec<String> {
    if let Some(cat) = category {
        vec![cat.to_string()]
    } else {
        // Default to linear + inverse perpetual contracts
        vec!["linear".to_string(), "inverse".to_string()]
    }
}

/// Execute screener: create, run, filter, and compute statistics
async fn execute_screener(
    exchange: Arc<dyn Exchange>,
    db: Database,
    mode: ScreenerMode,
    categories: Vec<String>,
    top: Option<usize>,
    min_change: Option<f64>,
) -> Result<(Vec<crate::core::models::PriceChange>, Statistics), AppError> {
    // Create and run screener
    let mut screener = Screener::new(db, exchange, mode, categories);
    let mut results = screener.run().await.map_err(|e| {
        error!("Screener execution failed: {}", e);
        AppError::internal_error(format!("Screener failed: {e}"))
    })?;

    // Apply filters
    results = crate::core::screener::output::apply_filters(&results, top, min_change, None, None);

    // Compute statistics
    let stats = results_to_statistics(&results);

    Ok((results, stats))
}

/// Build the screener router
pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/screener", get(run_screener))
}
