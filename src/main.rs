mod app;
mod input;
mod roon;
mod ui;

use std::fs::File;
use std::io;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use tokio::sync::mpsc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use app::{App, Popup, View};
use input::{handle_key, Action};

/// Message for album art loading
enum AlbumArtMsg {
    Loaded(image::DynamicImage, String),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging to file (TUI apps can't log to stdout/stderr)
    let log_file = File::create("/tmp/roon-tui.log").ok();
    if let Some(file) = log_file {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "roon_tui=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer().with_writer(file))
            .init();
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

    // Run app
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let mut last_poll = Instant::now();
    let poll_interval = Duration::from_secs(1);

    // Channel for album art loading
    let (art_tx, mut art_rx) = mpsc::channel::<AlbumArtMsg>(1);

    // Initial data fetch
    refresh_zones(app);

    loop {
        // Draw UI
        terminal.draw(|frame| ui::draw(frame, app))?;

        // Wait for events with timeout (this prevents CPU spinning)
        tokio::select! {
            // Check for keyboard input
            _ = tokio::task::spawn_blocking(|| event::poll(Duration::from_millis(100))) => {
                if event::poll(Duration::from_millis(0))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            let action = handle_key(key, app);
                            handle_action(action, app);
                        }
                    }
                }
            }

            // Check for loaded album art
            Some(msg) = art_rx.recv() => {
                match msg {
                    AlbumArtMsg::Loaded(image, url) => {
                        app.set_album_art(image, url);
                    }
                }
            }

            // Timeout for UI refresh (smooth progress bar)
            _ = tokio::time::sleep(Duration::from_millis(50)) => {}
        }

        // Periodically refresh zone data
        if last_poll.elapsed() >= poll_interval {
            refresh_zones(app);
            last_poll = Instant::now();
        }

