//! JWT Authentication module for CryptoScope API.
//!
//! Provides token generation, validation, and Axum extractors for authenticated requests.

use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::api::AppState;

/// JWT encoding/decoding keys
#[derive(Clone)]
pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/// Admin credentials loaded from environment
#[derive(Clone)]
pub struct AdminCredentials {
    pub username: String,
    pub password_hash: String,
}

/// Load JWT keys from environment with validation
///
/// # Errors
/// Returns an error if:
/// - JWT_SECRET is not set
/// - JWT_SECRET is less than 32 characters
pub fn load_keys() -> Result<Keys, String> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| "JWT_SECRET environment variable must be set. Generate a random 32+ character secret.".to_string())?;
    
    // Minimum 32 characters for security
    if secret.len() < 32 {
        return Err("JWT_SECRET must be at least 32 characters for security".to_string());
    }
    
    Ok(Keys::new(secret.as_bytes()))
}

/// Load admin credentials from environment
///
/// # Errors
/// Returns an error if ADMIN_USER or ADMIN_PASS_HASH is not set
pub fn load_admin_credentials() -> Result<AdminCredentials, String> {
    let username = std::env::var("ADMIN_USER")
        .map_err(|_| "ADMIN_USER environment variable must be set".to_string())?;
    let password_hash = std::env::var("ADMIN_PASS_HASH")
        .map_err(|_| "ADMIN_PASS_HASH environment variable must be set (use argon2id PHC format)".to_string())?;
    
    Ok(AdminCredentials { username, password_hash })
}

/// Build JWT validation configuration with explicit security settings
fn build_validation() -> Validation {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["exp", "iat"]);
    validation.validate_nbf = true;
    validation.leeway = 60;
    validation.reject_tokens_expiring_in_less_than = 30;
    validation
}

/// JWT claims structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// User ID (subject)
    pub sub: String,
    /// Expiration time (UTC timestamp)
    pub exp: i64,
    /// Issued at (UTC timestamp)
    pub iat: i64,
    /// User roles for authorization
    pub roles: Vec<String>,
}

/// Extractor for authenticated requests
impl FromRequestParts<AppState> for Claims {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // Extract Authorization header manually
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthError::MissingCredentials)?;

        // Parse Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidToken)?;

        let token_data = decode::<Claims>(
            token,
            &state.keys.decoding,
            &build_validation(),
        )
        .map_err(|e| {
            tracing::debug!("Token validation failed: {:?}", e);
            AuthError::InvalidToken
        })?;

        Ok(token_data.claims)
    }
}

/// Authentication errors
#[derive(Debug)]
pub enum AuthError {
    MissingCredentials,
    InvalidToken,
    TokenCreation,
    WrongCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        use axum::http::header::WWW_AUTHENTICATE;
        use serde_json::json;

        let (status, error_message, include_www_auth) = match self {
            AuthError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials", false)
            }
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token", true),
            AuthError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create token",
                false,
            ),
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials", true),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        if include_www_auth {
            (
                status,
                [(WWW_AUTHENTICATE, "Bearer".to_string())],
                body,
            )
                .into_response()
        } else {
            (status, body).into_response()
        }
    }
}

/// Generate a JWT token for a user
///
/// # Arguments
/// * `keys` - JWT encoding/decoding keys
/// * `user_id` - Unique user identifier
/// * `roles` - List of roles (e.g., ["user"], ["admin"], ["user", "admin"])
///
/// # Returns
/// * `Ok(String)` - JWT token string
/// * `Err(AuthError)` - Token creation failed
pub fn generate_token(keys: &Keys, user_id: &str, roles: Vec<String>) -> Result<String, AuthError> {
    let now = OffsetDateTime::now_utc();
    let exp = now + Duration::hours(24); // 24-hour tokens

    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp.unix_timestamp(),
        iat: now.unix_timestamp(),
        roles,
    };

    encode(&Header::default(), &claims, &keys.encoding).map_err(|e| {
        tracing::error!("Token encoding failed: {:?}", e);
        AuthError::TokenCreation
    })
}

/// Login request body
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login response
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub expires_in: i64,
}

/// Login endpoint handler
///
/// Validates credentials against ADMIN_USER and ADMIN_PASS_HASH environment variables.
/// Password must be hashed with argon2id (PHC format).
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 400, description = "Invalid credentials", body = crate::api::types::ErrorResponse),
    ),
)]
#[axum::debug_handler]
pub async fn login(
    State(state): State<crate::api::AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AuthError> {
    // Validate input is not empty
    if payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    // Check username matches
    if payload.username != state.admin_credentials.username {
        return Err(AuthError::WrongCredentials);
    }

    // Verify password hash using argon2id
    let parsed_hash = PasswordHash::new(&state.admin_credentials.password_hash)
        .map_err(|_| AuthError::WrongCredentials)?;
    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| AuthError::WrongCredentials)?;

    // Generate JWT token with admin role
    let token = generate_token(&state.keys, &payload.username, vec!["admin".to_string()])?;

    Ok(Json(LoginResponse {
        token,
        expires_in: 86400, // 24 hours in seconds
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_keys() -> Keys {
        Keys::new(b"test_secret_key_that_is_at_least_32_characters_long!")
    }

    #[test]
    fn test_token_generation_and_validation() {
        let keys = test_keys();
        let token = generate_token(&keys, "test_user", vec!["user".to_string()]).unwrap();
        assert!(!token.is_empty());

        // Decode and verify
        let decoded = decode::<Claims>(&token, &keys.decoding, &build_validation()).unwrap();
        assert_eq!(decoded.claims.sub, "test_user");
        assert!(decoded.claims.roles.contains(&"user".to_string()));
    }

}
