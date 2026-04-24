use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, Default, PartialEq, ValueEnum)]
pub enum OutputMode {
    Text,
    Json,
    #[default]
    Tui,
}

/// Command-line interface configuration for CryptoScope.
///
/// Parses and validates command-line arguments for controlling
/// exchange selection, category filtering, output format, and more.
#[derive(Parser, Debug)]
#[command(name = "cryptoscope")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Exchange to fetch from (e.g., bybit)
    #[arg(short, long, default_value = "bybit")]
    pub exchange: String,

    /// Category to fetch: linear, inverse, or all
    #[arg(short, long, default_value = "all")]
    pub category: String,

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
}

impl Cli {
    /// Parse command line arguments
    ///
    /// Uses clap to parse and validate command-line arguments.
    pub fn parse_args() -> Self {
        Self::parse()
    }

    /// Get categories as vector
    ///
    /// Converts the category argument into a list of categories to fetch.
    /// "linear" returns `["linear"]`, "inverse" returns `["inverse"]`,
    /// and "all" (or any other value) returns `["linear", "inverse"]`.
    pub fn get_categories(&self) -> Vec<&str> {
        match self.category.to_lowercase().as_str() {
            "linear" => vec!["linear"],
            "inverse" => vec!["inverse"],
            _ => vec!["linear", "inverse"],
        }
    }

    /// Get the effective output mode
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
        assert_eq!(cli.exchange, "bybit");
        assert_eq!(cli.category, "all");
        assert!(matches!(cli.output, OutputMode::Tui));
    }

    #[test]
    fn test_get_categories() {
        let cli_all = Cli::parse_from(["cryptoscope", "--category", "all"]);
        assert_eq!(cli_all.get_categories(), vec!["linear", "inverse"]);

        let cli_linear = Cli::parse_from(["cryptoscope", "--category", "linear"]);
        assert_eq!(cli_linear.get_categories(), vec!["linear"]);
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
        assert_eq!(cli.category, "linear");
        assert_eq!(cli.get_categories(), vec!["linear"]);
        assert_eq!(cli.get_output_mode(), OutputMode::Text);
    }
}
