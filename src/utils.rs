use terminal_size::{Width, terminal_size};
use tracing::warn;

/// Parse a string field to f64 with a warning on failure.
///
/// Returns `None` if parsing fails, logging a warning with context.
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
pub fn parse_f64_or_zero(field_name: &str, value: &str, symbol: &str) -> f64 {
    parse_f64(field_name, value, symbol).unwrap_or(0.0)
}

/// Get the current terminal width in characters.
///
/// Returns the terminal width if available, otherwise returns the default.
pub fn terminal_width(default: usize) -> usize {
    terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(default)
}
