use ratatui::{
    prelude::*,
    widgets::{Gauge, Paragraph},
};
use ratatui_image::StatefulImage;

use crate::app::App;

/// Draw the Now Playing view - centered layout
pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    // Calculate content height: art(20) + spacing(1) + title(1) + artist(1) + album(1) + spacing(1) + time(1) + progress(1) + status(1) + volume(1) = 29
    let content_height = 29u16;
    let content_width = 50u16;

    // Center vertically
    let vertical_padding = area.height.saturating_sub(content_height) / 2;
    let horizontal_padding = area.width.saturating_sub(content_width) / 2;

    let centered_area = Rect {
        x: area.x + horizontal_padding,
        y: area.y + vertical_padding,
        width: content_width.min(area.width),
        height: content_height.min(area.height),
    };

    // Layout: album art, track info, progress bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(20), // Album art (larger)
            Constraint::Length(1),  // Spacing
            Constraint::Length(1),  // Title
            Constraint::Length(1),  // Artist
            Constraint::Length(1),  // Album
            Constraint::Length(1),  // Spacing
            Constraint::Length(1),  // Time display
            Constraint::Length(1),  // Progress bar
            Constraint::Length(1),  // Playback status icons
            Constraint::Length(1),  // Volume display
            Constraint::Min(0),     // Remaining space
        ])
        .split(centered_area);

    // Album art
    draw_album_art(frame, chunks[0], app);

    // Track info
    let (title, artist, album) = app.track_info();

    // Title (bold, white)
    let title_text = Paragraph::new(title)
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    frame.render_widget(title_text, chunks[2]);

    // Artist (cyan)
    let artist_text = Paragraph::new(artist)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);
    frame.render_widget(artist_text, chunks[3]);

    // Album (gray)
    let album_text = Paragraph::new(album)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(album_text, chunks[4]);

    // Time display (above progress bar)
    let progress_display = app.progress_display();
    let time_text = Paragraph::new(progress_display)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(time_text, chunks[6]);

    // Progress bar (thin, no label)
    let progress = app.progress_ratio();
    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
        .ratio(progress)
        .label("")
        .use_unicode(true);
    frame.render_widget(gauge, chunks[7]);

    // Playback status icons (shuffle, loop, radio)
    let status_line = format!(
        "{} {} {} {}",
        app.playback_icon(),
        app.shuffle_icon(),
        app.loop_icon(),
        app.radio_icon()
    );
    let status_text = Paragraph::new(status_line)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(status_text, chunks[8]);

    // Volume display
    let volume_text = Paragraph::new(app.volume_display())
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(volume_text, chunks[9]);
}

/// Draw album art centered
fn draw_album_art(frame: &mut Frame, area: Rect, app: &mut App) {
    // Center the album art block
    let art_size = area.height.min(area.width);
    let art_x = area.x + (area.width.saturating_sub(art_size)) / 2;

    let art_area = Rect {
        x: art_x,
        y: area.y,
        width: art_size,
        height: area.height,
    };

    // Try to render album art if image and picker are available
    if let (Some(image), Some(picker)) = (&app.album_art, &mut app.image_picker) {
        let mut protocol = picker.new_resize_protocol(image.clone());
        let stateful_image = StatefulImage::new();
        frame.render_stateful_widget(stateful_image, art_area, &mut protocol);
        return;
    }

    // Show placeholder if no image
    let placeholder = Paragraph::new("♪ ♫ ♪")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(placeholder, art_area);
}
