//! Shared HTTP client utilities
//!
//! Provides common HTTP client configuration used across the codebase.

use reqwest::Client;
use std::time::Duration;

use crate::core::error::{CryptoScopeError, Result};

/// Default request timeout (30 seconds)
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Default connection timeout (5 seconds)
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

/// Build a configured reqwest::Client with standard timeouts.
pub fn build_http_client() -> Result<Client> {
    reqwest::Client::builder()
        .timeout(DEFAULT_TIMEOUT)
        .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
        .build()
        .map_err(CryptoScopeError::HttpError)
}
