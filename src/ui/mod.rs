mod browse;
mod help;
mod now_playing;
mod search;
mod zones;

use ratatui::{prelude::*, widgets::Paragraph};

use crate::app::{App, Popup, View};

/// Main draw function - renders the entire UI
pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Create main layout: tab bar + content area + status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Tab bar
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Status bar
        ])
        .split(area);

    // Draw tab bar
    draw_tab_bar(frame, chunks[0], app);

    // Draw content based on active view
    match app.view {
        View::NowPlaying => now_playing::draw(frame, chunks[1], app),
        View::Browse => browse::draw(frame, chunks[1], &app.browse),
        View::Search => search::draw(frame, chunks[1], &app.search),
    }

    // Draw status bar
    draw_status_bar(frame, chunks[2], app);

    // Draw popup if any
    if let Some(popup) = &app.popup {
        draw_popup(frame, area, popup, app);
    }
}

/// Draw the tab bar at the top
fn draw_tab_bar(frame: &mut Frame, area: Rect, app: &App) {
    let active_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let inactive_style = Style::default().fg(Color::DarkGray);

    let tabs = vec![
        ("1", "Now Playing", View::NowPlaying),
        ("2", "Browse", View::Browse),
        ("3", "Search", View::Search),
    ];

    let spans: Vec<Span> = tabs
        .iter()
        .flat_map(|(key, label, view)| {
            let style = if *view == app.view {
                active_style
            } else {
                inactive_style
            };
            vec![
                Span::styled(format!("[{}] ", key), style),
                Span::styled(format!("{}  ", label), style),
            ]
        })
        .collect();

    let tab_line = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(Color::Black));
    frame.render_widget(tab_line, area);
}

/// Draw the status bar at the bottom
fn draw_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    let connection_status = if app.connected {
        Span::styled("● Connected", Style::default().fg(Color::Green))
    } else {
        Span::styled("○ Disconnected", Style::default().fg(Color::Red))
    };

    let zone_name = Span::styled(
        format!(" │ Zone: {}", app.current_zone_name()),
        Style::default().fg(Color::Yellow),
    );

    let help_hint = Span::styled(
        " │ Press ? for help",
        Style::default().fg(Color::DarkGray),
    );

    let left = Line::from(vec![connection_status, zone_name]);
    let right = Line::from(vec![help_hint]);

    // Split status bar into left and right
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    frame.render_widget(
        Paragraph::new(left).style(Style::default().bg(Color::DarkGray).fg(Color::White)),
        chunks[0],
    );
    frame.render_widget(
        Paragraph::new(right)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White))
            .alignment(Alignment::Right),
        chunks[1],
    );
}

/// Draw a popup overlay
fn draw_popup(frame: &mut Frame, area: Rect, popup: &Popup, app: &App) {
    // Create centered popup area
    let popup_area = centered_rect(60, 60, area);

    // Clear the popup area
    frame.render_widget(ratatui::widgets::Clear, popup_area);

    match popup {
        Popup::Help => help::draw(frame, popup_area),
        Popup::ZoneSelector => zones::draw_selector(frame, popup_area, app),
    }
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
