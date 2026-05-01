//! Parsing utilities for string-to-number conversion with error handling.

use tracing::warn;

/// Parse a string field to f64 with a warning on failure.
///
/// Returns `None` if parsing fails, logging a warning with context.
///
/// # Arguments
/// * `field_name` - Name of the field being parsed (for logging)
/// * `value` - The string value to parse
/// * `symbol` - The symbol/context this value belongs to (for logging)
///
/// # Example
/// ```
/// use cryptoscope::core::utils::parse::parse_f64;
///
/// let result = parse_f64("price", "100.50", "BTCUSDT");
/// assert_eq!(result, Some(100.50));
///
/// let invalid = parse_f64("price", "invalid", "BTCUSDT");
/// assert_eq!(invalid, None);
/// ```
pub fn parse_f64(field_name: &str, value: &str, symbol: &str) -> Option<f64> {
    value.parse::<f64>().ok().or_else(|| {
        warn!(
            "Failed to parse {} '{}' for symbol '{}'",
            field_name, value, symbol
        );
        None
    })
}

/// Parse a string field to f64, defaulting to 0.0 on failure.
///
/// Convenience wrapper for cases where a default of 0.0 is acceptable.
///
/// # Arguments
/// * `field_name` - Name of the field being parsed (for logging)
/// * `value` - The string value to parse
/// * `symbol` - The symbol/context this value belongs to (for logging)
///
/// # Example
/// ```
/// use cryptoscope::core::utils::parse::parse_f64_or_zero;
///
/// let result = parse_f64_or_zero("volume", "1000.50", "BTCUSDT");
/// assert_eq!(result, 1000.50);
///
/// let invalid = parse_f64_or_zero("volume", "invalid", "BTCUSDT");
/// assert_eq!(invalid, 0.0);
/// ```
pub fn parse_f64_or_zero(field_name: &str, value: &str, symbol: &str) -> f64 {
    parse_f64(field_name, value, symbol).unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_f64_valid() {
        let result = parse_f64("price", "100.50", "BTCUSDT");
        assert_eq!(result, Some(100.50));
    }

    #[test]
    fn test_parse_f64_invalid() {
        let result = parse_f64("price", "invalid", "BTCUSDT");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_f64_or_zero_valid() {
        let result = parse_f64_or_zero("volume", "1000.50", "BTCUSDT");
        assert!((result - 1000.50).abs() < f64::EPSILON);
    }

    #[test]
    fn test_parse_f64_or_zero_invalid() {
        let result = parse_f64_or_zero("volume", "invalid", "BTCUSDT");
        assert_eq!(result, 0.0);
    }
}
