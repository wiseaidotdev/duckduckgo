// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Settings State
//!
//! Mirrors every configurable field from [`crate::browser::BrowserBuilder`] and
//! [`crate::params::SearchParams`] so the TUI Settings tab can surface them as
//! an interactive form.

use crate::params::{
    AddressBar, Favicons, Font, HeaderBehavior, PageNumbers, Placement, Region, SafeSearch, Size,
    Theme, Toggle, UnitsMeasure, VideoPlayback, Width,
};

/// Cycle to the next variant in a slice of values, wrapping around.
pub fn cycle_next<T: PartialEq + Clone>(variants: &[T], current: &T) -> T {
    variants
        .iter()
        .position(|v| v == current)
        .map(|i| variants[(i + 1) % variants.len()].clone())
        .unwrap_or_else(|| variants[0].clone())
}

/// Cycle to the previous variant in a slice of values, wrapping around.
pub fn cycle_prev<T: PartialEq + Clone>(variants: &[T], current: &T) -> T {
    variants
        .iter()
        .position(|v| v == current)
        .map(|i| {
            if i == 0 {
                variants[variants.len() - 1].clone()
            } else {
                variants[i - 1].clone()
            }
        })
        .unwrap_or_else(|| variants[0].clone())
}

/// All configurable search and browser settings exposed by the Settings tab.
#[derive(Debug, Clone)]
pub struct SettingsState {
    pub user_agent_name: String,
    pub cookie_store: bool,
    pub proxy: String,
    pub region: Region,
    pub safe_search: SafeSearch,
    pub open_instant_answers: Toggle,
    pub auto_load_images: Toggle,
    pub auto_load_results: Toggle,
    pub new_window: Toggle,
    pub favicons: Favicons,
    pub full_urls: Toggle,
    pub auto_suggest: Toggle,
    pub redirect: Toggle,
    pub https: Toggle,
    pub address_bar: AddressBar,
    pub video_playback: VideoPlayback,
    pub header_color: String,
    pub url_color: String,
    pub background_color: String,
    pub text_color: String,
    pub link_color: String,
    pub visited_link_color: String,
    pub theme: Theme,
    pub size: Size,
    pub width: Width,
    pub placement: Placement,
    pub link_font: Font,
    pub underline: Toggle,
    pub text_font: Font,
    pub header_behavior: HeaderBehavior,
    pub advertisements: Toggle,
    pub page_numbers: PageNumbers,
    pub units_measure: UnitsMeasure,
    pub max_results: usize,
    /// Index of the currently focused settings row.
    pub focus_idx: usize,
    /// `true` when a text-entry field is being edited.
    pub editing_text: bool,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            user_agent_name: String::from("firefox"),
            cookie_store: false,
            proxy: String::new(),
            region: Region::WtWt,
            safe_search: SafeSearch::Moderate,
            open_instant_answers: Toggle::On,
            auto_load_images: Toggle::On,
            auto_load_results: Toggle::On,
            new_window: Toggle::Off,
            favicons: Favicons::JustFavicons,
            full_urls: Toggle::Off,
            auto_suggest: Toggle::On,
            redirect: Toggle::On,
            https: Toggle::On,
            address_bar: AddressBar::Get,
            video_playback: VideoPlayback::AlwaysOnDdg,
            header_color: String::new(),
            url_color: String::new(),
            background_color: String::new(),
            text_color: String::new(),
            link_color: String::new(),
            visited_link_color: String::new(),
            theme: Theme::Default,
            size: Size::Medium,
            width: Width::Normal,
            placement: Placement::Middle,
            link_font: Font::ProximaNova,
            underline: Toggle::On,
            text_font: Font::ProximaNova,
            header_behavior: HeaderBehavior::OnScrolling,
            advertisements: Toggle::Off,
            page_numbers: PageNumbers::On,
            units_measure: UnitsMeasure::On,
            max_results: 20,
            focus_idx: 0,
            editing_text: false,
        }
    }
}

impl SettingsState {
    /// Returns the total number of editable settings rows.
    pub fn row_count() -> usize {
        34
    }

