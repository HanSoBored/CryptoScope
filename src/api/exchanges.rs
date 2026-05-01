//! Exchange management endpoints.

use axum::{routing::get, Json, Router};
use super::AppState;
use super::types::ExchangeListResponse;

/// List all supported exchanges
///
/// Returns a list of exchange names that can be used in other endpoints.
#[utoipa::path(
    get,
    path = "/api/v1/exchanges",
    tag = "Exchanges",
    responses(
        (status = 200, description = "List of supported exchanges", body = ExchangeListResponse),
    ),
)]
#[axum::debug_handler]
pub async fn get_exchanges() -> Json<ExchangeListResponse> {
    let exchanges = crate::core::exchange::get_supported_exchanges()
        .iter()
        .map(|s| s.to_string())
        .collect();

    Json(ExchangeListResponse { exchanges })
}

/// Build the exchanges router
pub fn router() -> Router<AppState> {
    Router::new().route("/api/v1/exchanges", get(get_exchanges))
}
