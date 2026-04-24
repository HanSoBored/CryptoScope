use crate::tui::app::AppState;
use crate::tui::mouse::ClickRegions;
use crate::tui::theme::CyberdeckTheme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState, _click_regions: &mut ClickRegions) {
    let exchange_line = Line::from(vec![Span::styled(
        format!(" Exchange: {}", state.exchange_name.to_uppercase()),
        Style::default()
            .fg(CyberdeckTheme::TAG)
            .add_modifier(Modifier::BOLD),
    )]);

    let category_line = Line::from(vec![Span::styled(
        format!("󰪩 Categories: {}", state.categories.join(", ")),
        Style::default()
            .fg(CyberdeckTheme::TAG)
            .add_modifier(Modifier::BOLD),
    )]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(CyberdeckTheme::LINE))
        .title("  CryptoScope ")
        .title_style(
            Style::default()
                .fg(CyberdeckTheme::BLUE)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(CyberdeckTheme::BLACK));

    let lines = vec![exchange_line, category_line];

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, area);
}
