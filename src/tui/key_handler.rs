use crate::models::ContractType;
use crate::tui::app::{AppState, AppView, Direction};
use crossterm::event::{KeyCode, KeyEvent};

/// Result of navigation key handling.
pub enum NavResult {
    /// User pressed q or Esc — app should quit.
    Quit,
    /// User pressed 'r' — caller should trigger a refresh.
    Refresh,
    /// Key was handled, no further action needed.
    Consumed,
    /// Key was not handled.
    Ignored,
}

/// Handle a key press event. Returns `NavResult` indicating what the caller should do.
pub fn handle_key_event(
    app_state: &mut AppState,
    key: KeyEvent,
) -> NavResult {
    if app_state.is_search_mode() {
        handle_search_input(app_state, key);
        return NavResult::Consumed;
    }
    handle_normal_keys(app_state, key)
}

/// Handle key input while in search mode.
fn handle_search_input(app_state: &mut AppState, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Enter => {
            app_state.toggle_search_mode();
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

/// Handle navigation keys (quit, scroll, search, view toggle/switch, refresh, screener/symbol list).
fn handle_nav_keys(app_state: &mut AppState, key: &KeyCode) -> NavResult {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => NavResult::Quit,
        KeyCode::Down | KeyCode::Char('j') => {
            app_state.scroll_view(Direction::Next);
            NavResult::Consumed
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app_state.scroll_view(Direction::Previous);
            NavResult::Consumed
        }
        KeyCode::Char('/') => {
            app_state.toggle_search_mode();
            NavResult::Consumed
        }
        KeyCode::Tab => {
            app_state.toggle_view();
            NavResult::Consumed
        }
        KeyCode::Char('s') => {
            app_state.navigate_to_screener();
            NavResult::Consumed
        }
        KeyCode::Char('l') => {
            app_state.navigate_to_symbol_list();
            NavResult::Consumed
        }
        KeyCode::Char('r') => NavResult::Refresh,
        _ => NavResult::Ignored,
    }
}

/// Handle screener-specific keys.
fn handle_screener_keys(app_state: &mut AppState, key: &KeyCode) -> NavResult {
    if !matches!(app_state.view, AppView::Screener) {
        return NavResult::Ignored;
    }
    match key {
        KeyCode::Char('S') => {
            app_state.toggle_screener_sort_desc();
            NavResult::Consumed
        }
        KeyCode::Char('o') | KeyCode::Char('O') => {
            app_state.toggle_screener_sort();
            NavResult::Consumed
        }
        _ => NavResult::Ignored,
    }
}

/// Handle contract type filter keys (0-4).
fn handle_contract_filter_keys(app_state: &mut AppState, key: &KeyCode) -> NavResult {
    if matches!(app_state.view, AppView::Screener) {
        return NavResult::Ignored;
    }
    let contract_type = match key {
        KeyCode::Char('1') => ContractType::LinearPerpetual,
        KeyCode::Char('2') => ContractType::LinearFutures,
        KeyCode::Char('3') => ContractType::InversePerpetual,
        KeyCode::Char('4') => ContractType::InverseFutures,
        KeyCode::Char('0') => {
            app_state.select_all_contract_types();
            return NavResult::Consumed;
        }
        _ => return NavResult::Ignored,
    };
    app_state.toggle_contract_type(contract_type);
    NavResult::Consumed
}

/// Handle key input in normal mode. Returns `NavResult` indicating what the caller should do.
///
/// Handlers are called in priority order: nav → screener → contract filter.
fn handle_normal_keys(app_state: &mut AppState, key: KeyEvent) -> NavResult {
    match handle_nav_keys(app_state, &key.code) {
        NavResult::Quit => return NavResult::Quit,
        NavResult::Refresh => return NavResult::Refresh,
        NavResult::Consumed => return NavResult::Consumed,
        NavResult::Ignored => {}
    }
    if matches!(
        handle_screener_keys(app_state, &key.code),
        NavResult::Consumed
    ) {
        return NavResult::Consumed;
    }
    if matches!(
        handle_contract_filter_keys(app_state, &key.code),
        NavResult::Consumed
    ) {
        return NavResult::Consumed;
    }
    NavResult::Ignored
}
