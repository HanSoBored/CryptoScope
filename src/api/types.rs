//! API request/response types with ts-rs annotations for TypeScript generation.
//!
//! Includes input validation using the validator crate.

use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use ts_rs::TS;
use utoipa::ToSchema;
use validator::Validate;

// Compile-time regex for exchange names (alphanumeric, underscore, hyphen)
static EXCHANGE_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap());

/// Validates exchange name: 1-50 characters, alphanumeric with underscores/hyphens only
fn validate_exchange(exchange: &str) -> Result<(), validator::ValidationError> {
    if exchange.is_empty() || exchange.len() > 50 {
        return Err(validator::ValidationError::new("exchange")
            .with_message("Exchange name must be 1-50 characters".into()));
    }
    if !EXCHANGE_REGEX.is_match(exchange) {
        return Err(validator::ValidationError::new("exchange")
            .with_message("Exchange name contains invalid characters".into()));
    }
    Ok(())
}

// ============================================================================
// Exchange List
// ============================================================================

/// Response for GET /api/v1/exchanges
#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct ExchangeListResponse {
    pub exchanges: Vec<String>,
}

// ============================================================================
// Symbols Endpoint
// ============================================================================

/// Query parameters for GET /api/v1/symbols
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SymbolQuery {
    /// Exchange name (e.g., "bybit")
    #[validate(custom(function = "validate_exchange"))]
    pub exchange: String,
    /// Category filter (e.g., "linear", "inverse", "spot")
    #[validate(length(max = 50, message = "Category must be at most 50 characters"))]
    pub category: Option<String>,
    /// Search term to filter symbols by name
    #[validate(length(max = 100, message = "Search term must be at most 100 characters"))]
    pub search: Option<String>,
}

/// Response for GET /api/v1/symbols
#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct SymbolResponse {
    pub symbols: Vec<crate::core::models::Symbol>,
}

// ============================================================================
// Screener Endpoint
// ============================================================================

/// Screener mode: kline (true daily open) or mark (mark price)
#[derive(Debug, Clone, Copy, Deserialize, ToSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum ScreenerModeQuery {
    /// Kline mode uses true 00:00 UTC daily open from K-line endpoint
    #[default]
    Kline,
    /// Mark mode uses mark price (not yet implemented)
    Mark,
}

/// Query parameters for GET /api/v1/screener
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ScreenerQuery {
    /// Exchange name (e.g., "bybit")
    #[validate(custom(function = "validate_exchange"))]
    pub exchange: String,
    /// Category filter (e.g., "linear", "inverse")
    #[validate(length(max = 50, message = "Category must be at most 50 characters"))]
    pub category: Option<String>,
    /// Screener mode: kline or mark
    #[serde(default)]
    pub mode: ScreenerModeQuery,
    /// Number of top results to return (capped at 100)
    #[validate(range(max = 100, message = "Top value cannot exceed 100"))]
    #[serde(default = "default_top")]
    pub top: usize,
    /// Minimum change percent filter (e.g., 5.0 for 5%)
    #[validate(range(min = 0.0, max = 100.0, message = "Min change must be between 0 and 100"))]
    #[serde(default)]
    pub min_change: Option<f64>,
}

fn default_top() -> usize {
    20
}

/// Response for GET /api/v1/screener
#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct ScreenerResponse {
    pub results: Vec<crate::core::models::PriceChange>,
    pub statistics: crate::core::models::Statistics,
}

// ============================================================================
// Stats Endpoint
// ============================================================================

/// Query parameters for GET /api/v1/stats
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct StatsQuery {
    /// Exchange name (e.g., "bybit")
    #[validate(custom(function = "validate_exchange"))]
    pub exchange: String,
    /// Category filter (e.g., "linear", "inverse", "spot")
    #[validate(length(max = 50, message = "Category must be at most 50 characters"))]
    pub category: Option<String>,
}

/// Response for GET /api/v1/stats
#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct StatsResponse {
    pub statistics: crate::core::models::Statistics,
}

// ============================================================================
// Refresh Endpoint
// ============================================================================

/// Response for POST /api/v1/refresh
#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct RefreshResponse {
    pub status: String,
    pub count: usize,
}

// ============================================================================
// Error Response
// ============================================================================

/// Standard error response format
#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: None,
        }
    }

    pub fn with_details(error: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: Some(details.into()),
        }
    }
}

/// Validation error response for input validation failures
#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct ValidationErrorResponse {
    pub error: String,
    pub validations: std::collections::HashMap<String, Vec<String>>,
}

/// Extract error messages from any ValidationErrorsKind variant recursively.
fn extract_error_messages(error_kind: &validator::ValidationErrorsKind) -> Vec<String> {
    match error_kind {
        validator::ValidationErrorsKind::Field(field_errors) => field_errors
            .iter()
            .filter_map(|e| e.message.clone().map(|cow| cow.into_owned()))
            .collect(),
        validator::ValidationErrorsKind::List(list_errors) => list_errors
            .values()
            .flat_map(|boxed_errors| {
                boxed_errors
                    .errors()
                    .values()
                    .flat_map(extract_error_messages)
                    .collect::<Vec<_>>()
            })
            .collect(),
        validator::ValidationErrorsKind::Struct(struct_errors) => struct_errors
            .errors()
            .values()
            .flat_map(extract_error_messages)
            .collect(),
    }
}

impl From<validator::ValidationErrors> for ValidationErrorResponse {
    fn from(errors: validator::ValidationErrors) -> Self {
        let validations = errors
            .errors()
            .iter()
            .filter_map(|(field, error_kind)| {
                // error_kind is &ValidationErrorsKind (validator 0.20 API)
                let messages: Vec<String> = extract_error_messages(error_kind);
                if messages.is_empty() {
                    None
                } else {
                    Some((field.to_string(), messages))
                }
            })
            .collect();

        Self {
            error: "Validation failed".to_string(),
            validations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_symbol_query_validation_valid() {
        let query = SymbolQuery {
            exchange: "bybit".to_string(),
            category: Some("linear".to_string()),
            search: Some("BTC".to_string()),
        };
        assert!(query.validate().is_ok());
    }

    #[test]
    fn test_symbol_query_validation_invalid_exchange() {
        let query = SymbolQuery {
            exchange: "".to_string(),
            category: None,
            search: None,
        };
        assert!(query.validate().is_err());

        let query = SymbolQuery {
            exchange: "invalid@exchange!".to_string(),
            category: None,
            search: None,
        };
        assert!(query.validate().is_err());
    }

    #[test]
    fn test_symbol_query_validation_max_length() {
        let query = SymbolQuery {
            exchange: "bybit".to_string(),
            category: Some("a".repeat(51)),
            search: None,
        };
        assert!(query.validate().is_err());

        let query = SymbolQuery {
            exchange: "bybit".to_string(),
            category: None,
            search: Some("a".repeat(101)),
        };
        assert!(query.validate().is_err());
    }

    #[test]
    fn test_screener_query_validation_top_limit() {
        let query = ScreenerQuery {
            exchange: "bybit".to_string(),
            category: None,
            mode: ScreenerModeQuery::Kline,
            top: 101,
            min_change: None,
        };
        assert!(query.validate().is_err());

        let query = ScreenerQuery {
            exchange: "bybit".to_string(),
            category: None,
            mode: ScreenerModeQuery::Kline,
            top: 50,
            min_change: None,
        };
        assert!(query.validate().is_ok());
    }
}
