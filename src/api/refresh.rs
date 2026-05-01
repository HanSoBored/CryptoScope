//! Cache refresh endpoints.

use axum::{extract::State, routing::post, Json, Router};
use tracing::{error, info};

use super::AppState;
use super::error::{AppError, HandlerResult};
use super::types::RefreshResponse;
use super::auth::Claims;
use super::utils::init_database;

/// Force refresh of cached data
///
/// Clears the daily open price cache, forcing a fresh fetch on the next
/// screener run. Requires authentication.
#[utoipa::path(
    post,
    path = "/api/v1/refresh",
    tag = "Cache",
    security(
        ("bearer_auth" = []),
    ),
    responses(
        (status = 200, description = "Cache refreshed", body = RefreshResponse),
        (status = 401, description = "Unauthorized", body = crate::api::types::ErrorResponse),
        (status = 500, description = "Internal server error", body = crate::api::types::ErrorResponse),
    ),
)]
#[axum::debug_handler]
pub async fn refresh_cache(
    _claims: Claims,
    State(_state): State<AppState>,
) -> HandlerResult<RefreshResponse> {
    info!("Refreshing cache (authenticated user: {})", _claims.sub);

    // Create database connection
    let mut db = init_database()?;

    // Get count before clearing
    let prices = db.get_all_open_prices().unwrap_or_default();
    let count = prices.len();

    // Clear the cache
    db.clear_price_data().map_err(|e| {
        error!("Failed to clear price data: {}", e);
        AppError::internal_error(format!("Failed to clear cache: {}", e))
    })?;

    info!("Cache refreshed: cleared {} entries", count);

    Ok(Json(RefreshResponse {
        status: "refreshed".to_string(),
        count,
    }))
}

/// Build the refresh router
pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/refresh", post(refresh_cache))
}
