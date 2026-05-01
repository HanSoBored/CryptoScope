//! Utility modules for CryptoScope.
//!
//! Provides shared functionality including:
//! - `parse`: String-to-number parsing utilities
//! - `path`: Path validation and normalization for security

pub mod parse;
pub mod path;

pub use parse::{parse_f64, parse_f64_or_zero};
// path module exports are available via crate::core::utils::path::*
