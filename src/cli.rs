use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Default, ValueEnum)]
pub enum OutputMode {
    #[default]
    Text,
    Json,
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
    #[arg(short, long, default_value = "text")]
    pub output: OutputMode,

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let cli = Cli::parse_from(["cryptoscope"]);
        assert_eq!(cli.exchange, "bybit");
        assert_eq!(cli.category, "all");
        assert!(matches!(cli.output, OutputMode::Text));
    }

    #[test]
    fn test_get_categories() {
        let cli_all = Cli::parse_from(["cryptoscope", "--category", "all"]);
        assert_eq!(cli_all.get_categories(), vec!["linear", "inverse"]);

        let cli_linear = Cli::parse_from(["cryptoscope", "--category", "linear"]);
        assert_eq!(cli_linear.get_categories(), vec!["linear"]);
    }
}
