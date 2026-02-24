#[allow(dead_code)]
mod models;

use anyhow::Result;
use std::process::Command;

pub use models::{BrowseItem, BrowseResult, PlaybackState, Zone};

/// Execute a roon CLI command and return stdout
fn run_command(args: &[&str]) -> Result<String> {
    let output = Command::new("roon").args(args).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let msg = if !stderr.is_empty() {
            stderr.to_string()
        } else if !stdout.is_empty() {
            stdout.to_string()
        } else {
            format!("exit code {}", output.status)
        };
        anyhow::bail!("roon {:?} failed: {}", args, msg.trim())
    }
}

/// Get all zones with their current state
pub fn get_zones() -> Result<Vec<Zone>> {
    let output = run_command(&["zones", "--json"])?;
    let zones: Vec<Zone> = serde_json::from_str(&output)?;
    Ok(zones)
}

/// Set the active zone by name
pub fn set_zone(name: &str) -> Result<()> {
    run_command(&["zone", "set", name])?;
    Ok(())
}

/// Toggle play/pause
pub fn playpause() -> Result<()> {
    run_command(&["playpause"])?;
    Ok(())
}

/// Skip to next track
pub fn next() -> Result<()> {
    run_command(&["next"])?;
    Ok(())
}

/// Skip to previous track
pub fn prev() -> Result<()> {
    run_command(&["prev"])?;
    Ok(())
}

/// Set shuffle mode
pub fn shuffle(on: bool) -> Result<()> {
    let value = if on { "on" } else { "off" };
    run_command(&["shuffle", value])?;
    Ok(())
}

/// Set loop mode (disabled, loop, loop_one)
pub fn set_loop(mode: &str) -> Result<()> {
    run_command(&["loop", mode])?;
    Ok(())
}

/// Set radio mode
pub fn radio(on: bool) -> Result<()> {
    let value = if on { "on" } else { "off" };
    run_command(&["radio", value])?;
    Ok(())
}

/// Set volume for an output
pub fn volume(output: &str, value: &str) -> Result<()> {
    run_command(&["volume", value, "--output", output])?;
    Ok(())
}

/// Mute an output
pub fn mute(output: &str) -> Result<()> {
    run_command(&["mute", "--output", output])?;
    Ok(())
}

/// Unmute an output
pub fn unmute(output: &str) -> Result<()> {
    run_command(&["unmute", "--output", output])?;
    Ok(())
}

/// Browse the library (resets to root)
pub fn browse() -> Result<BrowseResult> {
    let output = run_command(&["browse", "--json"])?;
    let result: BrowseResult = serde_json::from_str(&output)?;
    Ok(result)
}

/// Search the library
pub fn search(query: &str) -> Result<BrowseResult> {
    let output = run_command(&["search", query, "--json"])?;
    let result: BrowseResult = serde_json::from_str(&output)?;
    Ok(result)
}

/// Select an item by index in the current browse context (0-based internally, 1-based for CLI)
pub fn select(index: usize) -> Result<BrowseResult> {
    let output = run_command(&["select", &(index + 1).to_string(), "--json"])?;
    let result: BrowseResult = serde_json::from_str(&output)?;
    Ok(result)
}

/// Go back one level in the browse context
pub fn back() -> Result<BrowseResult> {
    let output = run_command(&["back", "--json"])?;
    let result: BrowseResult = serde_json::from_str(&output)?;
    Ok(result)
}