    /// Returns `(label, value_string, is_boolean, is_text_input)` for the given row index.
    pub fn row_info(&self, idx: usize) -> (&'static str, String, bool, bool) {
        match idx {
            0 => ("User Agent", self.user_agent_name.clone(), false, true),
            1 => ("Cookie Store", bool_label(self.cookie_store), true, false),
            2 => ("Proxy URL", self.proxy.clone(), false, true),
            3 => ("Region", self.region.as_str().to_string(), false, false),
            4 => (
                "Safe Search",
                format!("{:?}", self.safe_search),
                false,
                false,
            ),
            5 => (
                "Instant Answers",
                toggle_label(&self.open_instant_answers),
                false,
                false,
            ),
            6 => (
                "Auto Load Images",
                toggle_label(&self.auto_load_images),
                false,
                false,
            ),
            7 => (
                "Auto Load Results",
                toggle_label(&self.auto_load_results),
                false,
                false,
            ),
            8 => ("New Window", toggle_label(&self.new_window), false, false),
            9 => ("Favicons", format!("{:?}", self.favicons), false, false),
            10 => ("Full URLs", toggle_label(&self.full_urls), false, false),
            11 => (
                "Auto Suggest",
                toggle_label(&self.auto_suggest),
                false,
                false,
            ),
            12 => ("Redirect", toggle_label(&self.redirect), false, false),
            13 => ("HTTPS Only", toggle_label(&self.https), false, false),
            14 => (
                "Address Bar",
                format!("{:?}", self.address_bar),
                false,
                false,
            ),
            15 => (
                "Video Playback",
                format!("{:?}", self.video_playback),
                false,
                false,
            ),
            16 => ("Header Color", self.header_color.clone(), false, true),
            17 => ("URL Color", self.url_color.clone(), false, true),
            18 => (
                "Background Color",
                self.background_color.clone(),
                false,
                true,
            ),
            19 => ("Text Color", self.text_color.clone(), false, true),
            20 => ("Link Color", self.link_color.clone(), false, true),
            21 => (
                "Visited Link Color",
                self.visited_link_color.clone(),
                false,
                true,
            ),
            22 => ("Theme", self.theme.as_string(), false, false),
            23 => ("Size", format!("{:?}", self.size), false, false),
            24 => ("Width", format!("{:?}", self.width), false, false),
            25 => ("Placement", format!("{:?}", self.placement), false, false),
            26 => ("Link Font", self.link_font.as_string(), false, false),
            27 => ("Underline", toggle_label(&self.underline), false, false),
            28 => ("Text Font", self.text_font.as_string(), false, false),
            29 => (
                "Header Behavior",
                format!("{:?}", self.header_behavior),
                false,
                false,
            ),
            30 => (
                "Advertisements",
                toggle_label(&self.advertisements),
                false,
                false,
            ),
            31 => (
                "Page Numbers",
                format!("{:?}", self.page_numbers),
                false,
                false,
            ),
            32 => (
                "Units Measure",
                format!("{:?}", self.units_measure),
                false,
                false,
            ),
            33 => ("Max Results", self.max_results.to_string(), false, false),
            _ => ("", String::new(), false, false),
        }
    }

    /// Cycles the enum or toggle at the given row index forward one step.
    pub fn cycle_row_next(&mut self, idx: usize) {
        match idx {
            1 => self.cookie_store = !self.cookie_store,
            3 => {
                self.region = cycle_next(&all_regions(), &self.region);
            }
            4 => {
                self.safe_search = cycle_next(
                    &[SafeSearch::On, SafeSearch::Moderate, SafeSearch::Off],
                    &self.safe_search,
                );
            }
            5 => self.open_instant_answers = toggle_flip(&self.open_instant_answers),
            6 => self.auto_load_images = toggle_flip(&self.auto_load_images),
            7 => self.auto_load_results = toggle_flip(&self.auto_load_results),
            8 => self.new_window = toggle_flip(&self.new_window),
            9 => {
                self.favicons = cycle_next(
                    &[
                        Favicons::JustFavicons,
                        Favicons::JustWot,
                        Favicons::WotAndFavicons,
                        Favicons::Off,
                    ],
                    &self.favicons,
                );
            }
            10 => self.full_urls = toggle_flip(&self.full_urls),
            11 => self.auto_suggest = toggle_flip(&self.auto_suggest),
            12 => self.redirect = toggle_flip(&self.redirect),
            13 => self.https = toggle_flip(&self.https),
            14 => {
                self.address_bar =
                    cycle_next(&[AddressBar::Get, AddressBar::Post], &self.address_bar);
            }
            15 => {
                self.video_playback = cycle_next(
                    &[
                        VideoPlayback::AlwaysOnDdg,
                        VideoPlayback::ThirdPartySite,
                        VideoPlayback::Prompt,
                    ],
                    &self.video_playback,
                );
            }
            22 => {
                self.theme = cycle_next(
                    &[
                        Theme::Default,
                        Theme::Dark,
                        Theme::Contrast,
                        Theme::Retro,
                        Theme::Terminal,
                    ],
                    &self.theme,
                );
            }
            23 => {
                self.size = cycle_next(
                    &[
                        Size::Small,
                        Size::Medium,
                        Size::Large,
                        Size::Larger,
                        Size::Largest,
                    ],
                    &self.size,
                );
            }
            24 => {
                self.width =
                    cycle_next(&[Width::Normal, Width::Wide, Width::SuperWide], &self.width);
            }
            25 => {
                self.placement = cycle_next(&[Placement::Middle, Placement::Left], &self.placement);
            }
            26 => self.link_font = cycle_font_next(&self.link_font),
            27 => self.underline = toggle_flip(&self.underline),
            28 => self.text_font = cycle_font_next(&self.text_font),
            29 => {
                self.header_behavior = cycle_next(
                    &[
                        HeaderBehavior::OnFloating,
                        HeaderBehavior::OnScrolling,
                        HeaderBehavior::OffExceptInstant,
                        HeaderBehavior::Off,
                    ],
                    &self.header_behavior,
                );
            }
            30 => self.advertisements = toggle_flip(&self.advertisements),
            31 => {
                self.page_numbers = cycle_next(
                    &[PageNumbers::On, PageNumbers::OnNoNumbers, PageNumbers::Off],
                    &self.page_numbers,
                );
            }
            32 => {
                self.units_measure = cycle_next(
                    &[
                        UnitsMeasure::On,
                        UnitsMeasure::OnNoNumbers,
                        UnitsMeasure::Off,
                    ],
                    &self.units_measure,
                );
            }
            33 => {
                self.max_results = (self.max_results + 5).min(100);
            }
            _ => {}
        }
    }

