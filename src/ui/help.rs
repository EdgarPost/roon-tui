use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::input::help_text;

/// Draw the help popup
pub fn draw(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title(" Help - Keybindings ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().bg(Color::Black));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Build help text
    let bindings = help_text();
    let lines: Vec<Line> = bindings
        .iter()
        .map(|(key, desc)| {
            if desc.is_empty() {
                // Section header
                Line::from(Span::styled(
                    *key,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(vec![
                    Span::styled(
                        format!("{:12}", key),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(*desc, Style::default().fg(Color::White)),
                ])
            }
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .style(Style::default())
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, inner);

    // Close hint at bottom
    let close_hint = Line::from(Span::styled(
        "Press Esc or ? to close",
        Style::default().fg(Color::DarkGray),
    ));
    let hint_area = Rect {
        x: area.x + 2,
        y: area.y + area.height - 2,
        width: area.width - 4,
        height: 1,
    };
    frame.render_widget(
        Paragraph::new(close_hint).alignment(Alignment::Center),
        hint_area,
    );
}
