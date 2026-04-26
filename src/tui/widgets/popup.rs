use ratatui::Frame;
use ratatui::layout::{Margin, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Clear, Paragraph, Wrap};

use crate::tui::theme;

const MIN_WIDTH: u16 = 24;
const MAX_WIDTH: u16 = 60;
const PADDING_X: u16 = 4;
const PADDING_Y: u16 = 2;

/// Render a popup message centered near the top-right of the screen.
///
/// # Arguments
/// * `frame` - The terminal frame to render into
/// * `message` - The message text to display
/// * `is_error` - If true, uses error styling (red); otherwise uses info styling (blue)
///
/// # Safety: u16 casts
/// Terminal dimensions are bounded by the OS and will never exceed u16::MAX.
/// Message lengths in practice are short status strings, so truncation is not a concern.
#[allow(clippy::cast_possible_truncation)]
pub fn render_popup(frame: &mut Frame, message: &str, is_error: bool) {
    let area = frame.area();

    let max_line_len = message.split('\n').map(str::len).max().unwrap_or(0);

    let width = (max_line_len as u16 + PADDING_X).clamp(MIN_WIDTH, MAX_WIDTH);
    let max_content_width = (width - PADDING_X) as usize;

    let wrapped_lines: usize = message
        .split('\n')
        .map(|line| (line.len() + max_content_width) / max_content_width)
        .sum();
    let line_count = wrapped_lines.max(1) as u16;

    let height = line_count + PADDING_Y;

    let popup_area = Rect {
        x: area.width.saturating_sub(width + 2),
        y: 6,
        width,
        height,
    };

    let (title, border_color, text_style) = if is_error {
        (
            " ERROR ",
            theme::RED,
            Style::default().fg(theme::RED).add_modifier(Modifier::BOLD),
        )
    } else {
        (" INFO ", theme::BLUE, Style::default().fg(theme::WHITE))
    };

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .border_style(Style::default().fg(border_color))
        .style(
            Style::default()
                .bg(theme::BLACK)
                .add_modifier(Modifier::BOLD),
        );

    let paragraph = Paragraph::new(Text::from(message))
        .style(text_style)
        .wrap(Wrap { trim: true });

    frame.render_widget(Clear, popup_area);
    frame.render_widget(block, popup_area);
    frame.render_widget(
        paragraph,
        popup_area.inner(Margin {
            vertical: 1,
            horizontal: 2,
        }),
    );
}
