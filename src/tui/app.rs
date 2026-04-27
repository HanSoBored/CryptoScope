use crate::cli::ScreenerMode;
use crate::models::ContractType;
use crate::models::Statistics;
use crate::models::price::PriceChange;
use crate::models::symbol::Symbol;
use crate::tui::mouse::ClickAction;
use crate::tui::mouse_state::{MouseState, TableContext};
use crate::tui::popup_state::PopupState;
use ratatui::widgets::TableState;
use std::cmp::Ordering;

/// Check if a single symbol matches the search term (case-insensitive).
fn symbol_matches_search(symbol: &Symbol, search: &str) -> bool {
    symbol.symbol.to_lowercase().contains(&search.to_lowercase())
}

/// Direction for scroll/selection navigation.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Direction {
    Next,
    Previous,
}

/// Helper: adjust selection after filtering (clamp to valid range).
fn adjust_selection(state: &mut TableState, count: usize) {
    if state.selected().is_none_or(|s| s >= count) {
        state.select(if count == 0 { None } else { Some(0) });
    }
}

/// Helper: compare two PartialOrd values of the same type, defaulting to Equal.
fn partial_cmp_or<T: PartialOrd + ?Sized>(a: &T, b: &T) -> Ordering {
    a.partial_cmp(b).unwrap_or(Ordering::Equal)
}

/// Helper: generic filter + selection adjustment for zero-copy index filtering.
fn filter_indices<T>(
    items: &[T],
    table_state: &mut TableState,
    predicate: impl Fn(usize, &T) -> bool,
) -> Vec<usize> {
    let indices: Vec<usize> = items
        .iter()
        .enumerate()
        .filter(|(idx, item)| predicate(*idx, item))
        .map(|(idx, _)| idx)
        .collect();
    adjust_selection(table_state, indices.len());
    indices
}

/// Helper: apply sort direction (reverse if descending).
fn apply_sort_order(cmp: Ordering, desc: bool) -> Ordering {
    if desc { cmp.reverse() } else { cmp }
}

/// Filter predicate: passes the minimum absolute change threshold.
fn passes_min_change(pc: &PriceChange, min_change: f64) -> bool {
    min_change <= 0.0 || pc.change_percent.abs() >= min_change
}

/// Filter predicate: passes the search term (case-insensitive substring match).
fn passes_search(pc: &PriceChange, search: &str) -> bool {
    search.is_empty() || pc.symbol.to_lowercase().contains(&search.to_lowercase())
}

/// Filter predicate: passes the contract type filter.
///
/// Does exact `ContractType` matching — no category-level collapsing.
fn passes_contract_type(pc: &PriceChange, contract_types: &[ContractType]) -> bool {
    if contract_types.is_empty() {
        return true;
    }
    contract_types.contains(&pc.contract_type)
}

/// Represents the current view mode of the TUI application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    SymbolList,
    StatsDashboard,
    Screener,
}

/// Sort field for screener table
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScreenerSort {
    #[default]
    ChangePercent,
    Volume,
    Symbol,
}

/// State for the symbol list view.
#[derive(Debug, Default)]
pub struct SymbolListState {
    pub symbols: Vec<Symbol>,
    /// Indices into `symbols` — zero-copy filtering.
    pub filtered_indices: Vec<usize>,
    pub table_state: TableState,
    pub search: String,
    pub search_mode: bool,
}

impl SymbolListState {
    /// Get a filtered symbol by its index in the filtered list.
    #[allow(dead_code)]
    fn get_filtered(&self, index: usize) -> Option<&Symbol> {
        self.filtered_indices
            .get(index)
            .and_then(|&idx| self.symbols.get(idx))
    }

    /// Number of filtered symbols.
    pub fn filtered_len(&self) -> usize {
        self.filtered_indices.len()
    }
}

