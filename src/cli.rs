use clap::{Parser, Subcommand, ValueEnum};

/// Screener mode: Ticker (fast, rolling 24h) or Kline (accurate, true daily open).
#[derive(Debug, Clone, Copy, Default, PartialEq, ValueEnum)]
pub enum ScreenerMode {
    /// Ticker mode uses rolling 24h price (prev_price_24h) - inaccurate for daily open
    Ticker,
    /// Kline mode uses true 00:00 UTC daily open from K-line endpoint - accurate
    #[default]
    Kline,
}

/// Parse category string into a vector of category names.
/// Returns an error for unknown categories instead of silently defaulting.
pub(crate) fn parse_categories(
    category: &str,
) -> Result<Vec<&'static str>, crate::error::CryptoScopeError> {
    match category.to_lowercase().as_str() {
        "all" => Ok(vec!["linear", "inverse"]),
        "linear" => Ok(vec!["linear"]),
        "inverse" => Ok(vec!["inverse"]),
        _ => Err(crate::error::CryptoScopeError::ApiError {
            code: -1,
            message: format!(
                "Unknown category '{}'. Supported: all, linear, inverse",
                category
            ),
        }),
    }
}

/// Parse a non-negative f64 value for clap argument validation.
fn parse_non_negative_f64(s: &str) -> Result<f64, String> {
    let v: f64 = s.parse().map_err(|e| format!("Invalid number: {e}"))?;
    if v < 0.0 {
        Err(format!("Value must be non-negative, got {v}"))
    } else {
        Ok(v)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, ValueEnum)]
pub enum OutputMode {
    Text,
    Json,
    #[default]
    Tui,
}

/// Shared arguments for exchange and category selection.
#[derive(Parser, Debug, Clone)]
pub struct CommonArgs {
    /// Exchange to fetch from (e.g., bybit)
    #[arg(short, long, default_value = "bybit")]
    pub exchange: String,

    /// Category: linear (USDT perpetuals), inverse, or all
    /// Default: all - shows both linear and inverse perpetual prices
    #[arg(short = 'c', long, default_value = "all")]
    pub category: String,
}

/// Parse categories from shared CLI arguments.
pub fn parse_categories_from(
    common: &CommonArgs,
) -> Result<Vec<&'static str>, crate::error::CryptoScopeError> {
    parse_categories(&common.category)
}

/// Screener subcommand for price screening
#[derive(Parser, Debug)]
pub struct ScreenerCmd {
    #[command(flatten)]
    pub common: CommonArgs,

    /// Screener mode: ticker (fast, rolling 24h) or kline (accurate, true daily open)
    /// Default: kline - uses true 00:00 UTC open price from K-line endpoint
    #[arg(long, default_value = "kline")]
    pub mode: ScreenerMode,

    /// Force refresh open prices (ignore cache)
    #[arg(long)]
    pub force_refresh: bool,

    /// Show only top N symbols by change %
    #[arg(long)]
    pub top: Option<usize>,

    /// Filter by minimum change % (absolute value)
    #[arg(long, value_parser = parse_non_negative_f64)]
    pub min_change: Option<f64>,

    /// Filter by minimum 24h volume (in USDT)
    #[arg(long, value_parser = parse_non_negative_f64)]
    pub min_volume: Option<f64>,

    /// Search for specific symbol
    #[arg(long)]
    pub symbol: Option<String>,
}

/// Command-line interface configuration for CryptoScope.
#[derive(Parser, Debug)]
#[command(name = "cryptoscope")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub common: CommonArgs,

    /// Output format: text, json, or tui
    #[arg(short, long, default_value = "tui")]
    pub output: OutputMode,

    /// Use CLI text output instead of TUI (shorthand for --output text)
    #[arg(long, conflicts_with = "output")]
    pub cli: bool,

    /// Search symbols by name (case-insensitive)
    #[arg(long)]
    pub search: Option<String>,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Available subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the price screener to find symbols with significant price changes
    Screener(ScreenerCmd),
}

