use crate::models::Statistics;
use crate::models::symbol::Symbol;
use crate::output::SymbolFilter;
use crate::tui::mouse::{ClickAction, HeaderTab, ScrollDirection};
use ratatui::widgets::TableState;
use std::time::Instant;

/// Represents the current view mode of the TUI application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    SymbolList,
    StatsDashboard,
}

/// Shared application state managed behind `Arc<RwLock<AppState>>`.
///
/// Holds all mutable UI state including symbols, selection, search,
/// view mode, loading status, and popup messages.
#[derive(Debug)]
pub struct AppState {
    pub symbols: Vec<Symbol>,
    pub filtered: Vec<Symbol>,
    /// Table selection state. Synchronized by the outer `RwLock<AppState>`.
    pub table_state: TableState,
    pub search: String,
    pub search_mode: bool,
    pub view: AppView,
    pub loading: bool,
    pub stats: Option<Statistics>,
    pub exchange_name: String,
    pub categories: Vec<String>,
    pub popup_message: Option<String>,
    pub popup_timer: Option<Instant>,
    pub popup_is_error: bool,
}

impl AppState {
    /// Create a new `AppState` for the given exchange and categories.
    pub fn new(exchange_name: String, categories: Vec<String>) -> Self {
        Self {
            symbols: Vec::new(),
            filtered: Vec::new(),
            table_state: TableState::default(),
            search: String::new(),
            search_mode: false,
            view: AppView::SymbolList,
            loading: true,
            stats: None,
            exchange_name,
            categories,
            popup_message: None,
            popup_timer: None,
            popup_is_error: false,
        }
    }

    /// Replace the symbol list, re-apply filters, and compute statistics.
    ///
    /// Also resets the loading flag and initializes table selection if needed.
    pub fn set_symbols(&mut self, symbols: Vec<Symbol>) {
        self.symbols = symbols;
        self.apply_filters();
        self.stats = Some(Statistics::from_symbols(&self.filtered));
        self.loading = false;
        if self.table_state.selected().is_none() && !self.filtered.is_empty() {
            self.table_state.select(Some(0));
        }
    }

    /// Re-apply the current search filter to the full symbol list.
    ///
    /// Adjusts table selection if the current index is out of bounds.
    pub fn apply_filters(&mut self) {
        let mut filtered = self.symbols.clone();

        if !self.search.is_empty() {
            filtered = SymbolFilter::by_search(&filtered, &self.search);
        }

        self.filtered = filtered;

        let needs_adjust = self
            .table_state
            .selected()
            .is_none_or(|s| s >= self.filtered.len());
        if needs_adjust {
            let new_sel = self.filtered.len().saturating_sub(1);
            self.table_state
                .select(if new_sel == 0 && self.filtered.is_empty() {
                    None
                } else {
                    Some(new_sel)
                });
        }
    }

    /// Move the table selection to the next row (wraps to top).
    pub fn select_next(&mut self) {
        if self.filtered.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.filtered.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    /// Move the table selection to the previous row (wraps to bottom).
    pub fn select_prev(&mut self) {
        if self.filtered.is_empty() {
            return;
        }
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    /// Toggle between the symbol list and stats dashboard views.
    pub fn toggle_view(&mut self) {
        self.view = match &self.view {
            AppView::SymbolList => AppView::StatsDashboard,
            AppView::StatsDashboard => AppView::SymbolList,
        };
    }

    /// Toggle search mode on/off. Clears the search buffer when entering.
    pub fn toggle_search_mode(&mut self) {
        if self.search_mode {
            self.search_mode = false;
            self.apply_filters();
        } else {
            self.search_mode = true;
            self.search.clear();
        }
    }

    /// Append a character to the search buffer and re-apply filters.
    pub fn add_search_char(&mut self, c: char) {
        self.search.push(c);
        self.apply_filters();
    }

    /// Remove the last character from the search buffer and re-apply filters.
    pub fn remove_search_char(&mut self) {
        self.search.pop();
        self.apply_filters();
    }

    /// Show a popup message with the given error/info flag.
    pub fn show_popup(&mut self, message: String, is_error: bool) {
        self.popup_message = Some(message);
        self.popup_timer = Some(Instant::now());
        self.popup_is_error = is_error;
    }

    /// Check if the popup auto-dismiss timer has elapsed (5 seconds).
    pub fn update_popup(&mut self) {
        if let Some(timer) = self.popup_timer
            && timer.elapsed().as_secs() >= 5
        {
            self.popup_message = None;
            self.popup_timer = None;
        }
    }

    /// Dismiss the current popup message immediately.
    pub fn dismiss_popup(&mut self) {
        self.popup_message = None;
        self.popup_timer = None;
    }

    /// Handle scroll up action (scroll wheel up or click on scroll up button).
    pub fn on_scroll_up(&mut self) {
        self.select_prev();
    }

    /// Handle scroll down action (scroll wheel down or click on scroll down button).
    pub fn on_scroll_down(&mut self) {
        self.select_next();
    }

    /// Handle click on a table row at the given index.
    pub fn on_table_click(&mut self, row_index: usize) {
        if row_index < self.filtered.len() {
            self.table_state.select(Some(row_index));
        }
    }

    /// Handle click on a header tab.
    pub fn on_header_tab_click(&mut self, tab: HeaderTab) {
        self.view = match tab {
            HeaderTab::SymbolList => AppView::SymbolList,
            HeaderTab::StatsDashboard => AppView::StatsDashboard,
        };
    }

    /// Handle click on scrollbar track for page up/down.
    pub fn on_scrollbar_track_click(&mut self, direction: ScrollDirection) {
        if self.filtered.is_empty() {
            return;
        }
        // Jump 20% of list or 10 rows, whichever is larger
        let jump = 10.max(self.filtered.len() / 5);
        match direction {
            ScrollDirection::Up => {
                let current = self.table_state.selected().unwrap_or(0);
                let new_pos = current.saturating_sub(jump);
                self.table_state.select(Some(new_pos));
            }
            ScrollDirection::Down => {
                let current = self.table_state.selected().unwrap_or(0);
                let new_pos = (current + jump).min(self.filtered.len().saturating_sub(1));
                self.table_state.select(Some(new_pos));
            }
        }
    }

    /// Handle a mouse click action.
    pub fn on_mouse_click(&mut self, action: &ClickAction) {
        match action {
            ClickAction::ScrollUp => self.on_scroll_up(),
            ClickAction::ScrollDown => self.on_scroll_down(),
            ClickAction::TableRow(index) => self.on_table_click(*index),
            ClickAction::HeaderTab(tab) => self.on_header_tab_click(*tab),
            ClickAction::ScrollbarTrack(direction) => self.on_scrollbar_track_click(*direction),
        }
    }
}