/// State for the screener view: holds price change results, filtering indices,
/// sort settings, and loading state.
#[derive(Debug)]
pub struct ScreenerState {
    pub results: Vec<PriceChange>,
    /// Indices into `results` — zero-copy filtering.
    pub filtered_indices: Vec<usize>,
    pub table_state: TableState,
    pub sort: ScreenerSort,
    pub sort_desc: bool,
    pub min_change: f64,
    pub generation: u64,
    pub loading: bool,
    /// View-specific search buffer (separate from symbol list search).
    pub search: String,
    /// Whether search mode is active in the screener view.
    pub search_mode: bool,
    /// Total number of price changes before filtering (for display).
    pub total_count: usize,
}

impl Default for ScreenerState {
    fn default() -> Self {
        Self {
            results: Vec::new(),
            filtered_indices: Vec::new(),
            table_state: TableState::default(),
            sort: ScreenerSort::ChangePercent,
            sort_desc: true,
            min_change: 0.0,
            generation: 0,
            loading: true,
            search: String::new(),
            search_mode: false,
            total_count: 0,
        }
    }
}

impl ScreenerState {
    /// Get a filtered price change by its index in the filtered list.
    #[allow(dead_code)]
    fn get_filtered(&self, index: usize) -> Option<&PriceChange> {
        self.filtered_indices
            .get(index)
            .and_then(|&idx| self.results.get(idx))
    }

    /// Number of filtered results.
    pub fn filtered_len(&self) -> usize {
        self.filtered_indices.len()
    }
}

/// Shared application state managed behind `Arc<RwLock<AppState>>`.
///
/// Coordinates between `SymbolListState` and `ScreenerState` sub-structs,
/// plus shared concerns: view mode, loading, exchange metadata, popups, filters.
#[derive(Debug)]
pub struct AppState {
    pub symbol_list: SymbolListState,
    pub screener: ScreenerState,
    pub view: AppView,
    pub loading: bool,
    pub stats: Option<Statistics>,
    pub exchange_name: String,
    pub categories: Vec<String>,
    /// Screener mode (Ticker or Kline) — passed from CLI and used for refresh.
    pub screener_mode: ScreenerMode,
    /// Selected contract types for filtering. Empty = all selected (no filter).
    pub contract_types: Vec<ContractType>,
    pub popup: PopupState,
}

impl AppState {
    // === Constructors ===

    /// Create a new `AppState` for the given exchange and categories.
    pub fn new(exchange_name: String, categories: Vec<String>) -> Self {
        Self {
            symbol_list: SymbolListState::default(),
            // Default to Screener: shows market-wide price changes immediately on launch,
            // which is the most useful view for quick market scanning.
            screener: ScreenerState::default(),
            view: AppView::Screener,
            loading: true,
            stats: None,
            exchange_name,
            categories,
            screener_mode: ScreenerMode::default(),
            contract_types: Vec::new(),
            popup: PopupState::default(),
        }
    }

    /// Create a new `AppState` with initial contract type filters and screener mode.
    pub fn new_with_contract_types(
        exchange_name: String,
        categories: Vec<String>,
        contract_types: Vec<ContractType>,
        mode: ScreenerMode,
    ) -> Self {
        let mut state = Self::new(exchange_name, categories);
        state.contract_types = contract_types;
        state.screener_mode = mode;
        state
    }

    // === Symbol List & Filtering ===

    /// Replace the symbol list, re-apply filters, and compute statistics.
    ///
    /// Also resets the loading flag and initializes table selection if needed.
    pub fn set_symbols(&mut self, symbols: Vec<Symbol>) {
        self.symbol_list.symbols = symbols;
        self.apply_filters();
        self.stats = Some(Statistics::from_symbols(
            self.symbol_list
                .filtered_indices
                .iter()
                .map(|&i| &self.symbol_list.symbols[i]),
        ));
        self.loading = false;
    }

    /// Re-apply the current search filter to the full symbol list.
    /// Uses index-based filtering (zero-copy) for performance.
    pub fn apply_filters(&mut self) {
        let search = &self.symbol_list.search;
        let contract_types = &self.contract_types;
        self.symbol_list.filtered_indices = filter_indices(
            &self.symbol_list.symbols,
            &mut self.symbol_list.table_state,
            |_, symbol| {
                (search.is_empty() || symbol_matches_search(symbol, search))
                    && (contract_types.is_empty()
                        || contract_types.contains(&symbol.contract_type_parsed()))
            },
        );
    }

