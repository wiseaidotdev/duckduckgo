// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Application State
//!
//! Defines [`App`], the central mutable state struct for the TUI event loop,
//! and its [`Default`] implementation.

use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;
use tui_input::Input;

use crate::browser::Browser;
use crate::tui::settings::SettingsState;
use crate::tui::types::{
    HistoryEntry, ImageResult, InputMode, InstantResult, NewsResult, RenderedPage, Tab, WebResult,
};
use crate::user_agents::get;

/// State for the full-screen image viewer (Images tab).
pub struct ImageViewerState {
    /// Display title from the search result.
    pub title: String,
    /// URL of the source image file.
    pub image_url: String,
    /// URL of the page that contains this image.
    pub page_url: String,
    /// Decoded, protocol-ready image state for `ratatui-image`.
    pub proto: StatefulProtocol,
}

/// A mutable state for the TUI event loop.
pub struct App {
    /// Keyboard input mode (Normal navigation vs. search-bar Editing).
    pub input_mode: InputMode,
    /// Currently visible/active tab.
    pub selected_tab: Tab,
    /// The search input widget state.
    pub search_input: Input,
    /// The HTTP browser used for all DuckDuckGo requests.
    pub browser: Browser,
    /// Web (Lite) search result list.
    pub web_results: Vec<WebResult>,
    /// Image search result list.
    pub image_results: Vec<ImageResult>,
    /// News search result list.
    pub news_results: Vec<NewsResult>,
    /// Instant Answer result, if any.
    pub instant_result: Option<InstantResult>,
    /// Index of the highlighted result in the current list view.
    pub selected_result: usize,
    /// Vertical scroll position for the result list.
    pub scroll: u16,
    /// Status-bar text shown in the footer.
    pub status: String,
    /// Rendered page currently being shown in the browser view, or `None`.
    pub page_view: Option<RenderedPage>,
    /// Backward navigation history stack (last entry = most recently visited).
    pub history: Vec<HistoryEntry>,
    /// Forward navigation stack populated when going back.
    pub history_forward: Vec<HistoryEntry>,
    /// Vertical scroll position inside the page view.
    pub page_scroll: u16,
    /// Index of the highlighted link inside the page view.
    pub page_selected_link: usize,
    /// All user-configurable settings exposed by the Settings tab.
    pub settings: SettingsState,
    /// Text being typed into a settings text field.
    pub settings_text_input: Input,
    /// Terminal image picker, initialised once and reused across frames.
    pub image_picker: Option<Picker>,
    /// Decoded image bytes for inline page images, keyed by URL.
    pub page_images: Vec<(String, StatefulProtocol)>,
    /// Full-screen image viewer activated from the Images tab.
    pub image_viewer: Option<ImageViewerState>,
    /// Set to `true` to break the event loop and restore the terminal cleanly.
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self::new(None)
    }
}

impl App {
    /// Constructs a new `App` with a pre-initialised image picker.
    pub fn new(image_picker: Option<Picker>) -> Self {
        Self {
            input_mode: InputMode::Normal,
            selected_tab: Tab::Web,
            search_input: Input::default(),
            browser: Browser::new(),
            web_results: vec![],
            image_results: vec![],
            news_results: vec![],
            instant_result: None,
            selected_result: 0,
            scroll: 0,
            status: String::from(
                "Ready  |  'e' to search  |  Tab: switch  |  Enter: open  |  Backspace: back  |  'q' quit",
            ),
            page_view: None,
            history: vec![],
            history_forward: vec![],
            page_scroll: 0,
            page_selected_link: 0,
            settings: SettingsState::default(),
            settings_text_input: Input::default(),
            image_picker,
            page_images: vec![],
            image_viewer: None,
            should_quit: false,
        }
    }

    /// Returns `true` if the browser page view is currently open.
    pub fn is_page_open(&self) -> bool {
        self.page_view.is_some()
    }

    /// Returns `true` if the full-screen image viewer is active.
    pub fn has_image_viewer(&self) -> bool {
        self.image_viewer.is_some()
    }

    /// Closes the image viewer and returns to the Images tab.
    pub fn close_image_viewer(&mut self) {
        self.image_viewer = None;
    }

    /// Closes the page view and returns to the result list.
    pub fn close_page(&mut self) {
        self.page_view = None;
        self.page_scroll = 0;
        self.page_selected_link = 0;
        self.page_images.clear();
    }

    /// Opens a new page, pushing the previous one onto the back-history stack.
    pub fn push_page(&mut self, page: RenderedPage) {
        if let Some(current) = &self.page_view {
            self.history.push(HistoryEntry {
                url: current.url.clone(),
            });
        }
        self.history_forward.clear();
        self.page_scroll = 0;
        self.page_selected_link = 0;
        self.page_images.clear();
        self.page_view = Some(page);
    }

    /// Pops the last history entry and returns its URL so the caller can fetch it.
    ///
    /// If there is no history, closes the page view (returns to results).
    pub fn go_back(&mut self) -> Option<String> {
        match self.history.pop() {
            Some(entry) => {
                if let Some(current) = &self.page_view {
                    self.history_forward.push(HistoryEntry {
                        url: current.url.clone(),
                    });
                }
                self.page_scroll = 0;
                self.page_selected_link = 0;
                self.page_images.clear();
                Some(entry.url)
            }
            None => {
                self.close_page();
                None
            }
        }
    }

    /// Pops the forward stack and returns the URL, enabling redo-navigation.
    pub fn go_forward(&mut self) -> Option<String> {
        self.history_forward.pop().map(|entry| {
            if let Some(current) = &self.page_view {
                self.history.push(HistoryEntry {
                    url: current.url.clone(),
                });
            }
            self.page_scroll = 0;
            self.page_selected_link = 0;
            self.page_images.clear();
            entry.url
        })
    }

    /// Returns the URL of the currently selected web result, if valid.
    pub fn selected_web_url(&self) -> Option<&str> {
        self.web_results
            .get(self.selected_result)
            .map(|r| r.url.as_str())
    }

    /// Returns the URL of the currently selected news result, if valid.
    pub fn selected_news_url(&self) -> Option<&str> {
        self.news_results
            .get(self.selected_result)
            .map(|r| r.url.as_str())
    }

    /// Returns the link text and URL of the page link at `page_selected_link`.
    pub fn selected_page_link(&self) -> Option<&crate::tui::types::PageLink> {
        self.page_view
            .as_ref()
            .and_then(|p| p.links.get(self.page_selected_link))
    }

    /// Clamps `selected_result` to the length of the current result list.
    pub fn clamp_selected_result(&mut self) {
        let max = match self.selected_tab {
            Tab::Web => self.web_results.len(),
            Tab::Images => self.image_results.len(),
            Tab::News => self.news_results.len(),
            Tab::Instant | Tab::Settings => 0,
        };
        if max == 0 {
            self.selected_result = 0;
        } else if self.selected_result >= max {
            self.selected_result = max - 1;
        }
    }

    /// Rebuilds `self.browser` from the current settings state.
    pub fn apply_settings(&mut self) -> anyhow::Result<()> {
        let mut builder = Browser::builder();
        let ua = get(&self.settings.user_agent_name).unwrap_or("");
        if !ua.is_empty() {
            builder = builder.user_agent(ua);
        }
        if self.settings.cookie_store {
            builder = builder.cookie_store(true);
        }
        if !self.settings.proxy.is_empty() {
            builder = builder.proxy(&self.settings.proxy);
        }
        self.browser = builder.build()?;
        Ok(())
    }
}
