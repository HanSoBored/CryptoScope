use crate::tui::app::AppState;
use crate::tui::mouse::{ClickAction, ClickRegions, ScrollDirection};
use crate::tui::theme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Cell, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table};

/// Header row height
const HEADER_HEIGHT: u16 = 1;
/// Top border thickness
const TOP_BORDER: u16 = 1;

/// Determine row style based on selection state.
fn row_style(is_selected: bool) -> Style {
    if is_selected {
        Style::default().fg(theme::TAG).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::WHITE)
    }
}

/// Add click regions for visible table rows.
fn add_row_click_regions(click_regions: &mut ClickRegions, content_area: Rect, state: &AppState) {
    let offset = state.table_state.offset();
    let visible_rows = content_area.height.saturating_sub(HEADER_HEIGHT) as usize;
    let content_start_y = content_area.y + TOP_BORDER;

    for (visible_idx, symbol_idx) in (offset..state.filtered.len())
        .take(visible_rows)
        .enumerate()
    {
        let row_y = content_start_y + HEADER_HEIGHT + (visible_idx as u16);
        if row_y < content_area.y + content_area.height {
            click_regions.add(
                Rect::new(content_area.x, row_y, content_area.width, 1),
                ClickAction::TableRow(symbol_idx),
            );
        }
    }
}

/// Add click regions for scrollbar track (upper half = page up, lower half = page down).
fn add_scrollbar_click_regions(click_regions: &mut ClickRegions, scrollbar_area: Rect) {
    if scrollbar_area.is_empty() || scrollbar_area.height <= 2 {
        return;
    }
    let track_start_y = scrollbar_area.y + 1;
    let track_height = scrollbar_area.height - 2;
    let upper_half = track_height / 2;
    if upper_half == 0 {
        return;
    }
    click_regions.add(
        Rect::new(scrollbar_area.x, track_start_y, 1, upper_half),
        ClickAction::ScrollbarTrack(ScrollDirection::Up),
    );
    let lower_start_y = track_start_y + upper_half;
    let lower_height = track_height - upper_half;
    if lower_height > 0 {
        click_regions.add(
            Rect::new(scrollbar_area.x, lower_start_y, 1, lower_height),
            ClickAction::ScrollbarTrack(ScrollDirection::Down),
        );
    }
}

/// Build the table header and data rows from the current app state.
fn build_table_rows(state: &AppState) -> (Row<'_>, Vec<Row<'_>>) {
    let header = Row::new(vec![
        Cell::from("Symbol"),
        Cell::from("Contract"),
        Cell::from("Base/Quote"),
    ])
    .style(
        Style::default()
            .fg(theme::DARK_BG)
            .add_modifier(Modifier::BOLD),
    );

    let selected = state.table_state.selected();

    let rows: Vec<Row> = state
        .filtered
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let base_quote = format!("{}/{}", s.base_coin(), s.quote_coin());
            let cells = vec![
                Cell::from(s.symbol.clone()),
                Cell::from(s.contract_type()),
                Cell::from(base_quote),
            ];
            Row::new(cells).style(row_style(selected == Some(i)))
        })
        .collect();

    (header, rows)
}

/// Add click regions for visible table rows and scrollbar track.
fn add_click_regions(click_regions: &mut ClickRegions, content_area: Rect, state: &AppState) {
    add_row_click_regions(click_regions, content_area, state);

    let scrollbar_x = content_area.x + content_area.width;
    let scrollbar_area = Rect::new(scrollbar_x, content_area.y, 1, content_area.height);

    add_scrollbar_click_regions(click_regions, scrollbar_area);
}

/// Render the scrollbar on the right side of the table.
fn render_scrollbar(frame: &mut Frame, scrollbar_area: Rect, state: &AppState) {
    if scrollbar_area.is_empty() {
        return;
    }

    let chrome = HEADER_HEIGHT + TOP_BORDER;
    let content_height = scrollbar_area.height.saturating_sub(chrome) as usize;

    let total_rows = state.filtered.len();
    let offset = state.table_state.offset();
    let max_position = total_rows.saturating_sub(content_height);
    let mut scrollbar_state = ScrollbarState::new(max_position).position(offset.min(max_position));

    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("▲"))
        .end_symbol(Some("▼"))
        .thumb_symbol("│")
        .track_symbol(Some("║"))
        .style(Style::default().fg(theme::TAG));

    frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
}

/// Render the symbol table widget with scrollbar.
///
/// Displays a scrollable table with symbol, contract type, and base/quote
/// columns. The currently selected row is highlighted.
/// A vertical scrollbar is rendered on the right side (1 char width).
pub fn render(
    frame: &mut Frame,
    area: Rect,
    state: &mut AppState,
    click_regions: &mut ClickRegions,
) {
    // Split area: content (width-1) + scrollbar (width=1)
    let [content_area, scrollbar_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .areas(area);

    let widths = [20, 22, 18];
    let (header, rows) = build_table_rows(state);

    let table = Table::new(rows, widths)
        .header(header)
        .block(theme::themed_block(" List Crypto "))
        .row_highlight_style(Style::default().fg(theme::TAG).add_modifier(Modifier::BOLD))
        .highlight_symbol("▸ ");

    add_click_regions(click_regions, content_area, state);

    // SAFETY: `render_stateful_widget` requires `&mut TableState`.
    // We have exclusive access via the `RwLock<AppState>` write guard in runner.rs.
    #[allow(clippy::mutable_key_type)]
    let mut ts = state.table_state.clone();
    frame.render_stateful_widget(table, content_area, &mut ts);

    // SYNC: Copy offset back to actual state after rendering
    // This preserves ratatui's automatic offset management for navigation
    *state.table_state.offset_mut() = ts.offset();

    render_scrollbar(frame, scrollbar_area, state);
}
