use std::time::Instant;

use image::DynamicImage;
use ratatui_image::picker::Picker;

use crate::roon::{BrowseItem, PlaybackState, Zone};

/// Active view
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum View {
    #[default]
    NowPlaying,
    Browse,
    Search,
}

/// Popup overlay state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Popup {
    Help,
    ZoneSelector,
}

/// State for the library browse view
pub struct BrowseState {
    pub items: Vec<BrowseItem>,
    pub selected_index: usize,
    pub breadcrumbs: Vec<String>,
    pub loading: bool,
    pub error: Option<String>,
}

impl Default for BrowseState {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected_index: 0,
            breadcrumbs: vec!["Library".to_string()],
            loading: false,
            error: None,
        }
    }
}

impl BrowseState {
    pub fn reset(&mut self) {
        self.items.clear();
        self.selected_index = 0;
        self.breadcrumbs = vec!["Library".to_string()];
        self.loading = false;
        self.error = None;
    }
}

/// State for the search view
pub struct SearchState {
    pub query: String,
    pub input_active: bool,
    pub results: BrowseState,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            query: String::new(),
            input_active: true,
            results: BrowseState {
                breadcrumbs: vec!["Search".to_string()],
                ..Default::default()
            },
        }
    }
}

impl SearchState {
    pub fn reset(&mut self) {
        self.query.clear();
        self.input_active = true;
        self.results = BrowseState {
            breadcrumbs: vec!["Search".to_string()],
            ..Default::default()
        };
    }
}

/// Application state
pub struct App {
    /// Whether the app should quit
    pub should_quit: bool,

    /// Current active view
    pub view: View,

    /// Current popup overlay (if any)
    pub popup: Option<Popup>,

    /// Whether connected to roon CLI
    pub connected: bool,

    /// Error message if any
    pub error: Option<String>,

    // ========== Zones ==========
    /// All available zones
    pub zones: Vec<Zone>,

    /// Index of currently selected zone
    pub selected_zone_index: usize,

    /// Zone selector index (when popup is open)
    pub zone_selector_index: usize,

    // ========== Album Art ==========
    /// Current album art image (decoded)
    pub album_art: Option<DynamicImage>,

    /// Current album art URL (to avoid re-fetching)
    pub album_art_url: Option<String>,

    /// Image picker for protocol detection
    pub image_picker: Option<Picker>,

    // ========== Time Tracking ==========
    /// When zones were last refreshed (for interpolating progress)
    pub last_refresh: Instant,

