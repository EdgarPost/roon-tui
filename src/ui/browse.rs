use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState, Paragraph},
};

use crate::app::BrowseState;

/// Draw the browse view
pub fn draw(frame: &mut Frame, area: Rect, state: &BrowseState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Breadcrumbs
            Constraint::Min(0),    // List
            Constraint::Length(1), // Hints
        ])
        .split(area);

    // Breadcrumbs
    let crumbs = state.breadcrumbs.join(" > ");
    let breadcrumb_line = Paragraph::new(crumbs)
        .style(Style::default().fg(Color::Yellow));
    frame.render_widget(breadcrumb_line, chunks[0]);

    // Error or loading state
    if let Some(err) = &state.error {
        let error = Paragraph::new(err.as_str())
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center);
        frame.render_widget(error, chunks[1]);
        return;
    }

    if state.loading {
        let loading = Paragraph::new("Loading...")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        frame.render_widget(loading, chunks[1]);
        return;
    }

    if state.items.is_empty() {
        let empty = Paragraph::new("No items")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        frame.render_widget(empty, chunks[1]);
    } else {
        // Item list
        let items: Vec<ListItem> = state
            .items
            .iter()
            .map(|item| {
                let indicator = match item.hint.as_deref() {
                    Some("list") => "> ",
                    Some("action_list") => "▶ ",
                    _ => "  ",
                };

                let mut spans = vec![
                    Span::styled(indicator, Style::default().fg(Color::DarkGray)),
                    Span::styled(&item.title, Style::default().fg(Color::White)),
                ];

                if let Some(subtitle) = &item.subtitle {
                    spans.push(Span::raw("  "));
                    spans.push(Span::styled(
                        subtitle.as_str(),
                        Style::default().fg(Color::DarkGray),
                    ));
                }

                ListItem::new(Line::from(spans))
            })
            .collect();

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▸ ");

        let mut list_state = ListState::default();
        list_state.select(Some(state.selected_index));

        frame.render_stateful_widget(list, chunks[1], &mut list_state);
    }

    // Hints
    let hints = Paragraph::new("j/k navigate  Enter select  Esc back")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(hints, chunks[2]);
}
