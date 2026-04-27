use crate::tui::app::AppState;
use crate::tui::theme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

/// Render the stats dashboard view.
///
/// Displays four panels: overview total, category breakdown, and contract breakdown.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    render_total_panel(frame, left_chunks[0], state);
    render_category_panel(frame, left_chunks[1], state);
    render_contract_panel(frame, right_chunks[0], state);
}

fn render_total_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let total = state.symbol_list.filtered_len();
    let loading_indicator = if state.loading { " (loading...)" } else { "" };

    let lines = vec![Line::from(vec![Span::styled(
        format!("Total: {total}{loading_indicator}"),
        Style::default()
            .fg(theme::WHITE)
            .add_modifier(Modifier::BOLD),
    )])];

    let paragraph = Paragraph::new(lines).block(theme::themed_block(" Overview"));
    frame.render_widget(paragraph, area);
}

fn render_category_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let items = collect_sorted_stats(
        state
            .stats
            .iter()
            .flat_map(|stats| stats.by_category.iter().map(|(k, v)| (k.clone(), *v))),
    );

    render_stat_panel("  By Category", &items, 12, frame, area);
}

fn render_contract_panel(frame: &mut Frame, area: Rect, state: &AppState) {
    let items = collect_sorted_stats(
        state
            .stats
            .iter()
            .flat_map(|stats| stats.by_contract_type.iter().map(|(k, v)| (k.clone(), *v))),
    );

    render_stat_panel("  By Contract", &items, 20, frame, area);
}

/// Collect and sort stat items by count descending.
fn collect_sorted_stats(items: impl Iterator<Item = (String, usize)>) -> Vec<(String, usize)> {
    let mut counts: Vec<_> = items.collect();
    counts.sort_by_key(|b| std::cmp::Reverse(b.1));
    counts
}

/// Generic helper to render a stat panel with labeled key-value pairs.
///
/// # Arguments
/// * `title` - Panel title displayed in the block border
/// * `items` - Sorted list of (label, count) pairs
/// * `label_width` - Minimum width for left-aligned labels
/// * `frame` - Terminal frame to render into
/// * `area` - Rectangular area for the panel
fn render_stat_panel(
    title: &str,
    items: &[(String, usize)],
    label_width: usize,
    frame: &mut Frame,
    area: Rect,
) {
    let lines: Vec<Line<'_>> = items
        .iter()
        .map(|(label, count)| {
            Line::from(vec![
                Span::styled(
                    format!("{label:<label_width$}"),
                    Style::default().fg(theme::WHITE),
                ),
                Span::styled(format!(" {count}"), Style::default().fg(theme::WHITE)),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(lines).block(theme::themed_block(title));
    frame.render_widget(paragraph, area);
}