    // ========== Browse & Search ==========
    pub browse: BrowseState,
    pub search: SearchState,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            view: View::default(),
            popup: None,
            connected: false,
            error: None,
            zones: Vec::new(),
            selected_zone_index: 0,
            zone_selector_index: 0,
            album_art: None,
            album_art_url: None,
            image_picker: Picker::from_query_stdio().ok(),
            last_refresh: Instant::now(),
            browse: BrowseState::default(),
            search: SearchState::default(),
        }
    }

    /// Mark that zones were just refreshed
    pub fn mark_refreshed(&mut self) {
        self.last_refresh = Instant::now();
    }

    /// Get interpolated seek position (for smooth progress bar)
    fn interpolated_seek(&self) -> f64 {
        if let Some(zone) = self.current_zone() {
            if let Some(np) = &zone.now_playing {
                let base_position = np.seek_position;
                // Only interpolate if playing
                if zone.state == "playing" {
                    let elapsed = self.last_refresh.elapsed().as_secs_f64();
                    return (base_position + elapsed).min(np.length);
                }
                return base_position;
            }
        }
        0.0
    }

    /// Get the currently selected zone
    pub fn current_zone(&self) -> Option<&Zone> {
        self.zones.get(self.selected_zone_index)
    }

    /// Get the current zone name for display
    pub fn current_zone_name(&self) -> &str {
        self.current_zone()
            .map(|z| z.display_name.as_str())
            .unwrap_or("No Zone")
    }

    /// Get the display name of the first output in the current zone
    pub fn first_output_name(&self) -> Option<String> {
        self.current_zone()
            .and_then(|z| z.outputs.first())
            .map(|o| o.display_name.clone())
    }

    /// Get current playback state
    pub fn playback_state(&self) -> PlaybackState {
        self.current_zone()
            .map(|z| PlaybackState::from(z.state.as_str()))
            .unwrap_or_default()
    }

    /// Get playback state icon
    pub fn playback_icon(&self) -> &'static str {
        match self.playback_state() {
            PlaybackState::Playing => "▶",
            PlaybackState::Paused => "⏸",
            PlaybackState::Stopped => "⏹",
            PlaybackState::Loading => "⏳",
        }
    }

    /// Get shuffle icon
    pub fn shuffle_icon(&self) -> &'static str {
        let shuffle = self
            .current_zone()
            .map(|z| z.settings.shuffle)
            .unwrap_or(false);
        if shuffle {
            "🔀"
        } else {
            "  "
        }
    }

    /// Get loop icon
    pub fn loop_icon(&self) -> &'static str {
        let loop_mode = self
            .current_zone()
            .map(|z| z.settings.loop_mode.as_str())
            .unwrap_or("disabled");
        match loop_mode {
            "loop" => "🔁",
            "loop_one" => "🔂",
            _ => "  ",
        }
    }

    /// Get radio icon
    pub fn radio_icon(&self) -> &'static str {
        let radio = self
            .current_zone()
            .map(|z| z.settings.auto_radio)
            .unwrap_or(false);
        if radio {
            "📻"
        } else {
            "  "
        }
    }

    /// Get volume display string
    pub fn volume_display(&self) -> String {
        if let Some(zone) = self.current_zone() {
            if let Some(output) = zone.outputs.first() {
                if let Some(vol) = &output.volume {
                    if vol.is_muted {
                        return "🔇 Muted".to_string();
                    } else {
                        return format!("🔊 {:.0}%", vol.value);
                    }
                }
            }
        }
        "🔊 --".to_string()
    }

    /// Get progress display (current position / duration) with interpolation
    pub fn progress_display(&self) -> String {
        if let Some(zone) = self.current_zone() {
            if let Some(np) = &zone.now_playing {
                let current = self.interpolated_seek();
                return format!(
                    "{} / {}",
                    format_duration(current),
                    format_duration(np.length)
                );
            }
        }
        "00:00 / 00:00".to_string()
    }

    /// Get progress ratio (0.0 to 1.0) with interpolation
    pub fn progress_ratio(&self) -> f64 {
        if let Some(zone) = self.current_zone() {
            if let Some(np) = &zone.now_playing {
                if np.length > 0.0 {
                    let current = self.interpolated_seek();
                    return (current / np.length).clamp(0.0, 1.0);
                }
            }
        }
        0.0
    }

    /// Get current track info
    pub fn track_info(&self) -> (&str, &str, &str) {
        if let Some(zone) = self.current_zone() {
            if let Some(np) = &zone.now_playing {
                return (&np.track, &np.artist, &np.album);
            }
        }
        ("No track playing", "", "")
    }

    /// Get current album art URL if changed
    pub fn album_art_url_if_changed(&self) -> Option<&str> {
        if let Some(zone) = self.current_zone() {
            if let Some(np) = &zone.now_playing {
                if let Some(url) = &np.album_art_url {
                    if self.album_art_url.as_deref() != Some(url) {
                        return Some(url);
                    }
                }
            }
        }
        None
    }

    /// Set album art from decoded image data
    pub fn set_album_art(&mut self, image: DynamicImage, url: String) {
        self.album_art = Some(image);
        self.album_art_url = Some(url);
    }

    /// Clear album art
    pub fn clear_album_art(&mut self) {
        self.album_art = None;
        self.album_art_url = None;
    }

    /// Show a popup
    pub fn show_popup(&mut self, popup: Popup) {
        if popup == Popup::ZoneSelector {
            self.zone_selector_index = self.selected_zone_index;
        }
        self.popup = Some(popup);
    }

    /// Close any open popup
    pub fn close_popup(&mut self) {
        self.popup = None;
    }

    /// Move selection up (dispatched by context)
    pub fn select_up(&mut self) {
        if self.popup == Some(Popup::ZoneSelector) {
            if self.zone_selector_index > 0 {
                self.zone_selector_index -= 1;
            }
        } else {
            match self.view {
                View::Browse => {
                    if self.browse.selected_index > 0 {
                        self.browse.selected_index -= 1;
                    }
                }
                View::Search => {
                    if self.search.results.selected_index > 0 {
                        self.search.results.selected_index -= 1;
                    }
                }
                View::NowPlaying => {}
            }
        }
    }

    /// Move selection down (dispatched by context)
    pub fn select_down(&mut self) {
        if self.popup == Some(Popup::ZoneSelector) {
            if self.zone_selector_index < self.zones.len().saturating_sub(1) {
                self.zone_selector_index += 1;
            }
        } else {
            match self.view {
                View::Browse => {
                    if self.browse.selected_index < self.browse.items.len().saturating_sub(1) {
                        self.browse.selected_index += 1;
                    }
                }
                View::Search => {
                    if self.search.results.selected_index
                        < self.search.results.items.len().saturating_sub(1)
                    {
                        self.search.results.selected_index += 1;
                    }
                }
                View::NowPlaying => {}
            }
        }
    }

    /// Select the currently highlighted zone
    pub fn select_zone(&mut self) {
        if self.zone_selector_index < self.zones.len() {
            self.selected_zone_index = self.zone_selector_index;
            self.clear_album_art(); // Force reload album art for new zone
            self.close_popup();
        }
    }

    /// Get the zone name at selector index (for setting in CLI)
    pub fn get_selected_zone_name(&self) -> Option<String> {
        self.zones
            .get(self.zone_selector_index)
            .map(|z| z.display_name.clone())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Format seconds as mm:ss
fn format_duration(secs: f64) -> String {
    let total_secs = secs as u64;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}", mins, secs)
}