    /// Cycles the enum or toggle at the given row index backward one step.
    pub fn cycle_row_prev(&mut self, idx: usize) {
        match idx {
            1 => self.cookie_store = !self.cookie_store,
            3 => {
                self.region = cycle_prev(&all_regions(), &self.region);
            }
            4 => {
                self.safe_search = cycle_prev(
                    &[SafeSearch::On, SafeSearch::Moderate, SafeSearch::Off],
                    &self.safe_search,
                );
            }
            5 => self.open_instant_answers = toggle_flip(&self.open_instant_answers),
            6 => self.auto_load_images = toggle_flip(&self.auto_load_images),
            7 => self.auto_load_results = toggle_flip(&self.auto_load_results),
            8 => self.new_window = toggle_flip(&self.new_window),
            9 => {
                self.favicons = cycle_prev(
                    &[
                        Favicons::JustFavicons,
                        Favicons::JustWot,
                        Favicons::WotAndFavicons,
                        Favicons::Off,
                    ],
                    &self.favicons,
                );
            }
            10 => self.full_urls = toggle_flip(&self.full_urls),
            11 => self.auto_suggest = toggle_flip(&self.auto_suggest),
            12 => self.redirect = toggle_flip(&self.redirect),
            13 => self.https = toggle_flip(&self.https),
            14 => {
                self.address_bar =
                    cycle_prev(&[AddressBar::Get, AddressBar::Post], &self.address_bar);
            }
            15 => {
                self.video_playback = cycle_prev(
                    &[
                        VideoPlayback::AlwaysOnDdg,
                        VideoPlayback::ThirdPartySite,
                        VideoPlayback::Prompt,
                    ],
                    &self.video_playback,
                );
            }
            22 => {
                self.theme = cycle_prev(
                    &[
                        Theme::Default,
                        Theme::Dark,
                        Theme::Contrast,
                        Theme::Retro,
                        Theme::Terminal,
                    ],
                    &self.theme,
                );
            }
            23 => {
                self.size = cycle_prev(
                    &[
                        Size::Small,
                        Size::Medium,
                        Size::Large,
                        Size::Larger,
                        Size::Largest,
                    ],
                    &self.size,
                );
            }
            24 => {
                self.width =
                    cycle_prev(&[Width::Normal, Width::Wide, Width::SuperWide], &self.width);
            }
            25 => {
                self.placement = cycle_prev(&[Placement::Middle, Placement::Left], &self.placement);
            }
            26 => self.link_font = cycle_font_prev(&self.link_font),
            27 => self.underline = toggle_flip(&self.underline),
            28 => self.text_font = cycle_font_prev(&self.text_font),
            29 => {
                self.header_behavior = cycle_prev(
                    &[
                        HeaderBehavior::OnFloating,
                        HeaderBehavior::OnScrolling,
                        HeaderBehavior::OffExceptInstant,
                        HeaderBehavior::Off,
                    ],
                    &self.header_behavior,
                );
            }
            30 => self.advertisements = toggle_flip(&self.advertisements),
            31 => {
                self.page_numbers = cycle_prev(
                    &[PageNumbers::On, PageNumbers::OnNoNumbers, PageNumbers::Off],
                    &self.page_numbers,
                );
            }
            32 => {
                self.units_measure = cycle_prev(
                    &[
                        UnitsMeasure::On,
                        UnitsMeasure::OnNoNumbers,
                        UnitsMeasure::Off,
                    ],
                    &self.units_measure,
                );
            }
            33 => {
                self.max_results = self.max_results.saturating_sub(5).max(5);
            }
            _ => {}
        }
    }

