use crate::exchange::create_exchange;
use crate::fetcher::fetch_categories;
use crate::logging;
use crate::tui::app::AppState;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::cell::RefCell;
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::info;

type AppTerminal = Terminal<CrosstermBackend<io::Stdout>>;
type AppStateRef = Arc<RwLock<AppState>>;

/// RAII guard that suppresses logs on creation and restores them on drop.
struct LogGuard;

impl LogGuard {
    fn new() -> Self {
        logging::suppress_logs();
        Self
    }
}

impl Drop for LogGuard {
    fn drop(&mut self) {
        logging::restore_logs();
    }
}

/// Unified lifecycle manager for the TUI application.
///
/// Handles terminal setup/teardown with proper `Drop` cleanup.
/// Log suppression is managed separately by `LogGuard`.
struct TuiLifecycle {
    terminal: AppTerminal,
    state: AppStateRef,
    raw_mode_enabled: bool,
    _log_guard: LogGuard,
}

impl TuiLifecycle {
    /// Initialize the terminal and app state.
    ///
    /// Enables raw mode, enters alternate screen, enables mouse capture,
    /// and suppresses logs via `LogGuard`.
    fn init(exchange_name: &str, categories: &[&str]) -> Result<Self> {
        let _log_guard = LogGuard::new();

        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        crossterm::execute!(
            stdout,
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;

        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        let cat_strings: Vec<String> = categories.iter().map(ToString::to_string).collect();

        let state = Arc::new(RwLock::new(AppState::new(
            exchange_name.to_string(),
            cat_strings.clone(),
        )));

        // Spawn initial data fetch
        spawn_fetch(&state, exchange_name, &cat_strings);

        Ok(Self {
            terminal,
            state,
            raw_mode_enabled: true,
            _log_guard,
        })
    }

    /// Restore terminal state. Logs are restored by `LogGuard::drop`.
    fn restore(&mut self) -> Result<()> {
        if self.raw_mode_enabled {
            self.raw_mode_enabled = false;
            crossterm::terminal::disable_raw_mode()?;
            crossterm::execute!(
                io::stdout(),
                crossterm::terminal::LeaveAlternateScreen,
                crossterm::event::DisableMouseCapture
            )?;
        }
        Ok(())
    }

    fn terminal(&mut self) -> &mut AppTerminal {
        &mut self.terminal
    }

    fn state(&self) -> &AppStateRef {
        &self.state
    }
}

impl Drop for TuiLifecycle {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

/// Run the TUI application with the given exchange and categories.
pub async fn run(exchange_name: &str, categories: &[&str]) -> Result<()> {
    let mut lifecycle = TuiLifecycle::init(exchange_name, categories)?;

    let state = lifecycle.state().clone();
    event_loop(lifecycle.terminal(), &state).await?;

    Ok(())
}

/// Main event loop: render, poll events, handle input.
async fn event_loop(terminal: &mut AppTerminal, state: &AppStateRef) -> Result<()> {
    loop {
        // RefCell needed: render closure is FnMut and borrows mutably;
        // can't pass &mut click_regions through closure boundary.
        let click_regions_cell = RefCell::new(crate::tui::mouse::ClickRegions::new());

        let mut state_write = state.write().await;
        terminal.draw(|frame| {
            let regions = crate::tui::widgets::render(frame, &mut state_write);
            *click_regions_cell.borrow_mut() = regions;
        })?;
        drop(state_write);

        let click_regions = click_regions_cell.into_inner();

        if poll_and_dispatch(state, &click_regions).await? {
            return Ok(());
        }

        let mut app_state = state.write().await;
        app_state.update_popup();
    }
}

/// Poll for events and dispatch to handlers. Returns `true` if the app should quit.
async fn poll_and_dispatch(
    state: &AppStateRef,
    click_regions: &crate::tui::mouse::ClickRegions,
) -> Result<bool> {
    if !event::poll(Duration::from_millis(250))? {
        return Ok(false);
    }

    let event = event::read()?;
    let mut app_state = state.write().await;

    // Handle popup dismissal first (for any event)
    if app_state.popup_message.is_some() {
        app_state.dismiss_popup();
        if let Event::Key(_) = event {
            return Ok(false);
        }
    }

    match event {
        Event::Key(key) => {
            if key.kind != KeyEventKind::Press {
                return Ok(false);
            }
            Ok(handle_key_event(&mut app_state, key, state))
        }
        Event::Mouse(mouse) => {
            handle_mouse_event(&mut app_state, mouse, click_regions);
            Ok(false)
        }
        _ => Ok(false),
    }
}

/// Handle a key press event. Returns `true` if the app should quit.
fn handle_key_event(app_state: &mut AppState, key: KeyEvent, state: &AppStateRef) -> bool {
    if app_state.search_mode {
        handle_search_input(app_state, key);
        return false;
    }
    handle_normal_keys(app_state, key, state)
}

/// Handle key input while in search mode.
fn handle_search_input(app_state: &mut AppState, key: KeyEvent) {
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
}

/// Handle key input in normal mode. Returns `true` if the app should quit.
fn handle_normal_keys(app_state: &mut AppState, key: KeyEvent, state: &AppStateRef) -> bool {
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
            spawn_refresh(state, app_state);
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

fn spawn_fetch_task(
    state: &Arc<RwLock<AppState>>,
    exchange_name: &str,
    categories: &[String],
    is_refresh: bool,
) {
    let state_clone = state.clone();
    let exchange = exchange_name.to_string();
    let cats = categories.to_vec();

    tokio::spawn(async move {
        info!("Starting async fetch for exchange: {}", exchange);
        do_fetch(state_clone, &exchange, &cats, is_refresh).await;
    });
}

fn spawn_fetch(state: &Arc<RwLock<AppState>>, exchange_name: &str, categories: &[String]) {
    spawn_fetch_task(state, exchange_name, categories, false);
}

fn spawn_refresh(state: &Arc<RwLock<AppState>>, app_state: &AppState) {
    let exchange_name = app_state.exchange_name.clone();
    let categories = app_state.categories.clone();
    spawn_fetch_task(state, &exchange_name, &categories, true);
}

async fn do_fetch(
    state: Arc<RwLock<AppState>>,
    exchange: &str,
    categories: &[String],
    is_refresh: bool,
) {
    let cat_refs: Vec<&str> = categories.iter().map(|s| s.as_str()).collect();

    match create_exchange(exchange) {
        Ok(exchange_client) => match fetch_categories(&*exchange_client, &cat_refs).await {
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
        },
        Err(e) => {
            let mut s = state.write().await;
            s.show_popup(format!("Exchange error: {e}"), true);
        }
    }
}
