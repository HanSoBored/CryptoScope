//! Parameter types for MCP tools
//!
//! These structs define the input parameters for each tool using serde for JSON serialization.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Parameters for the `get_symbols` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Validate)]
pub struct SymbolsParams {
    /// Exchange ID (e.g., "bybit")
    #[validate(length(min = 1, max = 64))]
    pub exchange: String,
    /// Symbol category: linear, inverse, or spot
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 64))]
    pub category: Option<String>,
    /// Filter symbols by name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 256))]
    pub search: Option<String>,
}

/// Parameters for the `run_screener` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ScreenerParams {
    /// Exchange ID (default: "bybit")
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 64))]
    pub exchange: Option<String>,
    /// Minimum price change percentage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_change: Option<f64>,
    /// Minimum 24h volume
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_volume: Option<f64>,
    /// Limit to top N results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<i64>,
    /// Filter by symbol name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 256))]
    pub search: Option<String>,
    /// Filter by contract type: linear, inverse
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 64))]
    pub contract_type: Option<String>,
}

/// Parameters for the `login` tool
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Validate)]
pub struct LoginParams {
    /// Username (e.g., "admin")
    #[validate(length(min = 1, max = 64))]
    pub username: String,
    /// Password
    #[validate(length(min = 1, max = 256))]
    #[schemars(skip)]
    pub password: String,
}
