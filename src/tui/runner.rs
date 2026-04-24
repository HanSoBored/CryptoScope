use crate::exchange::create_exchange;
use crate::fetcher::InstrumentFetcher;
use crate::tui::app::AppState;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::cell::RefCell;
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::info;
use tracing_subscriber::EnvFilter;

struct LogGuard;

impl LogGuard {
    fn new() -> Self {
        suppress_logs();
        Self
    }
}

impl Drop for LogGuard {
    fn drop(&mut self) {
        restore_logs();
    }
}

struct TerminalGuard {
    restored: bool,
}

impl TerminalGuard {
    fn new() -> Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        crossterm::execute!(
            stdout,
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;
        Ok(Self { restored: false })
    }

    fn restore(&mut self) -> Result<()> {
        if !self.restored {
            self.restored = true;
            crossterm::terminal::disable_raw_mode()?;
            crossterm::execute!(
                io::stdout(),
                crossterm::terminal::LeaveAlternateScreen,
                crossterm::event::DisableMouseCapture
            )?;
        }
        Ok(())
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

fn suppress_logs() {
    if let Some(handle) = crate::LOG_RELOAD_HANDLE.get() {
        let _ = handle.modify(|filter| {
            *filter = EnvFilter::new("cryptoscope=error");
        });
    }
}

fn restore_logs() {
    if let Some(handle) = crate::LOG_RELOAD_HANDLE.get() {
        let level = std::env::var("RUST_LOG").unwrap_or_else(|_| "cryptoscope=info".to_string());
        let _ = handle.modify(|filter| {
            *filter = EnvFilter::new(level);
        });
    }
}

/// TUI application runner for the CryptoScope terminal interface.
///
/// Manages the terminal lifecycle, event loop, and async data fetching.
pub struct TuiApp;

impl TuiApp {
    /// Run the TUI application with the given exchange and categories.
    ///
    /// Sets up the terminal, spawns an initial data fetch, enters the
    /// event loop, and cleans up on exit.
    pub async fn run(exchange_name: &str, categories: &[&str]) -> Result<()> {
        let _log_guard = LogGuard::new();
        let mut guard = TerminalGuard::new()?;
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        let cat_strings: Vec<String> = categories.iter().map(ToString::to_string).collect();

        let state = Arc::new(RwLock::new(AppState::new(
            exchange_name.to_string(),
            cat_strings.clone(),
        )));

        // Spawn initial data fetch
        Self::spawn_fetch(&state, exchange_name, &cat_strings);

        // Main event loop
        loop {
            let click_regions_cell = RefCell::new(crate::tui::mouse::ClickRegions::new());
            
            let mut state_write = state.write().await;
            terminal.draw(|frame| {
                let regions = crate::tui::widgets::render(frame, &mut state_write);
                *click_regions_cell.borrow_mut() = regions;
            })?;
            drop(state_write);
            
            let click_regions = click_regions_cell.into_inner();

            if event::poll(Duration::from_millis(250))? {
                let event = event::read()?;
                let mut app_state = state.write().await;

                // Handle popup dismissal first (for any event)
                if app_state.popup_message.is_some() {
                    app_state.dismiss_popup();
                    if let Event::Key(_) = event {
                        continue;
                    }
                }

                match event {
                    Event::Key(key) => {
                        if key.kind != KeyEventKind::Press {
                            continue;
                        }
                        if Self::handle_key_event(&mut app_state, key, &state) {
                            // handle_key_event returned true → quit requested
                            guard.restore()?;
                            return Ok(());
                        }
                    }
                    Event::Mouse(mouse) => {
                        Self::handle_mouse_event(&mut app_state, mouse, &click_regions);
                    }
                    _ => {}
                }
            }

            let mut app_state = state.write().await;
            app_state.update_popup();
        }
    }

    /// Handle a key press event. Returns `true` if the app should quit.
    fn handle_key_event(
        app_state: &mut AppState,
        key: KeyEvent,
        state: &Arc<RwLock<AppState>>,
    ) -> bool {
        if app_state.search_mode {
            match key.code {
                KeyCode::Esc | KeyCode::Enter => {
                    app_state.search_mode = false;
                    app_state.apply_filters();
                }
                KeyCode::Char(c) => {
                    app_state.add_search_char(c);
                }
                KeyCode::Backspace => {
                    app_state.remove_search_char();
                }
                _ => {}
            }
            return false;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => true,
            KeyCode::Down | KeyCode::Char('j') => {
                app_state.select_next();
                false
            }
            KeyCode::Up | KeyCode::Char('k') => {
                app_state.select_prev();
                false
            }
            KeyCode::Char('/') => {
                app_state.toggle_search_mode();
                false
            }
            KeyCode::Tab => {
                app_state.toggle_view();
                false
            }
            KeyCode::Char('r') => {
                app_state.loading = true;
                app_state.dismiss_popup();
                Self::spawn_refresh(state, app_state);
                false
            }
            _ => false,
        }
    }

    /// Handle a mouse event by hit-testing against click regions.
    fn handle_mouse_event(
        app_state: &mut AppState,
        mouse: MouseEvent,
        click_regions: &crate::tui::mouse::ClickRegions,
    ) {
        match mouse.kind {
            MouseEventKind::ScrollDown => {
                app_state.on_scroll_down();
            }
            MouseEventKind::ScrollUp => {
                app_state.on_scroll_up();
            }
            MouseEventKind::Down(_) => {
                if let Some(action) = click_regions.hit_test(mouse.column, mouse.row) {
                    app_state.on_mouse_click(action);
                }
            }
            _ => {}
        }
    }

    /// Spawn an async task to fetch symbols from the exchange (initial load).
    fn spawn_fetch(state: &Arc<RwLock<AppState>>, exchange_name: &str, categories: &[String]) {
        let state_clone = state.clone();
        let exchange = exchange_name.to_string();
        let cats = categories.to_vec();

        tokio::spawn(async move {
            info!("Starting async fetch for exchange: {}", exchange);
            Self::do_fetch(state_clone, &exchange, &cats, false).await;
        });
    }

    /// Spawn an async task to refresh symbols from the exchange.
    fn spawn_refresh(state: &Arc<RwLock<AppState>>, app_state: &AppState) {
        let state_clone = state.clone();
        let exch = app_state.exchange_name.clone();
        let cats = app_state.categories.clone();

        tokio::spawn(async move {
            Self::do_fetch(state_clone, &exch, &cats, true).await;
        });
    }

    /// Common fetch logic used by both initial load and refresh.
    async fn do_fetch(
        state: Arc<RwLock<AppState>>,
        exchange: &str,
        categories: &[String],
        is_refresh: bool,
    ) {
        let cat_refs: Vec<&str> = categories.iter().map(|s| s.as_str()).collect();

        match create_exchange(exchange) {
            Ok(exchange_client) => {
                match InstrumentFetcher::fetch_categories(&*exchange_client, &cat_refs).await {
                    Ok(symbols) => {
                        info!("Fetched {} symbols", symbols.len());
                        let mut s = state.write().await;
                        s.set_symbols(symbols);
                        if is_refresh {
                            s.show_popup("Refresh complete".to_string(), false);
                        }
                    }
                    Err(e) => {
                        let mut s = state.write().await;
                        let prefix = if is_refresh { "Refresh" } else { "Fetch" };
                        s.show_popup(format!("{prefix} failed: {e}"), true);
                    }
                }
            }
            Err(e) => {
                let mut s = state.write().await;
                s.show_popup(format!("Exchange error: {e}"), true);
            }
        }
    }
}
