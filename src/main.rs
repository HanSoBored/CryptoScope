mod cli;
mod error;
mod exchange;
mod fetcher;
mod models;
mod output;
mod tui;

use anyhow::Result;
use std::sync::OnceLock;
use std::time::Instant;
use tracing::{Level, info};
use tracing_subscriber::reload::Handle;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

use cli::{Cli, OutputMode};
use exchange::create_exchange;
use fetcher::InstrumentFetcher;
use models::Statistics;
use output::{SymbolFilter, TextFormatter, print_json};

type ReloadHandle = Handle<EnvFilter, Registry>;

static LOG_RELOAD_HANDLE: OnceLock<ReloadHandle> = OnceLock::new();

/// Run the main application logic
///
/// Handles fetching, filtering, statistics calculation, and output.
async fn run(cli: Cli) -> Result<()> {
    // Start timing
    let start_time = Instant::now();

    // Create exchange client
    let exchange = create_exchange(&cli.exchange)?;
    info!("Created exchange client: {}", exchange.name());

    // Parse categories
    let categories = cli.get_categories();
    info!("Fetching categories: {:?}", categories);

    // Fetch instruments
    let all_symbols = InstrumentFetcher::fetch_categories(&*exchange, &categories).await?;
    info!("Total symbols fetched: {}", all_symbols.len());

    // Apply filters if specified
    let filtered_symbols = SymbolFilter::apply(&all_symbols, cli.search.as_deref());

    if cli.search.is_some() {
        info!(
            "Filtered from {} to {} symbols",
            all_symbols.len(),
            filtered_symbols.len()
        );
    }

    // Calculate statistics
    let stats = Statistics::from_symbols(&filtered_symbols);

    // Calculate elapsed time
    let elapsed = start_time.elapsed();

    // Output results
    match cli.get_output_mode() {
        OutputMode::Json => {
            print_json(&cli.exchange, &categories, &filtered_symbols, &stats)?;
        }
        OutputMode::Tui => {
            tui::TuiApp::run(&cli.exchange, &categories).await?;
        }
        OutputMode::Text => {
            TextFormatter::format(&cli.exchange, &categories, &filtered_symbols, &stats);
            println!("✅ Fetch completed in {:.1}s", elapsed.as_secs_f64());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse_args();

    let log_level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("cryptoscope={log_level}")));

    let (filter_layer, reload_handle) = tracing_subscriber::reload::Layer::new(filter);

    Registry::default()
        .with(filter_layer)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false),
        )
        .init();

    let _ = LOG_RELOAD_HANDLE.set(reload_handle);

    info!("Starting CryptoScope...");
    info!(
        "Exchange: {}, Category: {:?}, Output: {:?}",
        cli.exchange,
        cli.get_categories(),
        cli.get_output_mode()
    );

    run(cli).await
}
