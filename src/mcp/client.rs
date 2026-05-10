//! CryptoScope API Client for MCP Server
//!
//! HTTP client that communicates with the CryptoScope API server.

use anyhow::{Context, Result};
use serde_json::Value;
use std::env;

use crate::core::http::build_http_client;

/// HTTP client for CryptoScope API
#[derive(Clone)]
pub struct CryptoScopeClient {
    base_url: String,
    token: Option<String>,
    http_client: reqwest::Client,
}

impl CryptoScopeClient {
    /// Create a new client with default configuration
    pub fn new() -> Result<Self, String> {
        let base_url =
            env::var("CRYPTOSCOPE_API_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
        Ok(Self {
            base_url,
            token: None,
            http_client: build_http_client().map_err(|e| e.to_string())?,
        })
    }

    /// Set the authentication token for protected endpoints
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    /// Check if the client is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }

    /// Shared request executor — eliminates get/post duplication
    async fn execute_request(
        &self,
        builder: reqwest::RequestBuilder,
        method: &str,
        endpoint: &str,
    ) -> Result<Value> {
        let response = builder
            .send()
            .await
            .context(format!("{method} /api/v1/{endpoint} failed"))?;

        let response = response
            .error_for_status()
            .context(format!("{method} /api/v1/{endpoint} returned error status"))?;

        let json: Value = response
            .json()
            .await
            .context("Failed to parse JSON response")?;
        Ok(json)
    }

    /// Perform a GET request to the API
    pub async fn get(&self, endpoint: &str) -> Result<Value> {
        let url = format!("{}/api/v1/{}", self.base_url, endpoint);
        let mut builder = self.http_client.get(&url);
        if let Some(ref token) = self.token {
            builder = builder.bearer_auth(token);
        }
        self.execute_request(builder, "GET", endpoint).await
    }

    /// Perform a POST request to the API
    pub async fn post(&self, endpoint: &str, body: &Value) -> Result<Value> {
        let url = format!("{}/api/v1/{}", self.base_url, endpoint);
        let mut builder = self.http_client.post(&url).json(body);
        if let Some(ref token) = self.token {
            builder = builder.bearer_auth(token);
        }
        self.execute_request(builder, "POST", endpoint).await
    }
}

impl Default for CryptoScopeClient {
    fn default() -> Self {
        // NOTE: This fallback uses standard timeouts via build_http_client().
        // Prefer CryptoScopeClient::new() for production use (handles errors explicitly).
        Self {
            base_url: "http://localhost:3000".to_string(),
            token: None,
            http_client: build_http_client()
                .map_err(|_| ())
                .unwrap_or_else(|_| reqwest::Client::default()),
        }
    }
}
