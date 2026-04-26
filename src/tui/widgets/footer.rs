use crate::tui::app::AppState;
use crate::tui::mouse::{ClickAction, ClickRegions};
use crate::tui::theme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

pub fn render(frame: &mut Frame, area: Rect, state: &AppState, click_regions: &mut ClickRegions) {
    let mut spans = vec![];

    if state.loading {
        spans.push(Span::styled(
            "  Refreshing... ",
            theme::footer_item_style(),
        ));
    }

    if state.search_mode {
        spans.push(Span::styled(
            format!("  /{} ", state.search),
            theme::footer_style(),
        ));
    }

    let count_text = format!(" Rows: {} ", state.filtered.len());
    spans.push(Span::styled(&count_text, theme::footer_item_style()));

    let view_text = match &state.view {
        crate::tui::app::AppView::SymbolList => " [Tab]Stats ",
        crate::tui::app::AppView::StatsDashboard => " [Tab]List ",
    };
    spans.push(Span::styled(view_text, theme::footer_item_style()));

    let keys = " [q/Esc]quit [j/k]nav [/]search [r]efresh ";
    spans.push(Span::styled(
        keys,
        Style::default().fg(theme::DIM).bg(theme::BLACK),
    ));

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(Style::default().bg(theme::BLACK));
    frame.render_widget(paragraph, area);

    // Add scroll click regions on right edge of footer
    // Stacked vertically: top half = scroll up, bottom half = scroll down
    let scroll_height = area.height;
    let scroll_width = 3;

    // Scroll up (top half of right edge)
    click_regions.add(
        Rect::new(
            area.x + area.width - scroll_width,
            area.y,
            scroll_width,
            scroll_height / 2,
        ),
        ClickAction::ScrollUp,
    );

    // Scroll down (bottom half of right edge)
    let scroll_y = area.y + scroll_height / 2;
    let scroll_h = scroll_height - scroll_height / 2;
    click_regions.add(
        Rect::new(
            area.x + area.width - scroll_width,
            scroll_y,
            scroll_width,
            scroll_h,
        ),
        ClickAction::ScrollDown,
    );
}
