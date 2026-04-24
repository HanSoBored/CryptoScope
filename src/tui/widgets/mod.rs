pub mod footer;
pub mod header;
pub mod popup;
pub mod stats_panel;
pub mod symbol_table;

use crate::tui::app::AppState;
use crate::tui::mouse::ClickRegions;
use ratatui::Frame;

pub fn render(frame: &mut Frame, state: &mut AppState) -> ClickRegions {
    use ratatui::layout::{Constraint, Direction, Layout};

    let mut click_regions = ClickRegions::new();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(frame.area());

    header::render(frame, chunks[0], state, &mut click_regions);

    match &state.view {
        crate::tui::app::AppView::SymbolList => {
            symbol_table::render(frame, chunks[1], state, &mut click_regions);
        }
        crate::tui::app::AppView::StatsDashboard => {
            stats_panel::render(frame, chunks[1], state);
        }
    }

    footer::render(frame, chunks[2], state, &mut click_regions);

    if let Some(ref msg) = state.popup_message {
        popup::render_popup(frame, msg, state.popup_is_error);
    }

    click_regions
}
