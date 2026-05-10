//! MCP parameter validation utilities
//!
//! Provides validation functions for MCP tool parameters.

use crate::core::security::SecurityConfig;
use crate::mcp::types::ScreenerParams;

/// Validate parameter against security config
pub fn validate_param(
    param: &str,
    config: &SecurityConfig,
    param_type: &str,
) -> Result<(), rmcp::ErrorData> {
    if !config.is_param_safe_with_type(param, param_type) {
        return Err(rmcp::ErrorData::invalid_params(
            format!(
                "Invalid parameter '{}': does not match allowed pattern for type '{}'",
                param, param_type
            ),
            None,
        ));
    }
    Ok(())
}

/// Validate screener numeric parameters
pub fn validate_screener_params(params: &ScreenerParams) -> Result<(), rmcp::ErrorData> {
    // Validate top: must be 1-1000
    if let Some(top) = params.top {
        if top < 1 || top > 1000 {
            return Err(rmcp::ErrorData::invalid_params(
                "Parameter 'top' must be between 1 and 1000".to_string(),
                None,
            ));
        }
    }

    // Validate min_change: must be finite and in range -100 to 100
    if let Some(min_change) = params.min_change {
        if !min_change.is_finite() {
            return Err(rmcp::ErrorData::invalid_params(
                "Parameter 'min_change' must be a finite number (not NaN or Infinity)".to_string(),
                None,
            ));
        }
        if min_change < -100.0 || min_change > 100.0 {
            return Err(rmcp::ErrorData::invalid_params(
                "Parameter 'min_change' must be between -100 and 100".to_string(),
                None,
            ));
        }
    }

    // Validate min_volume: must be finite and non-negative
    if let Some(min_volume) = params.min_volume {
        if !min_volume.is_finite() {
            return Err(rmcp::ErrorData::invalid_params(
                "Parameter 'min_volume' must be a finite number (not NaN or Infinity)".to_string(),
                None,
            ));
        }
        if min_volume < 0.0 {
            return Err(rmcp::ErrorData::invalid_params(
                "Parameter 'min_volume' must be non-negative".to_string(),
                None,
            ));
        }
    }

    Ok(())
}
