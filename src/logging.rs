use std::sync::OnceLock;
use tracing::Level;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

pub type ReloadHandle = Handle<EnvFilter, Registry>;

static LOG_RELOAD_HANDLE: OnceLock<ReloadHandle> = OnceLock::new();

/// Get the global log reload handle.
pub fn get_reload_handle() -> Option<&'static ReloadHandle> {
    LOG_RELOAD_HANDLE.get()
}

/// Set the global log reload handle (call once during initialization).
pub fn set_reload_handle(handle: ReloadHandle) {
    let _ = LOG_RELOAD_HANDLE.set(handle);
}

/// Initialize logging and store the reload handle globally.
pub fn init_logging(verbose: bool) {
    let log_level = if verbose { Level::DEBUG } else { Level::INFO };

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

    set_reload_handle(reload_handle);
}

/// Suppress logs to error level only (for TUI mode).
pub fn suppress_logs() {
    if let Some(handle) = get_reload_handle() {
        let _ = handle.modify(|filter| {
            *filter = EnvFilter::new("cryptoscope=error");
        });
    }
}

/// Restore logs to the configured level (or default to info).
pub fn restore_logs() {
    if let Some(handle) = get_reload_handle() {
        let level = std::env::var("RUST_LOG").unwrap_or_else(|_| "cryptoscope=info".to_string());
        let _ = handle.modify(|filter| {
            *filter = EnvFilter::new(level);
        });
    }
}