        // Check if album art needs fetching
        if let Some(url) = app.album_art_url_if_changed() {
            let url = url.to_string();
            let tx = art_tx.clone();

            // Mark as loading by setting the URL
            app.album_art_url = Some(url.clone());

            // Spawn async task to fetch album art
            tokio::spawn(async move {
                tracing::debug!("Fetching album art: {}", url);
                match reqwest::get(&url).await {
                    Ok(response) => {
                        if let Ok(bytes) = response.bytes().await {
                            if let Ok(image) = image::load_from_memory(&bytes) {
                                let _ = tx.send(AlbumArtMsg::Loaded(image, url)).await;
                                tracing::debug!("Loaded album art");
                            } else {
                                tracing::warn!("Failed to decode album art image");
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to fetch album art: {}", e);
                    }
                }
            });
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

/// Refresh zone data from roon CLI
fn refresh_zones(app: &mut App) {
    match roon::get_zones() {
        Ok(zones) => {
            app.zones = zones;
            app.connected = true;
            app.error = None;
            app.mark_refreshed();
            tracing::debug!("Refreshed {} zones", app.zones.len());
        }
        Err(e) => {
            app.connected = false;
            app.error = Some(e.to_string());
            tracing::error!("Failed to get zones: {}", e);
        }
    }
}

fn handle_action(action: Action, app: &mut App) {
    match action {
        Action::Quit => app.should_quit = true,
        Action::ShowHelp => app.show_popup(Popup::Help),
        Action::ClosePopup => app.close_popup(),
        Action::PlayPause => {
            if let Err(e) = roon::playpause() {
                tracing::error!("Failed to toggle play/pause: {}", e);
            }
            refresh_zones(app);
        }
        Action::ShowZoneSelector => app.show_popup(Popup::ZoneSelector),
        Action::SelectUp => app.select_up(),
        Action::SelectDown => app.select_down(),
        Action::SelectZone => {
            if let Some(name) = app.get_selected_zone_name() {
                if let Err(e) = roon::set_zone(&name) {
                    tracing::error!("Failed to set zone: {}", e);
                }
            }
            app.select_zone();
            refresh_zones(app);
        }

        // ========== Playback Controls ==========
        Action::NextTrack => {
            if let Err(e) = roon::next() {
                tracing::error!("Failed to skip to next track: {}", e);
            }
            refresh_zones(app);
        }
        Action::PrevTrack => {
            if let Err(e) = roon::prev() {
                tracing::error!("Failed to skip to previous track: {}", e);
            }
            refresh_zones(app);
        }
        Action::ToggleShuffle => {
            let current = app
                .current_zone()
                .map(|z| z.settings.shuffle)
                .unwrap_or(false);
            if let Err(e) = roon::shuffle(!current) {
                tracing::error!("Failed to toggle shuffle: {}", e);
            }
            refresh_zones(app);
        }
        Action::CycleLoop => {
            let current = app
                .current_zone()
                .map(|z| z.settings.loop_mode.as_str())
                .unwrap_or("disabled");
            let next_mode = match current {
                "disabled" => "loop",
                "loop" => "loop_one",
                _ => "disabled",
            };
            if let Err(e) = roon::set_loop(next_mode) {
                tracing::error!("Failed to cycle loop mode: {}", e);
            }
            refresh_zones(app);
        }
        Action::ToggleRadio => {
            let current = app
                .current_zone()
                .map(|z| z.settings.auto_radio)
                .unwrap_or(false);
            if let Err(e) = roon::radio(!current) {
                tracing::error!("Failed to toggle radio: {}", e);
            }
            refresh_zones(app);
        }
        Action::VolumeUp => {
            if let Some(output) = app.first_output_name() {
                if let Err(e) = roon::volume(&output, "+5") {
                    tracing::error!("Failed to increase volume: {}", e);
                }
                refresh_zones(app);
            }
        }
        Action::VolumeDown => {
            if let Some(output) = app.first_output_name() {
                if let Err(e) = roon::volume(&output, "-5") {
                    tracing::error!("Failed to decrease volume: {}", e);
                }
                refresh_zones(app);
            }
        }
        Action::ToggleMute => {
            if let Some(zone) = app.current_zone() {
                if let Some(output) = zone.outputs.first() {
                    let is_muted = output.volume.as_ref().map(|v| v.is_muted).unwrap_or(false);
                    let name = output.display_name.clone();
                    let result = if is_muted {
                        roon::unmute(&name)
                    } else {
                        roon::mute(&name)
                    };
                    if let Err(e) = result {
                        tracing::error!("Failed to toggle mute: {}", e);
                    }
                    refresh_zones(app);
                }
            }
        }

        // ========== View Switching ==========
        Action::SwitchToNowPlaying => {
            app.view = View::NowPlaying;
        }
        Action::SwitchToBrowse => {
            app.view = View::Browse;
            app.browse.reset();
            app.browse.loading = true;
            match roon::browse() {
                Ok(result) => {
                    app.browse.items = result.items;
                    app.browse.selected_index = 0;
                    app.browse.breadcrumbs = vec!["Library".to_string()];
                    if let Some(title) = result.title {
                        app.browse.breadcrumbs = vec![title];
                    }
                    app.browse.loading = false;
                    app.browse.error = None;
                }
                Err(e) => {
                    app.browse.loading = false;
                    app.browse.error = Some(e.to_string());
                    tracing::error!("Failed to browse library: {}", e);
                }
            }
        }
        Action::SwitchToSearch => {
            app.view = View::Search;
            app.search.reset();
        }

        // ========== Browse/Search Navigation ==========
        Action::BrowseSelect => {
            let (index, is_search) = match app.view {
                View::Browse => (app.browse.selected_index, false),
                View::Search => (app.search.results.selected_index, true),
                _ => return,
            };

            match roon::select(index) {
                Ok(result) => {
                    if result.action.as_deref() == Some("message") {
                        // Play action executed - switch to Now Playing
                        app.view = View::NowPlaying;
                        refresh_zones(app);
                    } else {
                        let state = if is_search {
                            &mut app.search.results
                        } else {
                            &mut app.browse
                        };
                        // Push breadcrumb from the selected item title
                        if let Some(item) = state.items.get(index) {
                            state.breadcrumbs.push(item.title.clone());
                        }
                        // If the response has a title, use it as breadcrumb instead
                        if let Some(title) = &result.title {
                            let len = state.breadcrumbs.len();
                            if len > 0 {
                                state.breadcrumbs[len - 1] = title.clone();
                            }
                        }
                        state.items = result.items;
                        state.selected_index = 0;
                        state.error = None;
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to select item: {}", e);
                    let state = if is_search {
                        &mut app.search.results
                    } else {
                        &mut app.browse
                    };
                    state.error = Some(e.to_string());
                }
            }
        }
        Action::BrowseBack => {
            let (state, view) = match app.view {
                View::Browse => (&mut app.browse, View::Browse),
                View::Search => {
                    if app.search.input_active {
                        // Esc from empty search input -> back to Now Playing
                        app.view = View::NowPlaying;
                        return;
                    }
                    (&mut app.search.results, View::Search)
                }
                _ => return,
            };

            if state.breadcrumbs.len() <= 1 {
                // At root - switch back to Now Playing
                app.view = View::NowPlaying;
            } else {
                match roon::back() {
                    Ok(result) => {
                        state.breadcrumbs.pop();
                        state.items = result.items;
                        state.selected_index = 0;
                        state.error = None;
                    }
                    Err(e) => {
                        tracing::error!("Failed to go back: {}", e);
                        // If back fails, just go to Now Playing
                        app.view = View::NowPlaying;
                    }
                }
            }
            // Reassign view if we didn't switch away
            if app.view != View::NowPlaying {
                app.view = view;
            }
        }
        Action::SearchChar(c) => {
            app.search.query.push(c);
        }
        Action::SearchBackspace => {
            app.search.query.pop();
        }
        Action::SearchSubmit => {
            if !app.search.query.is_empty() {
                let query = app.search.query.clone();
                match roon::search(&query) {
                    Ok(result) => {
                        app.search.results.items = result.items;
                        app.search.results.selected_index = 0;
                        app.search.results.breadcrumbs = vec!["Search".to_string()];
                        if let Some(title) = result.title {
                            app.search.results.breadcrumbs = vec![title];
                        }
                        app.search.results.error = None;
                        app.search.input_active = false;
                    }
                    Err(e) => {
                        app.search.results.error = Some(e.to_string());
                        app.search.input_active = false;
                        tracing::error!("Failed to search: {}", e);
                    }
                }
            }
        }
        Action::SearchActivate => {
            app.search.input_active = true;
        }

        Action::None => {}
    }
}