    // === Navigation ===

    /// Toggle between views: Screener ↔ Symbol List ↔ Stats Dashboard
    pub fn toggle_view(&mut self) {
        self.view = match &self.view {
            AppView::Screener => AppView::SymbolList,
            AppView::SymbolList => AppView::StatsDashboard,
            AppView::StatsDashboard => AppView::Screener,
        };
    }

    /// Navigate directly to the Screener view.
    pub fn navigate_to_screener(&mut self) {
        self.view = AppView::Screener;
    }

    /// Navigate directly to the Symbol List view.
    pub fn navigate_to_symbol_list(&mut self) {
        self.view = AppView::SymbolList;
    }

    /// Apply filters for the current view (dispatches to the correct method).
    pub fn apply_current_filters(&mut self) {
        match self.view {
            AppView::Screener => self.apply_screener_filters(),
            _ => self.apply_filters(),
        }
    }

    // === Search ===

    /// Return mutable references to the active view's (search, search_mode) pair.
    fn active_search_mut(&mut self) -> Option<(&mut String, &mut bool)> {
        match self.view {
            AppView::SymbolList => Some((&mut self.symbol_list.search, &mut self.symbol_list.search_mode)),
            AppView::Screener => Some((&mut self.screener.search, &mut self.screener.search_mode)),
            AppView::StatsDashboard => None,
        }
    }

    /// Check if search mode is active for the current view.
    pub fn is_search_mode(&self) -> bool {
        match self.view {
            AppView::SymbolList => self.symbol_list.search_mode,
            AppView::Screener => self.screener.search_mode,
            AppView::StatsDashboard => false,
        }
    }

    /// Toggle search mode on/off. Clears the search buffer when entering.
    pub fn toggle_search_mode(&mut self) {
        if let Some((search, mode)) = self.active_search_mut() {
            if *mode {
                *mode = false;
            } else {
                *mode = true;
                search.clear();
            }
        }
        self.apply_current_filters();
    }

    /// Append a character to the search buffer and re-apply filters.
    pub fn add_search_char(&mut self, c: char) {
        if let Some((search, _)) = self.active_search_mut() {
            search.push(c);
        }
        self.apply_current_filters();
    }

    /// Remove the last character from the search buffer and re-apply filters.
    pub fn remove_search_char(&mut self) {
        if let Some((search, _)) = self.active_search_mut() {
            search.pop();
        }
        self.apply_current_filters();
    }

    // === Popup ===

    /// Show a popup message with the given error/info flag.
    pub fn show_popup(&mut self, message: String, is_error: bool) {
        self.popup.show(message, is_error);
    }

    /// Check if the popup auto-dismiss timer has elapsed.
    pub fn update_popup(&mut self) {
        self.popup.update();
    }

    /// Dismiss the current popup message immediately.
    pub fn dismiss_popup(&mut self) {
        self.popup.dismiss();
    }

    // === Mouse Handling ===

    /// Build table contexts for screener and symbol list views.
    fn build_table_contexts(&mut self) -> (TableContext<'_>, TableContext<'_>) {
        let screener_len = self.screener.filtered_len();
        let symbol_len = self.symbol_list.filtered_len();
        (
            TableContext {
                state: &mut self.screener.table_state,
                filtered_len: screener_len,
            },
            TableContext {
                state: &mut self.symbol_list.table_state,
                filtered_len: symbol_len,
            },
        )
    }

    /// Handle a mouse click action by delegating to MouseState.
    pub fn on_mouse_click(&mut self, action: &ClickAction) {
        let view = self.view;
        let (screener, symbol) = self.build_table_contexts();
        MouseState::on_click(action, view, screener, symbol);
    }

    /// Scroll in the appropriate view (Screener uses its own table state).
    pub(crate) fn scroll_view(&mut self, direction: Direction) {
        let view = self.view;
        let (screener, symbol) = self.build_table_contexts();
        MouseState::scroll_view(view, screener, symbol, direction);
    }

