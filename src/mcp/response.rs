//! MCP response utilities
//!
//! Provides helper functions for building MCP JSON responses.

/// Convert anyhow::Result<T> to rmcp::Result<T>
pub fn to_mcp_result<T>(result: anyhow::Result<T>) -> Result<T, rmcp::ErrorData> {
    result.map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
}

/// Convert serde_json::Error to rmcp::ErrorData
pub fn to_mcp_json(e: serde_json::Error) -> rmcp::ErrorData {
    rmcp::ErrorData::internal_error(e.to_string(), None)
}

/// Build MCP JSON response from serializable value
pub fn mcp_json_response<T: serde::Serialize>(
    value: &T,
) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
    Ok(rmcp::model::CallToolResult::success(vec![
        rmcp::model::Content::text(serde_json::to_string_pretty(value).map_err(to_mcp_json)?),
    ]))
}
