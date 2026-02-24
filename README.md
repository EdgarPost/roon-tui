# roon-tui

A terminal user interface for [Roon](https://roon.app/) built with Rust, [Ratatui](https://ratatui.rs/), and the [roon-cli](https://github.com/EdgarPost/roon-cli) daemon.

![Rust](https://img.shields.io/badge/rust-stable-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

- **Now Playing** — album art, track info, progress bar, playback state indicators, and volume display
- **Browse** — navigate your Roon library with breadcrumb trail and drill-down (artists, albums, playlists, genres, etc.)
- **Search** — search your library and drill into results to play
- **Playback Controls** — play/pause, next/prev, shuffle, loop, radio, volume, mute
- **Zone Management** — switch between Roon zones
- **Album Art** — inline album art via Kitty graphics protocol (Ghostty, Kitty, WezTerm, iTerm2)

## Requirements

- [roon-cli](https://github.com/EdgarPost/roon-cli) installed and configured
- Roon Core running on your network
- A terminal with Kitty graphics protocol support (for album art — optional)

## Installation

### Nix Flakes (recommended)

```bash
# Run directly
nix run github:EdgarPost/roon-tui

# Or install to your profile
nix profile install github:EdgarPost/roon-tui
```

The Nix flake automatically includes `roon-cli` on the PATH.

### From source

```bash
git clone https://github.com/EdgarPost/roon-tui.git
cd roon-tui
cargo build --release
./target/release/roon-tui
```

Make sure `roon` (from [roon-cli](https://github.com/EdgarPost/roon-cli)) is on your PATH.

## Usage

Start the roon-cli daemon first:

```bash
roon daemon
```

Then launch the TUI:

```bash
roon-tui
```

## Keybindings

### Navigation

| Key     | Action             |
|---------|--------------------|
| `1`     | Now Playing view   |
| `2`     | Browse library     |
| `3` `/` | Search library     |
| `z`     | Select zone        |
| `?`     | Show/hide help     |
| `q`     | Quit               |

### Playback

| Key     | Action             |
|---------|--------------------|
| `Space` | Play / Pause       |
| `n`     | Next track         |
| `p`     | Previous track     |
| `s`     | Toggle shuffle     |
| `l`     | Cycle loop mode    |
| `r`     | Toggle Roon Radio  |

### Volume

| Key     | Action             |
|---------|--------------------|
| `+` `=` | Volume up          |
| `-`     | Volume down        |
| `m`     | Toggle mute        |

### Browse / Search

| Key         | Action               |
|-------------|----------------------|
| `j` / `k`   | Navigate down / up   |
| `Enter`     | Select / drill in    |
| `Esc` `Bksp`| Go back              |
| `/`         | Activate search input|

## Architecture

```
roon-tui (Rust/Ratatui) ──CLI──> roon (Node.js) <──API──> Roon Core
```

`roon-tui` shells out to the `roon` CLI for all communication with Roon Core. The CLI handles authentication, transport subscriptions, and the browse/search API. The TUI polls zone state every second and fetches album art asynchronously over HTTP.

## Development

```bash
# Enter dev shell (includes Rust toolchain + roon-cli)
nix develop

# Build
cargo build

# Run with debug logging
RUST_LOG=roon_tui=debug cargo run
```

Logs are written to `/tmp/roon-tui.log`.

## License

MIT