    // === Screener ===

    /// Set screener results and apply filters (only if generation matches).
    ///
    /// Returns `true` if results were applied, `false` if they were stale.
    pub fn set_screener_results(&mut self, results: Vec<PriceChange>, generation: u64) -> bool {
        if generation != self.screener.generation {
            return false;
        }
        self.screener.total_count = results.len();
        self.screener.results = results;
        self.apply_screener_filters();
        self.screener.loading = false;
        adjust_selection(&mut self.screener.table_state, self.screener.filtered_indices.len());
        true
    }

    /// Apply screener filters (min change, search, contract types)
    ///
    /// Uses index-based filtering (zero-copy) for performance.
    /// Indices are sorted in-place according to the current sort criteria.
    pub fn apply_screener_filters(&mut self) {
        let min_change = self.screener.min_change;
        let search = &self.screener.search;
        let contract_types = &self.contract_types;
        self.screener.filtered_indices = filter_indices(
            &self.screener.results,
            &mut self.screener.table_state,
            |_, pc| {
                passes_min_change(pc, min_change)
                    && passes_search(pc, search)
                    && passes_contract_type(pc, contract_types)
            },
        );
        self.sort_screener_indices();
    }

    /// Toggle screener sort field (cycles: ChangePercent → Volume → Symbol → ChangePercent)
    pub fn toggle_screener_sort(&mut self) {
        self.screener.sort = match self.screener.sort {
            ScreenerSort::ChangePercent => ScreenerSort::Volume,
            ScreenerSort::Volume => ScreenerSort::Symbol,
            ScreenerSort::Symbol => ScreenerSort::ChangePercent,
        };
        self.apply_screener_filters();
    }

    /// Toggle screener sort direction
    pub fn toggle_screener_sort_desc(&mut self) {
        self.screener.sort_desc = !self.screener.sort_desc;
        self.apply_screener_filters();
    }

    /// Sort filtered indices by the current sort criteria (reorders indices, not data).
    fn sort_screener_indices(&mut self) {
        let desc = self.screener.sort_desc;
        let results = &self.screener.results;
        self.screener.filtered_indices.sort_by(|&a, &b| {
            let pc_a = &results[a];
            let pc_b = &results[b];
            let cmp = match self.screener.sort {
                ScreenerSort::ChangePercent => {
                    partial_cmp_or(&pc_a.change_percent, &pc_b.change_percent)
                }
                ScreenerSort::Volume => {
                    let vol_a = pc_a.volume_usdt();
                    let vol_b = pc_b.volume_usdt();
                    partial_cmp_or(&vol_a, &vol_b)
                }
                ScreenerSort::Symbol => pc_a.symbol.cmp(&pc_b.symbol),
            };
            apply_sort_order(cmp, desc)
        });
    }

    // === Contract Type Filters ===

    /// Format contract types for display, or None if all/none selected.
    pub fn contract_types_display(&self, formatter: fn(&ContractType) -> &str) -> Option<String> {
        self.contract_types_labels(formatter).map(|labels| labels.join(", "))
    }

    /// Return contract type labels for custom formatting, or None if all/none selected.
    pub fn contract_types_labels(
        &self,
        formatter: fn(&ContractType) -> &str,
    ) -> Option<Vec<&str>> {
        if !self.is_contract_filter_active() {
            return None;
        }
        Some(self.contract_types.iter().map(formatter).collect())
    }

    /// Check if the contract type filter is effectively active (some but not all selected).
    fn is_contract_filter_active(&self) -> bool {
        let all_count = ContractType::all().len();
        !self.contract_types.is_empty() && self.contract_types.len() < all_count
    }

    /// Toggle a contract type in the filter.
    pub fn toggle_contract_type(&mut self, contract: ContractType) {
        if self.contract_types.contains(&contract) {
            self.contract_types.retain(|&c| c != contract);
        } else {
            self.contract_types.push(contract);
        }
        self.apply_current_filters();
    }

