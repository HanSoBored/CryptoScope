use rusqlite::Connection;
use std::path::PathBuf;

use crate::error::{CryptoScopeError, Result};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Get the database file path at ~/.cryptoscope/data.db
///
/// Uses XDG_DATA_HOME on Linux or falls back to home directory.
/// Creates the directory if it doesn't exist.
pub fn get_database_path() -> Result<PathBuf> {
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
/// Opens or creates the database at the standard path.
pub fn create_connection() -> Result<Connection> {
    let db_path = get_database_path()?;

    let conn = Connection::open(&db_path)?;

    #[cfg(unix)]
    std::fs::set_permissions(&db_path, std::fs::Permissions::from_mode(0o600)).ok();

    Ok(conn)
}
