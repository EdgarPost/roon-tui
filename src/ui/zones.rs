use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

/// Draw the zone selector popup
pub fn draw_selector(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default()
        .title(" Select Zone ")
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().bg(Color::Black));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if app.zones.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from("No zones available").centered(),
            Line::from("").centered(),
            Line::from("Check Roon Core connection").centered(),
        ])
        .style(Style::default().fg(Color::DarkGray));

        frame.render_widget(empty, inner);
    } else {
        let items: Vec<ListItem> = app
            .zones
            .iter()
            .enumerate()
            .map(|(i, zone)| {
                let is_current = i == app.selected_zone_index;
                let prefix = if is_current { "● " } else { "○ " };

                let status = if zone.is_playing() { "▶ " } else { "  " };

                let style = if i == app.zone_selector_index {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let content = Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(status, Style::default().fg(Color::Green)),
                    Span::styled(&zone.display_name, style),
                ]);

                ListItem::new(content)
            })
            .collect();

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("");

        let mut state = ListState::default();
        state.select(Some(app.zone_selector_index));

        frame.render_stateful_widget(list, inner, &mut state);
    }
}
