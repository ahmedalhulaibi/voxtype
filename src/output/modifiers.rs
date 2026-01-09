//! Modifier key release functionality
//!
//! Releases all modifier keys (Shift, Ctrl, Alt, Super) before typing output.
//! This prevents held modifiers from interfering with typed text when using
//! compositor keybindings with modifiers (e.g., SUPER+CTRL+X).

use std::process::Stdio;
use tokio::process::Command;

/// Release all modifier keys using the best available tool.
/// Tries wtype first (Wayland-native), falls back to ydotool.
pub async fn release_all_modifiers() -> Result<(), String> {
    // Try wtype first (preferred for Wayland)
    if release_modifiers_wtype().await.is_ok() {
        tracing::debug!("Released modifiers via wtype");
        return Ok(());
    }

    // Fall back to ydotool
    if release_modifiers_ydotool().await.is_ok() {
        tracing::debug!("Released modifiers via ydotool");
        return Ok(());
    }

    Err("No tool available to release modifiers (tried wtype, ydotool)".to_string())
}

/// Release modifiers using wtype
/// wtype -m releases a modifier: shift, ctrl, logo, alt, altgr
async fn release_modifiers_wtype() -> Result<(), String> {
    let output = Command::new("wtype")
        .args([
            "-m", "shift",
            "-m", "ctrl",
            "-m", "logo",
            "-m", "alt",
            "-m", "altgr",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("wtype failed: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("wtype error: {}", stderr))
    }
}

/// Release modifiers using ydotool
/// Key codes: keycode:0 sends key release
/// - 42 = KEY_LEFTSHIFT, 54 = KEY_RIGHTSHIFT
/// - 29 = KEY_LEFTCTRL, 97 = KEY_RIGHTCTRL
/// - 56 = KEY_LEFTALT, 100 = KEY_RIGHTALT
/// - 125 = KEY_LEFTMETA (Super), 126 = KEY_RIGHTMETA
async fn release_modifiers_ydotool() -> Result<(), String> {
    let output = Command::new("ydotool")
        .args([
            "key",
            "42:0",  // Left Shift up
            "54:0",  // Right Shift up
            "29:0",  // Left Ctrl up
            "97:0",  // Right Ctrl up
            "56:0",  // Left Alt up
            "100:0", // Right Alt up
            "125:0", // Left Super up
            "126:0", // Right Super up
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("ydotool failed: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("ydotool error: {}", stderr))
    }
}

#[cfg(test)]
mod tests {
    // Note: These tests require wtype or ydotool to be installed
    // and are more integration tests than unit tests
}
