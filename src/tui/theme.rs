use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders};

/// Cyberpunk-inspired color theme for the TUI application.
///
/// Provides a consistent set of colors and styled widget builders
/// used across all TUI components.
pub struct CyberdeckTheme;

impl CyberdeckTheme {
    pub const BLACK: Color = Color::Rgb(0, 0, 0);
    /// Dark background color used for header/footer backgrounds.
    pub const DARK_BG: Color = Color::Rgb(34, 37, 41);
    pub const LINE: Color = Color::Rgb(57, 62, 70);
    pub const TAG: Color = Color::Rgb(86, 130, 177);
    pub const WHITE: Color = Color::Rgb(200, 200, 200);
    pub const DIM: Color = Color::Rgb(100, 100, 120);
    pub const RED: Color = Color::Rgb(255, 80, 80);
    #[allow(dead_code)]
    pub const YELLOW: Color = Color::Rgb(255, 255, 100);
    pub const BLUE: Color = Color::Rgb(47, 47, 228);

    /// Create a styled block with the cyberdeck theme.
    ///
    /// Returns a rounded-bordered block with themed colors,
    /// suitable for panels, tables, and stat displays.
    pub fn themed_block(title: &str) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Self::LINE))
            .title(format!(" {title} "))
            .title_style(Style::default().fg(Self::BLUE).add_modifier(Modifier::BOLD))
            .style(Style::default().fg(Self::WHITE).bg(Self::BLACK))
    }

    pub fn footer_style() -> Style {
        Style::default()
            .fg(Self::BLACK)
            .bg(Self::LINE)
            .add_modifier(Modifier::BOLD)
    }

    /// Style for footer item spans (white text on dark background, bold).
    pub fn footer_item_style() -> Style {
        Style::default()
            .fg(Self::WHITE)
            .bg(Self::DARK_BG)
            .add_modifier(Modifier::BOLD)
    }
}
