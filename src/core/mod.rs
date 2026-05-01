//! Core business logic module
//!
//! Contains all the domain logic for CryptoScope including:
//! - Exchange clients and traits
//! - Data models
//! - Database operations
//! - Price screener
//! - Instrument fetching

pub mod db;
pub mod exchange;
pub mod fetcher;
pub mod models;
pub mod output;
pub mod screener;
pub mod utils;

mod error;
mod logging;
mod test_utils;

pub use error::CryptoScopeError;
