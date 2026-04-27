use crate::tui::app::{AppState, AppView};
use crate::tui::mouse::ClickRegions;
use crate::tui::theme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState, _click_regions: &mut ClickRegions) {
    let exchange_line = Line::from(vec![Span::styled(
        format!(" Exchange: {}", state.exchange_name.to_uppercase()),
        Style::default().fg(theme::TAG).add_modifier(Modifier::BOLD),
    )]);

    let category_line = Line::from(vec![Span::styled(
        format!("󰪩 Categories: {}", state.categories.join(", ")),
        Style::default().fg(theme::TAG).add_modifier(Modifier::BOLD),
    )]);

    // Show contract type filter if active
    let contract_line = state.contract_types_display(|ct| ct.display_name()).map(|types| {
        Line::from(vec![Span::styled(
            format!("└─ Contract Types: {types}"),
            Style::default().fg(theme::TAG),
        )])
    });

    // Show screener count info when in Screener view
    let is_screener_view =
        state.view == AppView::Screener && state.screener.total_count > 0;
    let screener_line = is_screener_view.then(|| {
        let filtered = state.screener.filtered_len();
        let total = state.screener.total_count;
        let text = if filtered == total {
            format!(" Screener: {total} symbols")
        } else {
            format!(" Screener: {filtered}/{total} ({}/{} filtered)", total - filtered, total)
        };
        Line::from(vec![Span::styled(
            text,
            Style::default().fg(theme::TAG),
        )])
    });

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::LINE))
        .title("  CryptoScope ")
        .title_style(
            Style::default()
                .fg(theme::BLUE)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(theme::BLACK));

    let lines: Vec<Line<'_>> = vec![exchange_line, category_line]
        .into_iter()
        .chain(contract_line)
        .chain(screener_line)
        .collect();

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
