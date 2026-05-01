use rusqlite::Connection;

use crate::core::error::{CryptoScopeError, Result};

/// Current schema version. Increment when making breaking changes.
const CURRENT_SCHEMA_VERSION: i64 = 1;

// --- Named SQL constants ---

const SQL_CHECK_SCHEMA_TABLE: &str =
    "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='schema_version')";

const SQL_MAX_SCHEMA_VERSION: &str = "SELECT MAX(version) FROM schema_version";

const SQL_CREATE_SCHEMA_VERSION: &str = "CREATE TABLE IF NOT EXISTS schema_version (
        version INTEGER PRIMARY KEY,
        applied_at INTEGER NOT NULL
    )";

const SQL_CREATE_DAILY_OPEN_PRICES: &str = "CREATE TABLE IF NOT EXISTS daily_open_prices (
        symbol TEXT PRIMARY KEY,
        open_price REAL NOT NULL,
        fetch_date TEXT NOT NULL,
        fetch_timestamp INTEGER NOT NULL,
        source TEXT NOT NULL
    )";

const SQL_IDX_FETCH_DATE: &str =
    "CREATE INDEX IF NOT EXISTS idx_fetch_date ON daily_open_prices(fetch_date)";

const SQL_RECORD_VERSION: &str = "INSERT OR REPLACE INTO schema_version (version, applied_at) \
     VALUES (?1, strftime('%s', 'now'))";

/// Get the current schema version from the database.
///
/// Returns `None` if the schema_version table doesn't exist yet (pre-versioning database)
/// or if the table exists but has no rows.
fn get_schema_version(conn: &Connection) -> Result<Option<i64>> {
    let table_exists: bool = conn.query_row(SQL_CHECK_SCHEMA_TABLE, [], |row| row.get(0))?;

    if !table_exists {
        return Ok(None);
    }

    // MAX() returns NULL on empty table, so we query as Option<i64>
    let version: Option<i64> = conn.query_row(SQL_MAX_SCHEMA_VERSION, [], |row| row.get(0))?;
    Ok(version)
}

/// Run migrations from the current version to the target version.
fn run_migrations(conn: &Connection, from_version: i64, to_version: i64) -> Result<()> {
    for version in (from_version + 1)..=to_version {
        match version {
            1 => {
                conn.execute_batch(&format!(
                    "{}; {}",
                    SQL_CREATE_DAILY_OPEN_PRICES, SQL_IDX_FETCH_DATE
                ))?;
            }
            _ => {
                return Err(CryptoScopeError::DbInternal(format!(
                    "Unknown schema version: {version}. Please update CryptoScope."
                )));
            }
        }

        // Record the applied version
        conn.execute(SQL_RECORD_VERSION, rusqlite::params![version])?;
    }

    Ok(())
}

/// Initialize the database schema with version tracking and migration support.
///
/// Checks the current schema version and applies any pending migrations.
/// For new databases, creates the schema_version table and applies all migrations.
/// For existing databases, reads the version and runs only the needed migrations.
pub fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute(SQL_CREATE_SCHEMA_VERSION, [])?;

    let current_version = get_schema_version(conn)?.unwrap_or(0);

    if current_version < CURRENT_SCHEMA_VERSION {
        run_migrations(conn, current_version, CURRENT_SCHEMA_VERSION)?;
    } else if current_version > CURRENT_SCHEMA_VERSION {
        return Err(CryptoScopeError::DbInternal(format!(
            "Database schema version ({current_version}) is newer than \
             supported ({CURRENT_SCHEMA_VERSION}). \
             Please update CryptoScope to a newer version."
        )));
    }

    Ok(())
}
