//! Core business logic module
//!
//! Contains all the domain logic for CryptoScope including:
//! - Exchange clients and traits
//! - Data models
//! - Database operations
//! - Price screener
//! - Instrument fetching
//! - HTTP client utilities

pub mod db;
pub mod exchange;
pub mod fetcher;
pub mod http;
pub mod models;
pub mod output;
pub mod screener;
pub mod security;
pub mod utils;

mod error;
mod logging;
mod test_utils;

pub use error::CryptoScopeError;
