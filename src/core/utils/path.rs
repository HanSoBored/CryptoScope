//! Shared path validation utilities for preventing path traversal attacks.
//!
//! This module provides centralized path validation and normalization functions
//! used throughout the application to ensure database and data file paths are secure.

use std::path::{Path, PathBuf};

use crate::core::error::CryptoScopeError;

/// Result type for path validation
pub type PathResult<T> = std::result::Result<T, PathError>;

/// Path validation error types
#[derive(Debug)]
pub enum PathError {
    /// Path string is empty
    EmptyPath,
    /// Path contains null bytes
    NullBytes,
    /// Path is not in an allowed directory
    NotInAllowedDirectory(Vec<PathBuf>),
    /// IO error during path operations
    IoError(std::io::Error),
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathError::EmptyPath => write!(f, "Path cannot be empty"),
            PathError::NullBytes => write!(f, "Path contains null bytes"),
            PathError::NotInAllowedDirectory(dirs) => {
                write!(f, "Path must be in an allowed directory: {:?}", dirs)
            }
            PathError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for PathError {}

impl From<PathError> for CryptoScopeError {
    fn from(err: PathError) -> Self {
        CryptoScopeError::DbInternal(err.to_string())
    }
}

/// Normalize a path by removing `.` and `..` components without checking existence.
///
/// This function safely resolves path components without following symlinks or
/// checking if the path exists, making it suitable for security validation.
///
/// # Example
/// ```
/// use std::path::Path;
/// use cryptoscope::core::utils::path::normalize_path;
///
/// let path = Path::new("/tmp/../tmp/test.db");
/// let normalized = normalize_path(path);
/// assert_eq!(normalized.to_str().unwrap(), "/tmp/test.db");
/// ```
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {}
            _ => components.push(component),
        }
    }
    components.iter().collect()
}

/// Get safe base directory for relative paths.
///
/// Priority order:
/// 1. `CRYPTOSCOPE_DATA_DIR` environment variable (if absolute)
/// 2. XDG data local directory (`~/.local/share` on Linux)
/// 3. Current directory (development fallback, logs warning)
pub fn get_safe_base_directory() -> PathBuf {
    // Option 1: Use environment variable with validation
    if let Ok(base) = std::env::var("CRYPTOSCOPE_DATA_DIR") {
        let path = PathBuf::from(base);
        if path.is_absolute() {
            return path;
        }
    }

    // Option 2: Use standard data directory
    if let Some(data_dir) = dirs::data_local_dir() {
        return data_dir.join("cryptoscope");
    }

    // Option 3: Use current directory (least secure, for development only)
    tracing::warn!("Using current directory as data directory (development mode)");
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Get list of allowed parent directories for absolute paths.
///
/// Returns directories where database files are permitted to be stored:
/// 1. XDG data local directory (`~/.local/share/cryptoscope`)
/// 2. Directories from `ALLOWED_DB_PATHS` environment variable (colon-separated)
pub fn get_allowed_parent_directories() -> Vec<PathBuf> {
    let mut allowed = Vec::new();

    // Add data directory
    if let Some(data_dir) = dirs::data_local_dir() {
        allowed.push(data_dir.join("cryptoscope"));
    }

    // Add configured allowed directories from environment
    if let Ok(allowed_dirs) = std::env::var("ALLOWED_DB_PATHS") {
        for dir in allowed_dirs.split(':') {
            allowed.push(PathBuf::from(dir));
        }
    }

    allowed
}

/// Validate and normalize a path string, ensuring it's safe from path traversal.
///
/// This function:
/// 1. Rejects empty paths and paths with null bytes
/// 2. Resolves relative paths against a safe base directory
/// 3. Normalizes the path to remove `.` and `..` components
/// 4. Verifies absolute paths are in allowed directories
/// 5. Creates parent directories if they don't exist
///
/// # Arguments
/// * `raw_path` - The raw path string to validate
///
/// # Returns
/// * `Ok(PathBuf)` - The validated and normalized path
/// * `Err(PathError)` - The validation error
pub fn validate_and_normalize_path(raw_path: &str) -> PathResult<PathBuf> {
    if raw_path.is_empty() {
        return Err(PathError::EmptyPath);
    }

    if raw_path.contains('\0') {
        return Err(PathError::NullBytes);
    }

    let path = Path::new(raw_path);
    let safe_base = get_safe_base_directory();
    let full_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        safe_base.join(path)
    };

    let normalized = normalize_path(&full_path);

    // For absolute paths, verify they're in an allowed location
    if raw_path.starts_with('/') {
        let allowed_parents = get_allowed_parent_directories();
        let is_allowed = allowed_parents.iter().any(|p| normalized.starts_with(p));
        if !is_allowed {
            return Err(PathError::NotInAllowedDirectory(allowed_parents));
        }
    }

    // Ensure parent directory exists
    if let Some(parent) = normalized.parent() {
        std::fs::create_dir_all(parent).map_err(PathError::IoError)?;
    }

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path_removes_parent_dirs() {
        let path = Path::new("/tmp/../tmp/test.db");
        let normalized = normalize_path(path);
        assert_eq!(normalized.to_str().unwrap(), "/tmp/test.db");
    }

    #[test]
    fn test_normalize_path_removes_current_dir() {
        let path = Path::new("/tmp/./test.db");
        let normalized = normalize_path(path);
        assert_eq!(normalized.to_str().unwrap(), "/tmp/test.db");
    }

    #[test]
    fn test_validate_path_rejects_empty() {
        let result = validate_and_normalize_path("");
        assert!(matches!(result, Err(PathError::EmptyPath)));
    }

    #[test]
    fn test_validate_path_rejects_null_bytes() {
        let result = validate_and_normalize_path("/tmp/test\0.db");
        assert!(matches!(result, Err(PathError::NullBytes)));
    }

    #[test]
    fn test_validate_path_accepts_valid_relative_path() {
        let result = validate_and_normalize_path("./test.db");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_accepts_valid_absolute_path() {
        // Use a path in the current directory which should be in allowed directories
        // Set ALLOWED_DB_PATHS to include current directory for testing
        let current_dir = std::env::current_dir().unwrap();
        unsafe {
            std::env::set_var("ALLOWED_DB_PATHS", current_dir.to_str().unwrap());
        }
        
        let test_path = current_dir.join("test.db");
        let result = validate_and_normalize_path(test_path.to_str().unwrap());
        
        // Clean up
        unsafe {
            std::env::remove_var("ALLOWED_DB_PATHS");
        }
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_safe_base_directory_returns_absolute_path() {
        let base = get_safe_base_directory();
        assert!(base.is_absolute());
    }

    #[test]
    fn test_get_allowed_parent_directories_not_empty() {
        let allowed = get_allowed_parent_directories();
        // Should have at least the data directory or current dir
        assert!(!allowed.is_empty());
    }
}
