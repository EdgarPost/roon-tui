use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, Popup, View};

/// Action to perform based on input
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    PlayPause,
    ShowHelp,
    ShowZoneSelector,
    ClosePopup,
    SelectUp,
    SelectDown,
    SelectZone,
    // Playback controls
    NextTrack,
    PrevTrack,
    ToggleShuffle,
    CycleLoop,
    ToggleRadio,
    VolumeUp,
    VolumeDown,
    ToggleMute,
    // View switching
    SwitchToNowPlaying,
    SwitchToBrowse,
    SwitchToSearch,
    // Browse/search navigation
    BrowseSelect,
    BrowseBack,
    SearchChar(char),
    SearchBackspace,
    SearchSubmit,
    SearchActivate,
    None,
}

/// Handle key events and return the action to perform
pub fn handle_key(key: KeyEvent, app: &App) -> Action {
    // Handle popups first
    if let Some(popup) = &app.popup {
        return handle_popup_key(key, popup);
    }

    // Check for Ctrl+C to quit
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        if let KeyCode::Char('c') = key.code {
            return Action::Quit;
        }
    }

    // Dispatch by view
    match app.view {
        View::NowPlaying => handle_now_playing_key(key),
        View::Browse => handle_browse_key(key),
        View::Search => handle_search_key(key, app),
    }
}

/// Handle keys in Now Playing view
fn handle_now_playing_key(key: KeyEvent) -> Action {
    match key.code {
        // Global
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('?') => Action::ShowHelp,
        KeyCode::Char('z') => Action::ShowZoneSelector,
        // Playback
        KeyCode::Char(' ') => Action::PlayPause,
        KeyCode::Char('n') => Action::NextTrack,
        KeyCode::Char('p') => Action::PrevTrack,
        KeyCode::Char('s') => Action::ToggleShuffle,
        KeyCode::Char('l') => Action::CycleLoop,
        KeyCode::Char('r') => Action::ToggleRadio,
        KeyCode::Char('+') | KeyCode::Char('=') => Action::VolumeUp,
        KeyCode::Char('-') => Action::VolumeDown,
        KeyCode::Char('m') => Action::ToggleMute,
        // View switching
        KeyCode::Char('1') => Action::SwitchToNowPlaying,
        KeyCode::Char('2') => Action::SwitchToBrowse,
        KeyCode::Char('3') | KeyCode::Char('/') => Action::SwitchToSearch,
        _ => Action::None,
    }
}

/// Handle keys in Browse view
fn handle_browse_key(key: KeyEvent) -> Action {
    match key.code {
        // Global
        KeyCode::Char('q') => Action::Quit,
        KeyCode::Char('?') => Action::ShowHelp,
        KeyCode::Char('z') => Action::ShowZoneSelector,
        // Navigation
        KeyCode::Char('j') | KeyCode::Down => Action::SelectDown,
        KeyCode::Char('k') | KeyCode::Up => Action::SelectUp,
        KeyCode::Enter => Action::BrowseSelect,
        KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('h') => Action::BrowseBack,
        // Playback
        KeyCode::Char(' ') => Action::PlayPause,
        KeyCode::Char('n') => Action::NextTrack,
        KeyCode::Char('p') => Action::PrevTrack,
        KeyCode::Char('+') | KeyCode::Char('=') => Action::VolumeUp,
        KeyCode::Char('-') => Action::VolumeDown,
        KeyCode::Char('m') => Action::ToggleMute,
        // View switching
        KeyCode::Char('1') => Action::SwitchToNowPlaying,
        KeyCode::Char('2') => Action::SwitchToBrowse,
        KeyCode::Char('3') | KeyCode::Char('/') => Action::SwitchToSearch,
        _ => Action::None,
    }
}

/// Handle keys in Search view
fn handle_search_key(key: KeyEvent, app: &App) -> Action {
    if app.search.input_active {
        // Text input mode
        match key.code {
            KeyCode::Esc => Action::BrowseBack,
            KeyCode::Enter => Action::SearchSubmit,
            KeyCode::Backspace => Action::SearchBackspace,
            KeyCode::Char(c) => Action::SearchChar(c),
            _ => Action::None,
        }
    } else {
        // Result navigation mode
        match key.code {
            // Global
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Char('?') => Action::ShowHelp,
            KeyCode::Char('z') => Action::ShowZoneSelector,
            // Navigation
            KeyCode::Char('j') | KeyCode::Down => Action::SelectDown,
            KeyCode::Char('k') | KeyCode::Up => Action::SelectUp,
            KeyCode::Enter => Action::BrowseSelect,
            KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('h') => Action::BrowseBack,
            KeyCode::Char('/') => Action::SearchActivate,
            // Playback
            KeyCode::Char(' ') => Action::PlayPause,
            KeyCode::Char('n') => Action::NextTrack,
            KeyCode::Char('p') => Action::PrevTrack,
            KeyCode::Char('+') | KeyCode::Char('=') => Action::VolumeUp,
            KeyCode::Char('-') => Action::VolumeDown,
            KeyCode::Char('m') => Action::ToggleMute,
            // View switching
            KeyCode::Char('1') => Action::SwitchToNowPlaying,
            KeyCode::Char('2') => Action::SwitchToBrowse,
            KeyCode::Char('3') => Action::SwitchToSearch,
            _ => Action::None,
        }
    }
}

/// Handle keys when a popup is shown
fn handle_popup_key(key: KeyEvent, popup: &Popup) -> Action {
    match popup {
        Popup::Help => match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => Action::ClosePopup,
            _ => Action::None,
        },
        Popup::ZoneSelector => match key.code {
            KeyCode::Esc => Action::ClosePopup,
            KeyCode::Char('j') | KeyCode::Down => Action::SelectDown,
            KeyCode::Char('k') | KeyCode::Up => Action::SelectUp,
            KeyCode::Enter => Action::SelectZone,
            _ => Action::None,
        },
    }
}

/// Get help text for keybindings
pub fn help_text() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Navigation", ""),
        ("1", "Now Playing view"),
        ("2", "Browse library"),
        ("3 / /", "Search library"),
        ("z", "Select zone"),
        ("?", "Show / hide help"),
        ("q", "Quit"),
        ("", ""),
        ("Playback", ""),
        ("Space", "Play / Pause"),
        ("n", "Next track"),
        ("p", "Previous track"),
        ("s", "Toggle shuffle"),
        ("l", "Cycle loop mode"),
        ("r", "Toggle radio"),
        ("", ""),
        ("Volume", ""),
        ("+ / =", "Volume up"),
        ("-", "Volume down"),
        ("m", "Toggle mute"),
        ("", ""),
        ("Browse / Search", ""),
        ("j/k", "Navigate up / down"),
        ("Enter", "Select / drill in"),
        ("Esc/Bksp", "Go back"),
    ]
}
