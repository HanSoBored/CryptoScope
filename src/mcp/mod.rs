//! CryptoScope MCP Server Module
//!
//! This module provides the MCP server implementation using the rmcp SDK.
//! It exposes CryptoScope API capabilities to AI agents via the Model Context Protocol.

use crate::core::security::SecurityConfig;
use crate::mcp::client::CryptoScopeClient;
use crate::mcp::response::{mcp_json_response, to_mcp_result};
use crate::mcp::types::{LoginParams, ScreenerParams, SymbolsParams};
use crate::mcp::validation::{validate_param, validate_screener_params};
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    tool_handler, tool_router,
    transport::stdio,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod client;
pub mod response;
pub mod types;
pub mod validation;

/// Build a query string from key-value pairs
fn build_query(pairs: Vec<(String, String)>) -> String {
    use url::form_urlencoded;
    if pairs.is_empty() {
        return String::new();
    }
    let encoded = form_urlencoded::Serializer::new(String::new());
    pairs
        .into_iter()
        .fold(encoded, |mut acc, (k, v)| {
            acc.append_pair(&k, &v);
            acc
        })
        .finish()
}

/// MCP Router that handles all tool requests
#[derive(Clone)]
pub struct McpRouter {
    client: Arc<Mutex<CryptoScopeClient>>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl McpRouter {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            client: Arc::new(Mutex::new(CryptoScopeClient::new()?)),
            tool_router: Self::tool_router(),
        })
    }

    /// Make API GET request with proper error handling
    async fn api_call(&self, endpoint: &str) -> Result<serde_json::Value, rmcp::ErrorData> {
        let client = self.client.lock().await;
        to_mcp_result(client.get(endpoint).await)
    }

    /// Get list of available cryptocurrency exchanges
    #[rmcp::tool(description = "Get list of available cryptocurrency exchanges")]
    pub async fn get_exchanges(&self) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let result = self.api_call("exchanges").await?;
        mcp_json_response(&result)
    }

    /// Fetch symbols/instruments for a specific exchange.
    /// Supports filtering by category (linear, inverse, spot) and search.
    #[rmcp::tool]
    pub async fn get_symbols(
        &self,
        Parameters(params): Parameters<SymbolsParams>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let security_config = SecurityConfig::default_for_mcp();

        validate_param(&params.exchange, &security_config, "exchange")?;
        if let Some(ref cat) = params.category {
            validate_param(cat, &security_config, "category")?;
        }
        if let Some(ref search) = params.search {
            validate_param(search, &security_config, "search")?;
        }

        // Build query string using helper
        let mut pairs = Vec::new();
        pairs.push(("exchange".to_string(), params.exchange));
        if let Some(ref cat) = params.category {
            pairs.push(("category".to_string(), cat.clone()));
        }
        if let Some(ref search) = params.search {
            pairs.push(("search".to_string(), search.clone()));
        }

        let query = build_query(pairs);
        let endpoint = if query.is_empty() {
            "symbols".to_string()
        } else {
            format!("symbols?{query}")
        };

        let result = self.api_call(&endpoint).await?;
        mcp_json_response(&result)
    }

    /// Run price screener to find symbols matching criteria (price change %, volume, etc.)
    #[rmcp::tool(
        description = "Run price screener to find symbols matching criteria (price change %, volume, etc.)"
    )]
    pub async fn run_screener(
        &self,
        Parameters(params): Parameters<ScreenerParams>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let security_config = SecurityConfig::default_for_mcp();

        if let Some(ref exchange) = params.exchange {
            validate_param(exchange, &security_config, "exchange")?;
        }
        if let Some(ref search) = params.search {
            validate_param(search, &security_config, "search")?;
        }
        if let Some(ref contract_type) = params.contract_type {
            validate_param(contract_type, &security_config, "contract_type")?;
        }

        // Validate numeric parameters
        validate_screener_params(&params)?;

        // Build query parameters using helper
        let mut pairs = Vec::new();
        if let Some(ref exchange) = params.exchange {
            pairs.push(("exchange".to_string(), exchange.clone()));
        }
        if let Some(ref min_change) = params.min_change {
            pairs.push(("min_change".to_string(), min_change.to_string()));
        }
        if let Some(ref min_volume) = params.min_volume {
            pairs.push(("min_volume".to_string(), min_volume.to_string()));
        }
        if let Some(ref top) = params.top {
            pairs.push(("top".to_string(), top.to_string()));
        }
        if let Some(ref search) = params.search {
            pairs.push(("search".to_string(), search.clone()));
        }
        if let Some(ref contract_type) = params.contract_type {
            pairs.push(("contract_type".to_string(), contract_type.clone()));
        }

        let query = build_query(pairs);
        let endpoint = if query.is_empty() {
            "screener".to_string()
        } else {
            format!("screener?{query}")
        };

        let result = self.api_call(&endpoint).await?;
        mcp_json_response(&result)
    }

    /// Get cache statistics (total symbols, last updated, cache hit rate)
    #[rmcp::tool(
        description = "Get cache statistics (total symbols, last updated, cache hit rate)"
    )]
    pub async fn get_stats(&self) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let result = self.api_call("stats").await?;
        mcp_json_response(&result)
    }

    /// Authenticate with CryptoScope API to get JWT token for protected endpoints
    #[rmcp::tool(
        description = "Authenticate with CryptoScope API to get JWT token for protected endpoints"
    )]
    pub async fn login(
        &self,
        Parameters(params): Parameters<LoginParams>,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let body = serde_json::json!({
            "username": params.username,
            "password": params.password,
        });

        let mut client = self.client.lock().await;
        let result = to_mcp_result(client.post("auth/login", &body).await)?;

        // Extract and store token
        if let Some(token) = result.get("token").and_then(|v| v.as_str()) {
            client.set_token(token.to_string());
            tracing::debug!(
                username = %params.username,
                "MCP login successful"
            );
        }

        // Return masked response - never expose raw JWT token
        let masked_response = serde_json::json!({
            "success": true,
            "message": "Authentication successful",
            "token_stored": true
        });

        mcp_json_response(&masked_response)
    }

    /// Manually refresh the price cache (requires authentication via login first)
    #[rmcp::tool(
        description = "Manually refresh the price cache (requires authentication via login first)"
    )]
    pub async fn refresh_cache(&self) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        let client = self.client.lock().await;
        if !client.is_authenticated() {
            return Err(rmcp::ErrorData::invalid_params(
                "Authentication required. Call 'login' first with username and password."
                    .to_string(),
                None,
            ));
        }

        let result = to_mcp_result(client.post("refresh", &serde_json::json!({})).await)?;
        mcp_json_response(&result)
    }
}

#[tool_handler]
impl ServerHandler for McpRouter {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("CryptoScope MCP server for crypto market data".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

impl Default for McpRouter {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            tracing::error!("Failed to create default McpRouter: {}", e);
            // NOTE: Falls back to a minimal router if initialization fails.
            // Prefer McpRouter::new() for production use.
            // Fallback: create with default client
            Self {
                client: Arc::new(Mutex::new(CryptoScopeClient::default())),
                tool_router: Self::tool_router(),
            }
        })
    }
}

/// Run the MCP server
///
/// This function initializes the MCP server with stdio transport
/// and serves requests indefinitely.
pub async fn run() -> anyhow::Result<()> {
    let router = McpRouter::default();

    // Use stdio transport
    let transport = stdio();

    // Serve and keep running
    let service = router
        .serve(transport)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to start MCP server: {e}"))?;

    // Keep server running - waiting() blocks indefinitely
    service.waiting().await?;

    Ok(())
}
