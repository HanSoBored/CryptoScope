use crate::tui::app::AppState;
use crate::tui::mouse::{ClickAction, ClickRegions};
use crate::tui::theme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

fn loading_indicator(state: &AppState) -> Option<Span<'static>> {
    if state.loading || state.screener.loading {
        Some(Span::styled(
            "  Refreshing... ",
            theme::footer_item_style(),
        ))
    } else {
        None
    }
}

fn search_indicator_span(mode: bool, search: &str) -> Option<Span<'_>> {
    if mode {
        Some(Span::styled(format!("  /{} ", search), theme::footer_style()))
    } else {
        None
    }
}

fn search_indicator(state: &AppState) -> Option<Span<'_>> {
    match state.view {
        crate::tui::app::AppView::SymbolList => {
            search_indicator_span(state.symbol_list.search_mode, &state.symbol_list.search)
        }
        crate::tui::app::AppView::Screener => {
            search_indicator_span(state.screener.search_mode, &state.screener.search)
        }
        crate::tui::app::AppView::StatsDashboard => None,
    }
}

fn contract_filter_indicator(state: &AppState) -> Option<Span<'_>> {
    state
        .contract_types_labels(|ct| ct.abbreviation())
        .map(|labels| {
            Span::styled(
                format!(" Ⓚ {} ", labels.join(" ")),
                theme::footer_item_style(),
            )
        })
}

fn row_count_span(state: &AppState) -> Span<'_> {
    let count = match &state.view {
        crate::tui::app::AppView::Screener => state.screener.filtered_len(),
        _ => state.symbol_list.filtered_len(),
    };
    Span::styled(format!(" Rows: {count} "), theme::footer_item_style())
}

fn view_toggle_span(state: &AppState) -> Span<'static> {
    let text = match &state.view {
        crate::tui::app::AppView::Screener => " [Tab]List ",
        crate::tui::app::AppView::SymbolList => " [Tab]Stats ",
        crate::tui::app::AppView::StatsDashboard => " [Tab]Screener ",
    };
    Span::styled(text, theme::footer_item_style())
}

fn key_hints_span(state: &AppState) -> Span<'static> {
    let keys = match &state.view {
        crate::tui::app::AppView::Screener => {
            " [q/Esc]quit [j/k]nav [/]search [r]efresh [S]ort-dir [o]Sort "
        }
        _ => " [q/Esc]quit [j/k]nav [/]search [r]efresh [1-4]filter ",
    };
    Span::styled(keys, Style::default().fg(theme::DIM).bg(theme::BLACK))
}

fn build_footer_spans(state: &AppState) -> Vec<Span<'_>> {
    let mut spans = Vec::with_capacity(6);

    if let Some(span) = loading_indicator(state) {
        spans.push(span);
    }
    if let Some(span) = search_indicator(state) {
        spans.push(span);
    }
    if let Some(span) = contract_filter_indicator(state) {
        spans.push(span);
    }

    spans.push(row_count_span(state));
    spans.push(view_toggle_span(state));
    spans.push(key_hints_span(state));

    spans
}

pub fn render(frame: &mut Frame, area: Rect, state: &AppState, click_regions: &mut ClickRegions) {
    let spans = build_footer_spans(state);
    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(Style::default().bg(theme::BLACK));
    frame.render_widget(paragraph, area);

    add_footer_scroll_regions(click_regions, area);
}

fn add_footer_scroll_regions(click_regions: &mut ClickRegions, area: Rect) {
    let scroll_width = 3;
    let scroll_height = area.height;
    let x = area.x + area.width - scroll_width;

    // Top half = scroll up
    click_regions.add(
        Rect::new(x, area.y, scroll_width, scroll_height / 2),
        ClickAction::ScrollUp,
    );

    // Bottom half = scroll down
    let scroll_y = area.y + scroll_height / 2;
    let scroll_h = scroll_height - scroll_height / 2;
    click_regions.add(
        Rect::new(x, scroll_y, scroll_width, scroll_h),
        ClickAction::ScrollDown,
    );
}