    /// Select all contract types (clear filter, show all).
    pub fn select_all_contract_types(&mut self) {
        self.contract_types.clear();
        self.apply_current_filters();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a `PriceChange` with custom values.
    fn make_price_change(
        symbol: &str,
        category: &str,
        contract_type: ContractType,
        change_percent: f64,
        volume_24h: f64,
    ) -> PriceChange {
        PriceChange {
            symbol: symbol.to_string(),
            category: category.to_string(),
            contract_type,
            open_price: 100.0,
            current_price: 100.0 * (1.0 + change_percent / 100.0),
            change_value: change_percent,
            change_percent,
            volume_24h,
        }
    }

    /// Helper: create an `AppState` with a standard 3-item screener test dataset.
    fn make_screener_test_state() -> AppState {
        let mut state = AppState::new("bybit".into(), vec![]);
        state.screener.results = vec![
            make_price_change("BTCUSDT", "linear", ContractType::LinearPerpetual, 1.0, 100.0),
            make_price_change("ETHUSDT", "linear", ContractType::LinearPerpetual, 5.0, 200.0),
            make_price_change("SOLUSDT", "linear", ContractType::LinearPerpetual, -2.0, 150.0),
        ];
        state
    }

    #[test]
    fn test_sort_change_percent_descending() {
        let mut state = make_screener_test_state();
        state.screener.sort = ScreenerSort::ChangePercent;
        state.screener.sort_desc = true;
        state.apply_screener_filters();

        let symbols: Vec<&str> = state
            .screener
            .filtered_indices
            .iter()
            .map(|&i| state.screener.results[i].symbol.as_str())
            .collect();
        assert_eq!(symbols, vec!["ETHUSDT", "BTCUSDT", "SOLUSDT"]);
    }

    #[test]
    fn test_sort_change_percent_ascending() {
        let mut state = make_screener_test_state();
        state.screener.sort = ScreenerSort::ChangePercent;
        state.screener.sort_desc = false;
        state.apply_screener_filters();

        let symbols: Vec<&str> = state
            .screener
            .filtered_indices
            .iter()
            .map(|&i| state.screener.results[i].symbol.as_str())
            .collect();
        assert_eq!(symbols, vec!["SOLUSDT", "BTCUSDT", "ETHUSDT"]);
    }

    #[test]
    fn test_sort_volume_descending() {
        let mut state = AppState::new("bybit".into(), vec![]);
        state.screener.results = vec![
            make_price_change("BTCUSDT", "linear", ContractType::LinearPerpetual, 1.0, 100.0),
            make_price_change("ETHUSDT", "linear", ContractType::LinearPerpetual, 5.0, 500.0),
            make_price_change("SOLUSDT", "linear", ContractType::LinearPerpetual, -2.0, 300.0),
        ];
        state.screener.sort = ScreenerSort::Volume;
        state.screener.sort_desc = true;
        state.apply_screener_filters();

        let symbols: Vec<&str> = state
            .screener
            .filtered_indices
            .iter()
            .map(|&i| state.screener.results[i].symbol.as_str())
            .collect();
        assert_eq!(symbols, vec!["ETHUSDT", "SOLUSDT", "BTCUSDT"]);
    }

    #[test]
    fn test_sort_symbol_ascending() {
        let mut state = AppState::new("bybit".into(), vec![]);
        state.screener.results = vec![
            make_price_change("SOLUSDT", "linear", ContractType::LinearPerpetual, 1.0, 100.0),
            make_price_change("BTCUSDT", "linear", ContractType::LinearPerpetual, 5.0, 200.0),
            make_price_change("ETHUSDT", "linear", ContractType::LinearPerpetual, -2.0, 150.0),
        ];
        state.screener.sort = ScreenerSort::Symbol;
        state.screener.sort_desc = false;
        state.apply_screener_filters();

        let symbols: Vec<&str> = state
            .screener
            .filtered_indices
            .iter()
            .map(|&i| state.screener.results[i].symbol.as_str())
            .collect();
        assert_eq!(symbols, vec!["BTCUSDT", "ETHUSDT", "SOLUSDT"]);
    }

    #[test]
    fn test_filter_by_min_change() {
        let mut state = AppState::new("bybit".into(), vec![]);
        state.screener.results = vec![
            make_price_change("BTCUSDT", "linear", ContractType::LinearPerpetual, 1.0, 100.0),
            make_price_change("ETHUSDT", "linear", ContractType::LinearPerpetual, 5.0, 200.0),
            make_price_change("SOLUSDT", "linear", ContractType::LinearPerpetual, 0.3, 150.0),
        ];
        state.screener.min_change = 2.0;
        state.apply_screener_filters();

        assert_eq!(state.screener.filtered_len(), 1);
        assert_eq!(
            state.screener.get_filtered(0).map(|pc| pc.symbol.as_str()),
            Some("ETHUSDT")
        );
    }

    #[test]
    fn test_filter_by_search() {
        let mut state = make_screener_test_state();
        state.screener.search = "eth".to_string();
        state.apply_screener_filters();

        assert_eq!(state.screener.filtered_len(), 1);
        assert_eq!(
            state.screener.get_filtered(0).map(|pc| pc.symbol.as_str()),
            Some("ETHUSDT")
        );
    }

    #[test]
    fn test_filter_by_contract_type_linear_perp_only() {
        let mut state = AppState::new("bybit".into(), vec![]);
        state.screener.results = vec![
            make_price_change("BTCUSDT", "linear", ContractType::LinearPerpetual, 1.0, 100.0),
            make_price_change("ETHUSD", "inverse", ContractType::InversePerpetual, 5.0, 200.0),
            make_price_change("SOLUSDT", "linear", ContractType::LinearPerpetual, -2.0, 150.0),
        ];
        state.contract_types = vec![ContractType::LinearPerpetual];
        state.apply_screener_filters();

        assert_eq!(state.screener.filtered_len(), 2);
        assert!(state
            .screener
            .filtered_indices
            .iter()
            .all(|&i| state.screener.results[i].contract_type == ContractType::LinearPerpetual));
    }

    #[test]
    fn test_selection_clamped_after_filter() {
        let mut state = make_screener_test_state();
        // Select the last item (index 2)
        state.screener.table_state.select(Some(2));
        // Filter to only 1 result
        state.screener.min_change = 4.0;
        state.apply_screener_filters();

        // Selection should be clamped to 0 (the only valid index)
        assert_eq!(state.screener.table_state.selected(), Some(0));
    }

    #[test]
    fn test_selection_none_when_all_filtered_out() {
        let mut state = AppState::new("bybit".into(), vec![]);
        state.screener.results = vec![
            make_price_change("BTCUSDT", "linear", ContractType::LinearPerpetual, 1.0, 100.0),
            make_price_change("ETHUSDT", "linear", ContractType::LinearPerpetual, 5.0, 200.0),
        ];
        state.screener.table_state.select(Some(0));
        // Filter out everything
        state.screener.min_change = 100.0;
        state.apply_screener_filters();

        assert_eq!(state.screener.filtered_len(), 0);
        assert_eq!(state.screener.table_state.selected(), None);
    }

    #[test]
    fn test_toggle_sort_cycles_through_fields() {
        let mut state = AppState::new("bybit".into(), vec![]);
        assert_eq!(state.screener.sort, ScreenerSort::ChangePercent);

        state.toggle_screener_sort();
        assert_eq!(state.screener.sort, ScreenerSort::Volume);

        state.toggle_screener_sort();
        assert_eq!(state.screener.sort, ScreenerSort::Symbol);

        state.toggle_screener_sort();
        assert_eq!(state.screener.sort, ScreenerSort::ChangePercent);
    }

    #[test]
    fn test_toggle_sort_desc_flips_boolean() {
        let mut state = AppState::new("bybit".into(), vec![]);
        // Default is true (descending)
        assert!(state.screener.sort_desc);

        state.toggle_screener_sort_desc();
        assert!(!state.screener.sort_desc);

        state.toggle_screener_sort_desc();
        assert!(state.screener.sort_desc);
    }
}
