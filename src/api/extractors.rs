//! Re-exports from error module for backward compatibility.
//!
//! All extractor functionality has been moved to `crate::api::error`.
//! This module exists for backward compatibility.

#[allow(unused_imports)]
pub use crate::api::error::{AppError, HandlerResult, ValidatedQuery};
