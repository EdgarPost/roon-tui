# Roon TUI

A terminal user interface for [Roon](https://roon.app/) via the [roon-cli](https://github.com/EdgarPost/roon-cli) daemon.

Built with Rust and [Ratatui](https://ratatui.rs/), inspired by lazygit, k9s, and yazi.

## Features

- **Now Playing View** - Current track info, progress bar, playback controls
- **Browse View** - Navigate your Roon library (artists, albums, playlists)
- **Queue View** - View and manage the current playback queue
- **Zone Management** - Switch between zones, view zone grouping
- **Album Art** - Displays album art using Kitty graphics protocol (Ghostty, Kitty, iTerm2)
- **Real-time Updates** - Live position, state, and track updates via subscription

## Requirements

- [roon-cli](https://github.com/EdgarPost/roon-cli) daemon running
- Roon Core on your network
- Terminal with Kitty graphics protocol support (for album art)

## Installation

### Using Nix Flakes

```bash
# Run directly
nix run github:EdgarPost/roon-tui

# Or install
nix profile install github:EdgarPost/roon-tui
```

### From Source

```bash
cargo build --release
./target/release/roon-tui
```

## Usage

Start the roon-cli daemon first:
```bash
roon-daemon
```

Then run the TUI:
```bash
roon-tui
```

## Keyboard Shortcuts

### Global
| Key | Action |
|-----|--------|
| `q` | Quit |
| `?` | Show help |
| `Tab` | Next view |
| `Shift+Tab` | Previous view |
| `Esc` | Back / Close popup |

### Playback
| Key | Action |
|-----|--------|
| `Space` | Play / Pause |
| `n` | Next track |
| `p` | Previous track |
| `+` / `-` | Volume up / down |
| `m` | Mute / Unmute |
| `s` | Toggle shuffle |
| `r` | Cycle loop mode |
| `R` | Toggle Roon Radio |

### Navigation
| Key | Action |
|-----|--------|
| `j` / `k` | Move down / up |
| `h` / `l` | Navigate back / select |
| `Enter` | Select / Confirm |
| `g` / `G` | Go to top / bottom |

### Zones
| Key | Action |
|-----|--------|
| `z` | Zone selector |
| `Ctrl+g` | Zone grouping |

### Search
| Key | Action |
|-----|--------|
| `/` | Search library |

## Development

```bash
# Enter development shell
nix develop

# Build
cargo build

# Run with debug logging
RUST_LOG=roon_tui=debug cargo run 2> debug.log
```

## Architecture

```
roon-tui (Rust) <--Unix Socket--> roon-daemon (Node.js) <--> Roon Core
```

The TUI communicates with the roon-cli daemon via Unix socket at `/tmp/roon-cli.sock` using JSON-RPC.

## License

MIT
