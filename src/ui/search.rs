use ratatui::{
    prelude::*,
    widgets::Paragraph,
};

use crate::app::SearchState;
use super::browse;

/// Draw the search view
pub fn draw(frame: &mut Frame, area: Rect, state: &SearchState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Search input
            Constraint::Length(1), // Spacing
            Constraint::Min(0),    // Results (reuses browse view)
        ])
        .split(area);

    // Search input
    let cursor = if state.input_active { "█" } else { "" };
    let input_text = format!("Search: {}{}", state.query, cursor);
    let input_style = if state.input_active {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::White)
    };
    let input = Paragraph::new(input_text).style(input_style);
    frame.render_widget(input, chunks[0]);

    // Results (reuse browse view drawing)
    if !state.results.items.is_empty() || state.results.breadcrumbs.len() > 1 {
        browse::draw(frame, chunks[2], &state.results);
    } else if !state.query.is_empty() && !state.input_active {
        let empty = Paragraph::new("No results found")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        frame.render_widget(empty, chunks[2]);
    } else {
        let hint = Paragraph::new("Type a search query and press Enter")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[2]);
    }
}
