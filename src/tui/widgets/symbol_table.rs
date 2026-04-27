use crate::tui::app::AppState;
use crate::tui::mouse::{ClickAction, ClickRegions};
use crate::tui::theme;
use crate::tui::widgets::table_common;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Cell, Row, Table};

fn add_click_regions(click_regions: &mut ClickRegions, content_area: Rect, state: &AppState) {
    table_common::add_table_click_regions(click_regions, content_area, |cr, ca| {
        let offset = state.symbol_list.table_state.offset();
        let filtered_len = state.symbol_list.filtered_len();
        table_common::add_row_click_regions(cr, ca, offset, filtered_len, |symbol_idx| {
            ClickAction::TableRow(symbol_idx)
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

    let widths = [20, 22, 18];
    let selected = state.symbol_list.table_state.selected();
    let filtered_len = state.symbol_list.filtered_len();
    let symbols = &state.symbol_list.symbols;
    let filtered_indices = &state.symbol_list.filtered_indices;

    let rows: Vec<Row> = (0..filtered_len)
        .filter_map(|i| filtered_indices.get(i).and_then(|&idx| symbols.get(idx)))
        .enumerate()
        .map(|(i, s)| {
            let cells = vec![
                Cell::from(s.symbol.as_str()),
                Cell::from(s.contract_type()),
                Cell::from(format!("{}/{}", s.base_coin(), s.quote_coin())),
            ];
            Row::new(cells).style(table_common::row_style(selected == Some(i)))
        })
        .collect();

    let header = Row::new(vec![
        Cell::from("Symbol"),
        Cell::from("Contract"),
        Cell::from("Base/Quote"),
    ])
    .style(table_common::header_style());

    let table = Table::new(rows, widths)
        .header(header)
        .block(theme::themed_block(" List Crypto"))
        .row_highlight_style(table_common::row_highlight_style())
        .highlight_symbol(table_common::HIGHLIGHT_SYMBOL);

    add_click_regions(click_regions, content_area, state);

    table_common::render_table_and_scrollbar(
        frame,
        content_area,
        scrollbar_area,
        table,
        &mut state.symbol_list.table_state,
        filtered_len,
    );
}