    /// Returns `true` if the given row is a free-text input field.
    pub fn is_text_row(idx: usize) -> bool {
        matches!(idx, 0 | 2 | 16 | 17 | 18 | 19 | 20 | 21)
    }

    /// Mutably borrows the text value for a text-input row.
    pub fn text_row_mut(&mut self, idx: usize) -> Option<&mut String> {
        match idx {
            0 => Some(&mut self.user_agent_name),
            2 => Some(&mut self.proxy),
            16 => Some(&mut self.header_color),
            17 => Some(&mut self.url_color),
            18 => Some(&mut self.background_color),
            19 => Some(&mut self.text_color),
            20 => Some(&mut self.link_color),
            21 => Some(&mut self.visited_link_color),
            _ => None,
        }
    }
}

fn bool_label(v: bool) -> String {
    if v {
        "[x] Enabled".to_string()
    } else {
        "[ ] Disabled".to_string()
    }
}

fn toggle_label(t: &Toggle) -> String {
    match t {
        Toggle::On => "[x] On".to_string(),
        Toggle::Off => "[ ] Off".to_string(),
    }
}

fn toggle_flip(t: &Toggle) -> Toggle {
    match t {
        Toggle::On => Toggle::Off,
        Toggle::Off => Toggle::On,
    }
}

fn cycle_font_next(f: &Font) -> Font {
    let fonts = all_fonts();
    cycle_next(&fonts, f)
}

fn cycle_font_prev(f: &Font) -> Font {
    let fonts = all_fonts();
    cycle_prev(&fonts, f)
}

fn all_fonts() -> Vec<Font> {
    vec![
        Font::Arial,
        Font::CenturyGothic,
        Font::Georgia,
        Font::Helvetica,
        Font::ProximaNova,
        Font::SansSerif,
        Font::SegoeUi,
        Font::Serif,
        Font::Tahoma,
        Font::Times,
        Font::TrebuchetMs,
        Font::Verdana,
    ]
}

fn all_regions() -> Vec<Region> {
    vec![
        Region::WtWt,
        Region::UsEn,
        Region::UkEn,
        Region::ArEs,
        Region::AuEn,
        Region::AtDe,
        Region::BeFr,
        Region::BeNl,
        Region::BrPt,
        Region::BgBg,
        Region::CaEn,
        Region::CaFr,
        Region::CtCa,
        Region::ClEs,
        Region::CnZh,
        Region::CoEs,
        Region::HrHr,
        Region::CzCs,
        Region::DkDa,
        Region::EeEt,
        Region::FiFi,
        Region::FrFr,
        Region::DeDe,
        Region::GrEl,
        Region::HkTzh,
        Region::HuHu,
        Region::InEn,
        Region::IdId,
        Region::IdEn,
        Region::IeEn,
        Region::IlHe,
        Region::ItIt,
        Region::JpJp,
        Region::KrKr,
        Region::LvLv,
        Region::LtLt,
        Region::XlEs,
        Region::MyMs,
        Region::MyEn,
        Region::MxEs,
        Region::NlNl,
        Region::NzEn,
        Region::NoNo,
        Region::PeEs,
        Region::PhEn,
        Region::PhTl,
        Region::PlPl,
        Region::PtPt,
        Region::RoRo,
        Region::RuRu,
        Region::SgEn,
        Region::SkSk,
        Region::SlSl,
        Region::ZaEn,
        Region::EsEs,
        Region::SeSv,
        Region::ChDe,
        Region::ChFr,
        Region::ChIt,
        Region::TwTzh,
        Region::ThTh,
        Region::TrTr,
        Region::UaUk,
        Region::UeEs,
        Region::VeEs,
        Region::VnVi,
        Region::XaAr,
        Region::XaEn,
    ]
}
