use crate::tui::app::Direction;
use crate::tui::mouse::{ClickAction, ClickRegions, ScrollDirection};
use crate::tui::theme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState, TableState, Table};

/// Helper: scroll selection with wrap-around.
pub fn scroll_select(state: &mut TableState, count: usize, direction: Direction) {
    if count == 0 {
        state.select(None);
        return;
    }
    let current = state.selected().unwrap_or(0);
    let next = match direction {
        Direction::Next => {
            if current >= count - 1 {
                0
            } else {
                current + 1
            }
        }
        Direction::Previous => {
            if current == 0 {
                count - 1
            } else {
                current - 1
            }
        }
    };
    state.select(Some(next));
}

/// Header row height (shared across all table widgets).
pub(crate) const HEADER_HEIGHT: u16 = 1;

/// Top border thickness (shared across all table widgets).
pub(crate) const TOP_BORDER: u16 = 1;

/// Highlight symbol shown before the selected row.
pub const HIGHLIGHT_SYMBOL: &str = "▸ ";

/// Determine row style based on selection state.
pub fn row_style(is_selected: bool) -> Style {
    if is_selected {
        Style::default().fg(theme::TAG).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::WHITE)
    }
}

/// Style applied to the highlighted/selected row.
pub fn row_highlight_style() -> Style {
    row_style(true)
}

/// Add click regions for scrollbar track (upper half = page up, lower half = page down).
pub fn add_scrollbar_click_regions(click_regions: &mut ClickRegions, scrollbar_area: Rect) {
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

/// Render the scrollbar on the right side of the table.
///
/// # Arguments
/// * `frame` - The terminal frame to render on
/// * `scrollbar_area` - The area reserved for the scrollbar
/// * `total_rows` - Total number of data rows in the table
/// * `offset` - Current scroll offset (first visible row index)
pub fn render_scrollbar(
    frame: &mut Frame,
    scrollbar_area: Rect,
    total_rows: usize,
    offset: usize,
) {
    if scrollbar_area.is_empty() {
        return;
    }

    let chrome = HEADER_HEIGHT + TOP_BORDER;
    let content_height = scrollbar_area.height.saturating_sub(chrome) as usize;

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

/// Build the standard table header style (bold, dark background).
pub fn header_style() -> Style {
    Style::default()
        .fg(theme::DARK_BG)
        .add_modifier(Modifier::BOLD)
}

/// Add row click regions for any table, parameterized by offset, count, and action factory.
pub fn add_row_click_regions(
    click_regions: &mut ClickRegions,
    content_area: Rect,
    offset: usize,
    filtered_len: usize,
    make_action: impl Fn(usize) -> ClickAction,
) {
    let visible_rows = content_area.height.saturating_sub(2) as usize;
    let content_start_y = content_area.y.saturating_add(1);

    for (visible_idx, symbol_idx) in (offset..filtered_len)
        .take(visible_rows)
        .enumerate()
    {
        let row_y = content_start_y
            .saturating_add(1)
            .saturating_add(visible_idx as u16);

        if row_y < content_area.y.saturating_add(content_area.height) {
            click_regions.add(
                Rect::new(content_area.x, row_y, content_area.width, 1),
                make_action(symbol_idx),
            );
        }
    }
}

/// Add scrollbar click regions for a table given its content area.
///
/// Computes the scrollbar area (1 column to the right of content) and adds
/// both row click regions (via `add_row_regions`) and scrollbar track regions.
///
/// # Arguments
/// * `click_regions` — Mutable reference to accumulate click regions
/// * `content_area` — The rectangular area occupied by the table content
/// * `add_row_regions` — Closure that adds row-level click regions
pub fn add_table_click_regions<F>(
    click_regions: &mut ClickRegions,
    content_area: Rect,
    add_row_regions: F,
) where
    F: FnOnce(&mut ClickRegions, Rect),
{
    add_row_regions(click_regions, content_area);

    let scrollbar_x = content_area.x + content_area.width;
    let scrollbar_area = Rect::new(scrollbar_x, content_area.y, 1, content_area.height);
    add_scrollbar_click_regions(click_regions, scrollbar_area);
}

/// Split a table area into content and scrollbar regions.
pub fn split_table_area(area: Rect) -> (Rect, Rect) {
    let areas: [Rect; 2] = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Min(1),
            ratatui::layout::Constraint::Length(1),
        ])
        .areas(area);
    (areas[0], areas[1])
}

/// Shared table render pipeline: render table + scrollbar.
///
/// Caller is responsible for splitting the area and adding click regions
/// via `add_table_click_regions` before calling this.
///
/// # Arguments
/// * `frame` — Terminal frame to render on
/// * `content_area` — Area for the table content
/// * `scrollbar_area` — Area for the scrollbar
/// * `table` — Built Table widget (header, rows, block, styles already set)
/// * `table_state` — Mutable table state for selection/scroll
/// * `total_rows` — Total number of data rows (for scrollbar)
pub fn render_table_and_scrollbar(
    frame: &mut Frame,
    content_area: Rect,
    scrollbar_area: Rect,
    table: Table<'_>,
    table_state: &mut ratatui::widgets::TableState,
    total_rows: usize,
) {
    frame.render_stateful_widget(table, content_area, table_state);
    render_scrollbar(frame, scrollbar_area, total_rows, table_state.offset());
}
