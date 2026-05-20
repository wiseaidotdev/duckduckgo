// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Emoji Compatibility
//!
//! Provides cross-platform emoji rendering.

/// Returns `emoji` on non-Windows targets, or a short bracketed text label
/// (e.g. `"[duck]"`) on Windows where the codepoint may not render.
#[cfg(target_os = "windows")]
pub fn safe_str(emoji: &str) -> String {
    emoji
        .chars()
        .map(|c| {
            let s = c.to_string();
            emojis::get(&s)
                .and_then(|e| e.shortcodes().next())
                .map(|name| format!("[{name}]"))
                .unwrap_or(s)
        })
        .collect()
}

/// Returns the emoji string unchanged on non-Windows platforms.
#[cfg(not(target_os = "windows"))]
pub fn safe_str(emoji: &str) -> String {
    emoji.to_string()
}