impl Cli {
    /// Parse command line arguments using clap.
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Get the effective output mode.
    ///
    /// Returns Text if --cli flag is set, otherwise returns the configured output mode.
    pub fn get_output_mode(&self) -> OutputMode {
        if self.cli {
            OutputMode::Text
        } else {
            self.output
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let cli = Cli::parse_from(["cryptoscope"]);
        assert_eq!(cli.common.exchange, "bybit");
        assert_eq!(cli.common.category, "all");
        assert!(matches!(cli.output, OutputMode::Tui));
    }

    #[test]
    fn test_parse_categories_from() {
        let cli_all = Cli::parse_from(["cryptoscope", "--category", "all"]);
        assert_eq!(
            parse_categories_from(&cli_all.common).unwrap(),
            vec!["linear", "inverse"]
        );

        let cli_linear = Cli::parse_from(["cryptoscope", "--category", "linear"]);
        assert_eq!(
            parse_categories_from(&cli_linear.common).unwrap(),
            vec!["linear"]
        );
    }

    #[test]
    fn test_cli_flag() {
        // Test --cli flag sets output to Text
        let cli = Cli::parse_from(["cryptoscope", "--cli"]);
        assert!(cli.cli);
        assert_eq!(cli.get_output_mode(), OutputMode::Text);

        // Test default (no --cli) uses configured output (tui by default)
        let cli_default = Cli::parse_from(["cryptoscope"]);
        assert!(!cli_default.cli);
        assert_eq!(cli_default.get_output_mode(), OutputMode::Tui);

        // Test --output text still works
        let cli_text = Cli::parse_from(["cryptoscope", "--output", "text"]);
        assert!(!cli_text.cli);
        assert_eq!(cli_text.get_output_mode(), OutputMode::Text);
    }

    #[test]
    fn test_cli_with_search() {
        // Test --cli combined with --search
        let cli = Cli::parse_from(["cryptoscope", "--cli", "--search", "BTC"]);
        assert!(cli.cli);
        assert_eq!(cli.get_output_mode(), OutputMode::Text);
        assert_eq!(cli.search, Some("BTC".to_string()));
    }

    #[test]
    fn test_cli_with_verbose() {
        // Test --cli combined with --verbose
        let cli = Cli::parse_from(["cryptoscope", "--cli", "--verbose"]);
        assert!(cli.cli);
        assert!(cli.verbose);
        assert_eq!(cli.get_output_mode(), OutputMode::Text);
    }

    #[test]
    fn test_cli_with_category() {
        // Test --cli combined with --category
        let cli = Cli::parse_from(["cryptoscope", "--cli", "--category", "linear"]);
        assert!(cli.cli);
        assert_eq!(cli.common.category, "linear");
        assert_eq!(parse_categories_from(&cli.common).unwrap(), vec!["linear"]);
        assert_eq!(cli.get_output_mode(), OutputMode::Text);
    }

    #[test]
    fn test_parse_categories_all() {
        assert_eq!(parse_categories("all").unwrap(), vec!["linear", "inverse"]);
        assert_eq!(parse_categories("ALL").unwrap(), vec!["linear", "inverse"]);
        // Unknown categories now return an error
        assert!(parse_categories("anything").is_err());
    }

    #[test]
    fn test_parse_categories_specific() {
        assert_eq!(parse_categories("linear").unwrap(), vec!["linear"]);
        assert_eq!(parse_categories("LINEAR").unwrap(), vec!["linear"]);
        assert_eq!(parse_categories("inverse").unwrap(), vec!["inverse"]);
        assert_eq!(parse_categories("INVERSE").unwrap(), vec!["inverse"]);
    }

    #[test]
    fn test_screener_cmd_defaults() {
        let cmd = ScreenerCmd::parse_from(["screener"]);
        assert!(matches!(cmd.mode, ScreenerMode::Kline));
        assert_eq!(cmd.common.category, "all");
        assert!(!cmd.force_refresh);
        assert!(cmd.top.is_none());
        assert!(cmd.min_change.is_none());
        assert!(cmd.min_volume.is_none());
        assert!(cmd.symbol.is_none());
    }

    #[test]
    fn test_parse_non_negative_f64_valid() {
        assert_eq!(parse_non_negative_f64("0").unwrap(), 0.0);
        assert_eq!(parse_non_negative_f64("42.5").unwrap(), 42.5);
        assert_eq!(parse_non_negative_f64("1e10").unwrap(), 1e10);
    }

    #[test]
    fn test_parse_non_negative_f64_negative() {
        assert!(parse_non_negative_f64("-1").is_err());
        assert!(parse_non_negative_f64("-0.01").is_err());
    }

    #[test]
    fn test_parse_non_negative_f64_nan() {
        // NaN parses as f64::NAN but is not < 0.0, so it passes the check.
        // This is acceptable — NaN is a valid f64 value for filtering purposes.
        let result = parse_non_negative_f64("NaN");
        assert!(result.is_ok());
        assert!(result.unwrap().is_nan());
    }

    #[test]
    fn test_parse_non_negative_f64_infinity() {
        let result = parse_non_negative_f64("inf");
        assert!(result.is_ok());
        assert!(result.unwrap().is_infinite());

        let result_neg = parse_non_negative_f64("-inf");
        assert!(result_neg.is_err());
    }

    #[test]
    fn test_parse_non_negative_f64_negative_zero() {
        // -0 parses to 0.0 which is >= 0.0, so it's accepted
        let result = parse_non_negative_f64("-0");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0.0);
    }

    #[test]
    fn test_parse_non_negative_f64_very_large() {
        // f64::MAX parses successfully and is finite
        let result = parse_non_negative_f64("1.7976931348623157e308");
        assert!(result.is_ok());
        assert!(result.unwrap().is_finite());

        // A value larger than f64::MAX overflows to infinity
        let result = parse_non_negative_f64("1e309");
        assert!(result.is_ok());
        assert!(result.unwrap().is_infinite());
    }

    #[test]
    fn test_parse_non_negative_f64_non_numeric() {
        assert!(parse_non_negative_f64("abc").is_err());
        assert!(parse_non_negative_f64("").is_err());
        assert!(parse_non_negative_f64("12abc").is_err());
    }
}
