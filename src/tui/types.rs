// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # TUI Types
//!
//! Shared data structures and enumerations used throughout the TUI module.

use ratatui::text::Line;
use strum::IntoEnumIterator;
use strum_macros::{Display as DeriveDisplay, EnumIter as DeriveEnumIter};

/// Selectable tab within the TUI, corresponding to a DuckDuckGo search backend or UI section.
#[derive(Debug, Clone, DeriveEnumIter, DeriveDisplay, PartialEq)]
pub enum Tab {
    #[strum(to_string = "Web")]
    Web,
    #[strum(to_string = "Images")]
    Images,
    #[strum(to_string = "News")]
    News,
    #[strum(to_string = "Instant")]
    Instant,
    #[strum(to_string = "Settings")]
    Settings,
}

impl Tab {
    /// Returns the tab that follows this one, wrapping around.
    pub fn next(self) -> Self {
        let mut iter = Tab::iter().cycle();
        while let Some(tab) = iter.next() {
            if tab == self {
                return iter.next().unwrap_or(self);
            }
        }
        self
    }

    /// Returns the tab that precedes this one, wrapping around.
    pub fn previous(self) -> Self {
        let tabs: Vec<_> = Tab::iter().collect();
        let idx = tabs.iter().position(|t| *t == self).unwrap_or(0);
        if idx == 0 {
            tabs.last().cloned().unwrap_or(self)
        } else {
            tabs[idx - 1].clone()
        }
    }

    /// Returns the display emoji icon for the tab.
    pub fn icon(&self) -> &'static str {
        match self {
            Tab::Web => "Web",
            Tab::Images => "Images",
            Tab::News => "News",
            Tab::Instant => "Instant",
            Tab::Settings => "Settings",
        }
    }
}

/// Whether the terminal is in search-bar editing mode or normal navigation mode.
#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

/// A single web search result from the DuckDuckGo Lite endpoint.
#[derive(Debug, Clone)]
pub struct WebResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

/// A single image result from the DuckDuckGo Images endpoint.
#[derive(Debug, Clone)]
pub struct ImageResult {
    pub title: String,
    pub page_url: String,
    pub image_url: String,
    pub source: String,
}

/// A single news article result from the DuckDuckGo News endpoint.
#[derive(Debug, Clone)]
pub struct NewsResult {
    pub date: String,
    pub title: String,
    pub url: String,
    pub source: String,
    pub body: String,
}

/// Structured data from the DuckDuckGo Instant Answer API.
#[derive(Debug, Clone, Default)]
pub struct InstantResult {
    pub heading: String,
    pub abstract_text: String,
    pub abstract_source: String,
    pub abstract_url: String,
    pub answer: String,
    pub definition: String,
    pub entity: String,
    pub result_type: String,
}

/// A navigable hyperlink extracted from a rendered web page.
#[derive(Debug, Clone)]
pub struct PageLink {
    /// Human-readable anchor text.
    pub text: String,
    /// Absolute URL the link points to.
    pub url: String,
}

/// A fully rendered web page ready for display in the TUI.
#[derive(Debug, Clone)]
pub struct RenderedPage {
    /// The source URL of the page.
    pub url: String,
    /// Hostname/domain extracted from the URL (e.g. `"wiseai.dev"`).
    pub domain: String,
    /// Pre-styled text lines ready for `Paragraph::new`.
    pub lines: Vec<Line<'static>>,
    /// Ordered list of clickable links found in the page.
    pub links: Vec<PageLink>,
    /// Absolute image URLs found in the page for inline rendering.
    pub image_urls: Vec<String>,
}

/// An entry in the browser navigation history stack.
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// The URL that was visited.
    pub url: String,
}
