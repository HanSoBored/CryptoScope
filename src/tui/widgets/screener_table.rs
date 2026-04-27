use crate::screener::output::{format_price, format_volume};
use crate::tui::app::AppState;
use crate::tui::mouse::{ClickAction, ClickRegions};
use crate::tui::theme;
use crate::tui::widgets::table_common;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Cell, Row, Table};

fn add_click_regions(click_regions: &mut ClickRegions, content_area: Rect, state: &AppState) {
    table_common::add_table_click_regions(click_regions, content_area, |cr, ca| {
        let offset = state.screener.table_state.offset();
        let filtered_len = state.screener.filtered_len();
        table_common::add_row_click_regions(cr, ca, offset, filtered_len, |symbol_idx| {
            ClickAction::ScreenerTableRow(symbol_idx)
        });
    });
}

pub fn render(
    frame: &mut Frame,
    area: Rect,
    state: &mut AppState,
    click_regions: &mut ClickRegions,
) {
    let (content_area, scrollbar_area) = table_common::split_table_area(area);

    let widths = [15, 12, 12, 12, 12, 15];

    // Extract needed state upfront to avoid borrow conflicts later.
    let selected = state.screener.table_state.selected();
    let filtered_len = state.screener.filtered_len();
    let results = &state.screener.results;
    let filtered_indices = &state.screener.filtered_indices;

    // Build rows directly from filtered references — no intermediate Vec of cloned tuples.
    // symbol/category use .as_str() (Cell accepts &str); formatted values are owned Strings.
    let rows: Vec<Row> = (0..filtered_len)
        .filter_map(|i| filtered_indices.get(i).and_then(|&idx| results.get(idx)))
        .enumerate()
        .map(|(i, pc)| {
            let cells = vec![
                Cell::from(pc.symbol.as_str()),
                Cell::from(pc.category.as_str()),
                Cell::from(format_price(pc.open_price)),
                Cell::from(format_price(pc.current_price)),
                Cell::from(pc.change_percent_formatted()).style(
                    ratatui::style::Style::default()
                        .fg(theme::change_color(pc.change_percent)),
                ),
                Cell::from(format_volume(pc.volume_usdt())),
            ];
            Row::new(cells).style(table_common::row_style(selected == Some(i)))
        })
        .collect();

    let header = Row::new(vec![
        Cell::from("Symbol"),
        Cell::from("Category"),
        Cell::from("Open"),
        Cell::from("Current"),
        Cell::from("Change%"),
        Cell::from("Volume"),
    ])
    .style(table_common::header_style());

    let table = Table::new(rows, widths)
        .header(header)
        .block(theme::themed_block(" Screener"))
        .row_highlight_style(table_common::row_highlight_style())
        .highlight_symbol(table_common::HIGHLIGHT_SYMBOL);

    add_click_regions(click_regions, content_area, state);

    table_common::render_table_and_scrollbar(
        frame,
        content_area,
        scrollbar_area,
        table,
        &mut state.screener.table_state,
        filtered_len,
    );
}

pub fn render_loading(frame: &mut Frame, area: Rect) {
    use ratatui::layout::Alignment;
    use ratatui::style::Style;
    use ratatui::widgets::Paragraph;

    let block = theme::themed_block(" Screener");
    let paragraph = Paragraph::new("Fetching screener data...")
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme::DIM));

    frame.render_widget(paragraph, area);
}
