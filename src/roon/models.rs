use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zone {
    pub zone_id: String,
    pub display_name: String,
    pub state: String,
    pub outputs: Vec<Output>,
    pub now_playing: Option<NowPlaying>,
    #[serde(default)]
    pub queue_items_remaining: u32,
    #[serde(default)]
    pub queue_time_remaining: u32,
    pub settings: ZoneSettings,
}

impl Zone {
    pub fn is_playing(&self) -> bool {
        self.state == "playing"
    }

    pub fn is_paused(&self) -> bool {
        self.state == "paused"
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub output_id: String,
    pub display_name: String,
    pub volume: Option<Volume>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    #[serde(default)]
    pub is_muted: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NowPlaying {
    pub artist: String,
    pub track: String,
    pub album: String,
    pub image_key: String,
    #[serde(default)]
    pub seek_position: f64,
    #[serde(default)]
    pub length: f64,
    pub album_art_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZoneSettings {
    #[serde(rename = "loop")]
    pub loop_mode: String,
    pub shuffle: bool,
    pub auto_radio: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlbumArt {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseItem {
    pub item_key: Option<String>,
    pub title: String,
    pub subtitle: Option<String>,
    pub image_key: Option<String>,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowseResult {
    pub action: Option<String>,
    #[serde(default)]
    pub items: Vec<BrowseItem>,
    pub title: Option<String>,
    pub level: Option<i32>,
    pub count: Option<usize>,
    pub message: Option<String>,
}

/// Playback state enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlaybackState {
    Playing,
    Paused,
    #[default]
    Stopped,
    Loading,
}

impl From<&str> for PlaybackState {
    fn from(s: &str) -> Self {
        match s {
            "playing" => PlaybackState::Playing,
            "paused" => PlaybackState::Paused,
            "loading" => PlaybackState::Loading,
            _ => PlaybackState::Stopped,
        }
    }
}
