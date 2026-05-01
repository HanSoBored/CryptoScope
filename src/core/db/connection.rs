use rusqlite::Connection;
use std::path::PathBuf;

use crate::core::error::{CryptoScopeError, Result};
use crate::core::utils::path::validate_and_normalize_path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Get the database file path
///
/// Uses DATABASE_PATH environment variable if set, otherwise falls back to
/// the standard path at ~/.cryptoscope/data.db.
///
/// Uses XDG_DATA_HOME on Linux or falls back to home directory.
/// Creates the directory if it doesn't exist.
pub fn get_database_path() -> Result<PathBuf> {
    // Check for DATABASE_PATH environment variable
    if let Ok(db_path) = std::env::var("DATABASE_PATH") {
        return validate_and_normalize_path(&db_path).map_err(|e| {
            CryptoScopeError::DbInternal(format!("Path validation failed: {}", e))
        });
    }

    // Fall back to standard location
    let base_dir = dirs::data_dir()
        .or_else(dirs::home_dir)
        .ok_or_else(|| CryptoScopeError::DbInternal("No home directory found".to_string()))?
        .join("cryptoscope");

    std::fs::create_dir_all(&base_dir).map_err(|e| {
        CryptoScopeError::DbInternal(format!("Failed to create database directory: {}", e))
    })?;

    Ok(base_dir.join("data.db"))
}

/// Create a new SQLite database connection
///
/// Opens or creates the database at the standard path or DATABASE_PATH.
pub fn create_connection() -> Result<Connection> {
    let db_path = get_database_path()?;

    tracing::debug!("Opening database at: {:?}", db_path);

    let conn = Connection::open(&db_path)?;

    #[cfg(unix)]
    std::fs::set_permissions(&db_path, std::fs::Permissions::from_mode(0o600)).ok();

    Ok(conn)
}
