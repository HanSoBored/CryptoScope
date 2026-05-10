//! Security configuration for MCP server
use regex::Regex;
use std::path::{Path, PathBuf};

/// Security configuration for MCP server
///
/// Note: Only used in MCP binary, so dead_code warnings in library builds are expected.
#[allow(dead_code)]
pub struct SecurityConfig {
    /// Regex patterns for parameter validation
    exchange_pattern: Regex,
    category_pattern: Regex,
    search_pattern: Regex,
    contract_type_pattern: Regex,
}

impl SecurityConfig {
    #[allow(dead_code)]
    pub fn default_for_mcp() -> Self {
        Self {
            // Only allow known exchange names (e.g., "bybit")
            exchange_pattern: Regex::new(r"^(bybit)$").unwrap(),
            // Only allow "linear", "inverse", "spot"
            category_pattern: Regex::new(r"^(linear|inverse|spot)$").unwrap(),
            // Allow alphanumeric + limited special chars (-, _, space)
            search_pattern: Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap(),
            // Only allow "linear", "inverse"
            contract_type_pattern: Regex::new(r"^(linear|inverse)$").unwrap(),
        }
    }

    /// Validate parameter based on its type
    ///
    /// # Arguments
    /// * `param` - The parameter value to validate
    /// * `param_type` - The type of parameter: "exchange", "category", "search", or "contract_type"
    ///
    /// # Returns
    /// * `true` if the parameter matches the allowlist pattern for its type
    /// * `false` if the parameter is unsafe or the type is unknown
    #[allow(dead_code)]
    pub fn is_param_safe(&self, param: &str) -> bool {
        // Default to search pattern for backward compatibility
        self.is_param_safe_with_type(param, "search")
    }

    /// Validate parameter with explicit type
    #[allow(dead_code)]
    pub fn is_param_safe_with_type(&self, param: &str, param_type: &str) -> bool {
        match param_type {
            "exchange" => self.exchange_pattern.is_match(param),
            "category" => self.category_pattern.is_match(param),
            "search" => self.search_pattern.is_match(param),
            "contract_type" => self.contract_type_pattern.is_match(param),
            _ => false, // Unknown parameter type is unsafe
        }
    }

    /// Safely resolve a path to prevent path traversal attacks
    ///
    /// Uses canonicalize() + starts_with() pattern to ensure the resolved path
    /// is within the allowed base directory.
    ///
    /// # Arguments
    /// * `base` - The base directory that should contain all resolved paths
    /// * `path` - The user-provided path to resolve
    ///
    /// # Returns
    /// * `Ok(PathBuf)` - The canonicalized path if it's within the base directory
    /// * `Err(String)` - Error message if path traversal is detected or resolution fails
    #[allow(dead_code)]
    pub fn safe_resolve(base: &Path, path: &Path) -> Result<PathBuf, String> {
        // Canonicalize the base directory first
        let canonical_base = base
            .canonicalize()
            .map_err(|e| format!("Failed to canonicalize base directory: {e}"))?;

        // Join and canonicalize the target path
        let target = base.join(path);
        let canonical_target = target
            .canonicalize()
            .map_err(|e| format!("Failed to canonicalize target path: {e}"))?;

        // Ensure the resolved path starts with the base directory
        if canonical_target.starts_with(&canonical_base) {
            Ok(canonical_target)
        } else {
            Err("Path traversal detected: resolved path is outside base directory".to_string())
        }
    }
}
