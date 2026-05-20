// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # TUI Module
//!
//! Provides the full-screen interactive terminal interface for querying
//! DuckDuckGo from the command line and browsing web pages inline.
//!
//! The TUI is split across focused sub-modules:
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`types`] | Shared data structures and enumerations |
//! | [`emoji`] | Cross-platform emoji compatibility |
//! | [`settings`] | [`SettingsState`] mirroring all browser config fields |
//! | [`renderer`] | HTML-to-ratatui-Lines page renderer |
//! | [`app`] | [`App`] central state struct |
//! | [`events`] | Event loop and keyboard handler |
//! | [`ui`] | All frame-level render functions |
//!
//! ## Key Bindings
//!
//! ### Normal mode (result list)
//!
//! | Key | Action |
//! |-----|--------|
//! | `e` | Enter editing mode to type a search query |
//! | `Enter` | Execute search / open highlighted result as web page |
//! | `Backspace` | Go back to previous page (or close page view) |
//! | `Tab / Shift+Tab` | Switch between search tabs |
//! | `◄ ► / a d` | Switch between search tabs |
//! | `↑ ↓ / j k` | Scroll results |
//! | `PgUp / PgDn` | Scroll results faster |
//! | `q` | Quit the TUI |
//!
//! ### Page view mode
//!
//! | Key | Action |
//! |-----|--------|
//! | `j / k / ↑ ↓` | Scroll page / move link selection |
//! | `Enter` | Navigate to highlighted link |
//! | `Backspace / b` | Go back to previous page or search results |
//! | `f` | Go forward (redo navigation) |
//! | `Esc / q` | Close page, return to results |
//!
//! ## See Also
//!
//! - [DuckDuckGo Lite](https://lite.duckduckgo.com/lite/)
//! - [DuckDuckGo Images API](https://duckduckgo.com/i.js)
//! - [DuckDuckGo News API](https://duckduckgo.com/news.js)
//! - [DuckDuckGo Instant Answer API](https://duckduckgo.com/duckduckgo-help-pages/open-source/instant-answer-interface/)

pub mod app;
pub mod emoji;
pub mod events;
pub mod renderer;
pub mod settings;
pub mod types;
pub mod ui;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui_image::picker::Picker;
use std::io;

/// Initialises the terminal, runs the TUI event loop, and restores the
/// terminal on exit.
///
/// # Errors
///
/// Returns an [`anyhow::Error`] if terminal setup or any crossterm operation
/// fails.
pub async fn run_tui() -> Result<()> {
    enable_raw_mode()?;

    let image_picker = Picker::from_query_stdio().ok();

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = events::run_app(&mut terminal, image_picker).await;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = result {
        eprintln!("TUI error: {err:?}");
    }
    Ok(())
}
