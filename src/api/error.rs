//! Error types for the API layer.

use axum::{
    extract::rejection::QueryRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use validator::Validate;

use crate::api::types::{ErrorResponse, ValidationErrorResponse};
use crate::core::CryptoScopeError;

/// Application error type for unified error handling
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum AppError {
    /// Query parameter parsing failed
    QueryParseError(QueryRejection),
    /// Query validation failed
    ValidationError(validator::ValidationErrors),
    /// Generic error with status code
    HttpError(StatusCode, String),
}

impl From<QueryRejection> for AppError {
    fn from(err: QueryRejection) -> Self {
        Self::QueryParseError(err)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        Self::ValidationError(err)
    }
}

/// Convert CryptoScopeError to AppError
impl From<CryptoScopeError> for AppError {
    fn from(err: CryptoScopeError) -> Self {
        match err {
            CryptoScopeError::UnknownExchange(name) => {
                AppError::bad_request(format!("Unknown exchange: {}", name))
            }
            CryptoScopeError::HttpError(e) => {
                AppError::bad_gateway(format!("HTTP request failed: {}", e))
            }
            CryptoScopeError::ParseError(e) => {
                AppError::internal_error(format!("JSON parsing failed: {}", e))
            }
            CryptoScopeError::DbError(e) => {
                AppError::internal_error(format!("Database error: {}", e))
            }
            CryptoScopeError::DbInternal(msg) => AppError::internal_error(msg),
            CryptoScopeError::ApiError { code, message } => {
                AppError::bad_gateway(format!("API error code {}: {}", code, message))
            }
        }
    }
}

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::HttpError(StatusCode::BAD_REQUEST, message.into())
    }

    pub fn bad_gateway(message: impl Into<String>) -> Self {
        Self::HttpError(StatusCode::BAD_GATEWAY, message.into())
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::HttpError(StatusCode::INTERNAL_SERVER_ERROR, message.into())
    }

    pub fn not_implemented(message: impl Into<String>) -> Self {
        Self::HttpError(StatusCode::NOT_IMPLEMENTED, message.into())
    }

    #[allow(dead_code)]
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::HttpError(StatusCode::UNAUTHORIZED, message.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::QueryParseError(e) => {
                (StatusCode::BAD_REQUEST, Json(ErrorResponse::new(e.to_string()))).into_response()
            }
            AppError::ValidationError(e) => {
                let validation_error: ValidationErrorResponse = e.into();
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse::with_details(
                        validation_error.error,
                        format!("{:?}", validation_error.validations),
                    )),
                )
                    .into_response()
            }
            AppError::HttpError(status, msg) => {
                (status, Json(ErrorResponse::new(msg))).into_response()
            }
        }
    }
}

/// Validated query extractor — replaces manual validate() calls in handlers.
///
/// This extractor automatically validates query parameters using the `validator` crate.
/// If validation fails, it returns a 400 Bad Request with detailed error information.
///
/// # Example
/// ```rust,no_run
/// use axum::Json;
/// use cryptoscope::api::extractors::{ValidatedQuery, AppError, HandlerResult};
/// use cryptoscope::api::types::{SymbolQuery, SymbolResponse};
///
/// async fn get_symbols(
///     ValidatedQuery(query): ValidatedQuery<SymbolQuery>,
/// ) -> HandlerResult<SymbolResponse> {
///     // query is already validated - no need for manual validation
///     Ok(Json(SymbolResponse { symbols: vec![] }))
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

impl<T, S> axum::extract::FromRequestParts<S> for ValidatedQuery<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let axum::extract::Query(value) =
            axum::extract::Query::<T>::from_request_parts(parts, state).await?;
        value.validate()?;
        Ok(ValidatedQuery(value))
    }
}

/// Convenience type alias for handler results
pub type HandlerResult<T> = Result<axum::Json<T>, AppError>;
