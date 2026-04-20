// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Search Parameters
//!
//! This module provides the [`SearchParams`] builder and all the typed enums
//! that map to DuckDuckGo URL query parameters (`kl`, `kp`, `kae`, etc.).
//!
//! ## Parameter Reference
//!
//! The table below lists every parameter supported by [`SearchParams`] and
//! the [`SearchParams`] setter that controls it.
//!
//! | DDG Param | Type | Setter |
//! |-----------|------|--------|
//! | `kl` | [`Region`] | [`SearchParams::region`] |
//! | `kp` | [`SafeSearch`] | [`SearchParams::safe_search`] |
//! | `kz` | [`Toggle`] | [`SearchParams::open_instant_answers`] |
//! | `kc` | [`Toggle`] | [`SearchParams::auto_load_images`] |
//! | `kav` | [`Toggle`] | [`SearchParams::auto_load_results`] |
//! | `kn` | [`Toggle`] | [`SearchParams::new_window`] |
//! | `kf` | [`Favicons`] | [`SearchParams::favicons`] |
//! | `kaf` | [`Toggle`] | [`SearchParams::full_urls`] |
//! | `kac` | [`Toggle`] | [`SearchParams::auto_suggest`] |
//! | `kd` | [`Toggle`] | [`SearchParams::redirect`] |
//! | `kh` | [`Toggle`] | [`SearchParams::https`] |
//! | `kg` | [`AddressBar`] | [`SearchParams::address_bar`] |
//! | `k5` | [`VideoPlayback`] | [`SearchParams::video_playback`] |
//! | `kj` | `String` | [`SearchParams::header_color`] |
//! | `kx` | `String` | [`SearchParams::url_color`] |
//! | `k7` | `String` | [`SearchParams::background_color`] |
//! | `k8` | `String` | [`SearchParams::text_color`] |
//! | `k9` | `String` | [`SearchParams::link_color`] |
//! | `kaa` | `String` | [`SearchParams::visited_link_color`] |
//! | `kae` | [`Theme`] | [`SearchParams::theme`] |
//! | `ks` | [`Size`] | [`SearchParams::size`] |
//! | `kw` | [`Width`] | [`SearchParams::width`] |
//! | `km` | [`Placement`] | [`SearchParams::placement`] |
//! | `ka` | [`Font`] | [`SearchParams::link_font`] |
//! | `ku` | [`Toggle`] | [`SearchParams::underline`] |
//! | `kt` | [`Font`] | [`SearchParams::text_font`] |
//! | `ko` | [`HeaderBehavior`] | [`SearchParams::header_behavior`] |
//! | `k1` | [`Toggle`] | [`SearchParams::advertisements`] |
//! | `kv` | [`PageNumbers`] | [`SearchParams::page_numbers`] |
//! | `kaj` | [`UnitsMeasure`] | [`SearchParams::units_measure`] |
//! | `t` | `String` | [`SearchParams::source`] |
//!
//! ## See Also
//!
//! - [DuckDuckGo URL Parameters](https://duckduckgo.com/duckduckgo-help-pages/settings/params/)
//! - [DuckDuckGo Settings UI](https://duckduckgo.com/settings)
//! - [DuckDuckGo Help - Search Syntax](https://help.duckduckgo.com/duckduckgo-help-pages/results/syntax/)

/// The region / language setting for DuckDuckGo searches (`kl` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Region {
    /// Arabia (`xa-ar`)
    XaAr,
    /// Arabia (en) (`xa-en`)
    XaEn,
    /// Argentina (`ar-es`)
    ArEs,
    /// Australia (`au-en`)
    AuEn,
    /// Austria (`at-de`)
    AtDe,
    /// Belgium (fr) (`be-fr`)
    BeFr,
    /// Belgium (nl) (`be-nl`)
    BeNl,
    /// Brazil (`br-pt`)
    BrPt,
    /// Bulgaria (`bg-bg`)
    BgBg,
    /// Canada (`ca-en`)
    CaEn,
    /// Canada (fr) (`ca-fr`)
    CaFr,
    /// Catalan (`ct-ca`)
    CtCa,
    /// Chile (`cl-es`)
    ClEs,
    /// China (`cn-zh`)
    CnZh,
    /// Colombia (`co-es`)
    CoEs,
    /// Croatia (`hr-hr`)
    HrHr,
    /// Czech Republic (`cz-cs`)
    CzCs,
    /// Denmark (`dk-da`)
    DkDa,
    /// Estonia (`ee-et`)
    EeEt,
    /// Finland (`fi-fi`)
    FiFi,
    /// France (`fr-fr`)
    FrFr,
    /// Germany (`de-de`)
    DeDe,
    /// Greece (`gr-el`)
    GrEl,
    /// Hong Kong (`hk-tzh`)
    HkTzh,
    /// Hungary (`hu-hu`)
    HuHu,
    /// India (`in-en`)
    InEn,
    /// Indonesia (`id-id`)
    IdId,
    /// Indonesia (en) (`id-en`)
    IdEn,
    /// Ireland (`ie-en`)
    IeEn,
    /// Israel (`il-he`)
    IlHe,
    /// Italy (`it-it`)
    ItIt,
    /// Japan (`jp-jp`)
    JpJp,
    /// Korea (`kr-kr`)
    KrKr,
    /// Latvia (`lv-lv`)
    LvLv,
    /// Lithuania (`lt-lt`)
    LtLt,
    /// Latin America (`xl-es`)
    XlEs,
    /// Malaysia (`my-ms`)
    MyMs,
    /// Malaysia (en) (`my-en`)
    MyEn,
    /// Mexico (`mx-es`)
    MxEs,
    /// Netherlands (`nl-nl`)
    NlNl,
    /// New Zealand (`nz-en`)
    NzEn,
    /// Norway (`no-no`)
    NoNo,
    /// Peru (`pe-es`)
    PeEs,
    /// Philippines (`ph-en`)
    PhEn,
    /// Philippines (tl) (`ph-tl`)
    PhTl,
    /// Poland (`pl-pl`)
    PlPl,
    /// Portugal (`pt-pt`)
    PtPt,
    /// Romania (`ro-ro`)
    RoRo,
    /// Russia (`ru-ru`)
    RuRu,
    /// Singapore (`sg-en`)
    SgEn,
    /// Slovak Republic (`sk-sk`)
    SkSk,
    /// Slovenia (`sl-sl`)
    SlSl,
    /// South Africa (`za-en`)
    ZaEn,
    /// Spain (`es-es`)
    EsEs,
    /// Sweden (`se-sv`)
    SeSv,
    /// Switzerland (de) (`ch-de`)
    ChDe,
    /// Switzerland (fr) (`ch-fr`)
    ChFr,
    /// Switzerland (it) (`ch-it`)
    ChIt,
    /// Taiwan (`tw-tzh`)
    TwTzh,
    /// Thailand (`th-th`)
    ThTh,
    /// Turkey (`tr-tr`)
    TrTr,
    /// Ukraine (`ua-uk`)
    UaUk,
    /// United Kingdom (`uk-en`)
    UkEn,
    /// United States (`us-en`)
    UsEn,
    /// United States (es) (`ue-es`)
    UeEs,
    /// Venezuela (`ve-es`)
    VeEs,
    /// Vietnam (`vn-vi`)
    VnVi,
    /// No region (`wt-wt`)
    WtWt,
}

impl Region {
    /// Returns the DDG region code string (value for the `kl` parameter).
    pub fn as_str(&self) -> &'static str {
        match self {
            Region::XaAr => "xa-ar",
            Region::XaEn => "xa-en",
            Region::ArEs => "ar-es",
            Region::AuEn => "au-en",
            Region::AtDe => "at-de",
            Region::BeFr => "be-fr",
            Region::BeNl => "be-nl",
            Region::BrPt => "br-pt",
            Region::BgBg => "bg-bg",
            Region::CaEn => "ca-en",
            Region::CaFr => "ca-fr",
            Region::CtCa => "ct-ca",
            Region::ClEs => "cl-es",
            Region::CnZh => "cn-zh",
            Region::CoEs => "co-es",
            Region::HrHr => "hr-hr",
            Region::CzCs => "cz-cs",
            Region::DkDa => "dk-da",
            Region::EeEt => "ee-et",
            Region::FiFi => "fi-fi",
            Region::FrFr => "fr-fr",
            Region::DeDe => "de-de",
            Region::GrEl => "gr-el",
            Region::HkTzh => "hk-tzh",
            Region::HuHu => "hu-hu",
            Region::InEn => "in-en",
            Region::IdId => "id-id",
            Region::IdEn => "id-en",
            Region::IeEn => "ie-en",
            Region::IlHe => "il-he",
            Region::ItIt => "it-it",
            Region::JpJp => "jp-jp",
            Region::KrKr => "kr-kr",
            Region::LvLv => "lv-lv",
            Region::LtLt => "lt-lt",
            Region::XlEs => "xl-es",
            Region::MyMs => "my-ms",
            Region::MyEn => "my-en",
            Region::MxEs => "mx-es",
            Region::NlNl => "nl-nl",
            Region::NzEn => "nz-en",
            Region::NoNo => "no-no",
            Region::PeEs => "pe-es",
            Region::PhEn => "ph-en",
            Region::PhTl => "ph-tl",
            Region::PlPl => "pl-pl",
            Region::PtPt => "pt-pt",
            Region::RoRo => "ro-ro",
            Region::RuRu => "ru-ru",
            Region::SgEn => "sg-en",
            Region::SkSk => "sk-sk",
            Region::SlSl => "sl-sl",
            Region::ZaEn => "za-en",
            Region::EsEs => "es-es",
            Region::SeSv => "se-sv",
            Region::ChDe => "ch-de",
            Region::ChFr => "ch-fr",
            Region::ChIt => "ch-it",
            Region::TwTzh => "tw-tzh",
            Region::ThTh => "th-th",
            Region::TrTr => "tr-tr",
            Region::UaUk => "ua-uk",
            Region::UkEn => "uk-en",
            Region::UsEn => "us-en",
            Region::UeEs => "ue-es",
            Region::VeEs => "ve-es",
            Region::VnVi => "vn-vi",
            Region::WtWt => "wt-wt",
        }
    }
}

/// Safe search setting (`kp` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafeSearch {
    /// Safe search on (`kp=1`)
    On,
    /// Safe search moderate (`kp=-1`)
    Moderate,
    /// Safe search off (`kp=-2`)
    Off,
}

impl SafeSearch {
    /// Returns the string value for the `kp` parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            SafeSearch::On => "1",
            SafeSearch::Moderate => "-1",
            SafeSearch::Off => "-2",
        }
    }
}

/// On/off toggle used by several DDG parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Toggle {
    /// On (`1`)
    On,
    /// Off (`-1`)
    Off,
}

impl Toggle {
    /// Returns `"1"` or `"-1"`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Toggle::On => "1",
            Toggle::Off => "-1",
        }
    }
}

/// Address bar submission method (`kg` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressBar {
    /// Use GET requests (`g`)
    Get,
    /// Use POST requests (`p`)
    Post,
}

impl AddressBar {
    /// Returns the string value for the `kg` parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            AddressBar::Get => "g",
            AddressBar::Post => "p",
        }
    }
}

/// Video playback preference (`k5` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoPlayback {
    /// Always play on DuckDuckGo (`1`)
    AlwaysOnDdg,
    /// Open on third-party site (`2`)
    ThirdPartySite,
    /// Prompt the user (`-1`)
    Prompt,
}

impl VideoPlayback {
    /// Returns the string value for the `k5` parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoPlayback::AlwaysOnDdg => "1",
            VideoPlayback::ThirdPartySite => "2",
            VideoPlayback::Prompt => "-1",
        }
    }
}

/// Favicons display setting (`kf` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Favicons {
    /// Show only favicons (`1`)
    JustFavicons,
    /// Show only WOT ratings (`w`)
    JustWot,
    /// Show both WOT and favicons (`fw`)
    WotAndFavicons,
    /// Off (`-1`)
    Off,
}

impl Favicons {
    /// Returns the string value for the `kf` parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            Favicons::JustFavicons => "1",
            Favicons::JustWot => "w",
            Favicons::WotAndFavicons => "fw",
            Favicons::Off => "-1",
        }
    }
}

/// Theme setting (`kae` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Theme {
    /// Default theme (`-1`)
    Default,
    /// Contrast theme (`c`)
    Contrast,
    /// Retro theme (`r`)
    Retro,
    /// Dark theme (`d`)
    Dark,
    /// Terminal theme (`t`)
    Terminal,
    /// Custom colour code (e.g. `"395323"`)
    Custom(String),
}

impl Theme {
    /// Returns the string value for the `kae` parameter.
    pub fn as_string(&self) -> String {
        match self {
            Theme::Default => "-1".to_string(),
            Theme::Contrast => "c".to_string(),
            Theme::Retro => "r".to_string(),
            Theme::Dark => "d".to_string(),
            Theme::Terminal => "t".to_string(),
            Theme::Custom(code) => code.clone(),
        }
    }
}

/// Result size setting (`ks` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Size {
    /// Large (`n`)
    Large,
    /// Larger (`l`)
    Larger,
    /// Largest (`t`)
    Largest,
    /// Medium (`m`)
    Medium,
    /// Small (`s`)
    Small,
}

impl Size {
    /// Returns the string value for the `ks` parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            Size::Large => "n",
            Size::Larger => "l",
            Size::Largest => "t",
            Size::Medium => "m",
            Size::Small => "s",
        }
    }
}

/// Page width setting (`kw` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Width {
    /// Normal (`n`)
    Normal,
    /// Wide (`w`)
    Wide,
    /// Super wide (`s`)
    SuperWide,
}

impl Width {
    /// Returns the string value for the `kw` parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            Width::Normal => "n",
            Width::Wide => "w",
            Width::SuperWide => "s",
        }
    }
}

/// Content placement setting (`km` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Placement {
    /// Middle (`m`)
    Middle,
    /// Left (`l`)
    Left,
}

impl Placement {
    /// Returns the string value for the `km` parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            Placement::Middle => "m",
            Placement::Left => "l",
        }
    }
}

/// Font selection for link and text fonts (`ka` / `kt` parameters).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Font {
    /// Arial (`a`)
    Arial,
    /// Century Gothic (`c`)
    CenturyGothic,
    /// Georgia (`g`)
    Georgia,
    /// Helvetica (`h`)
    Helvetica,
    /// Proxima Nova (`p`) - default
    ProximaNova,
    /// Sans-serif (`n`)
    SansSerif,
    /// Segoe UI (`e`)
    SegoeUi,
    /// Serif (`s`)
    Serif,
    /// Tahoma (`o`)
    Tahoma,
    /// Times (`t`)
    Times,
    /// Trebuchet MS (`b`)
    TrebuchetMs,
    /// Verdana (`v`)
    Verdana,
    /// Custom font name
    Custom(String),
}

impl Font {
    /// Returns the string value for the font parameter.
    pub fn as_string(&self) -> String {
        match self {
            Font::Arial => "a".to_string(),
            Font::CenturyGothic => "c".to_string(),
            Font::Georgia => "g".to_string(),
            Font::Helvetica => "h".to_string(),
            Font::ProximaNova => "p".to_string(),
            Font::SansSerif => "n".to_string(),
            Font::SegoeUi => "e".to_string(),
            Font::Serif => "s".to_string(),
            Font::Tahoma => "o".to_string(),
            Font::Times => "t".to_string(),
            Font::TrebuchetMs => "b".to_string(),
            Font::Verdana => "v".to_string(),
            Font::Custom(name) => name.clone(),
        }
    }
}

/// Header behaviour setting (`ko` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeaderBehavior {
    /// On and floating (`1`)
    OnFloating,
    /// On and scrolling (`s`) - default
    OnScrolling,
    /// Off except for Instant Answer Menu (`-1`)
    OffExceptInstant,
    /// Completely off (`-2`)
    Off,
}

impl HeaderBehavior {
    /// Returns the string value for the `ko` parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            HeaderBehavior::OnFloating => "1",
            HeaderBehavior::OnScrolling => "s",
            HeaderBehavior::OffExceptInstant => "-1",
            HeaderBehavior::Off => "-2",
        }
    }
}

/// Page numbers display (`kv` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageNumbers {
    /// On with numbers (`1`)
    On,
    /// On without numbers (`n`)
    OnNoNumbers,
    /// Off (`-1`)
    Off,
}

impl PageNumbers {
    /// Returns the string value for the `kv` parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            PageNumbers::On => "1",
            PageNumbers::OnNoNumbers => "n",
            PageNumbers::Off => "-1",
        }
    }
}

/// Units of measure display (`kaj` parameter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitsMeasure {
    /// On with units (`1`)
    On,
    /// On without numbers (`n`)
    OnNoNumbers,
    /// Off (`-1`)
    Off,
}

impl UnitsMeasure {
    /// Returns the string value for the `kaj` parameter.
    pub fn as_str(&self) -> &'static str {
        match self {
            UnitsMeasure::On => "1",
            UnitsMeasure::OnNoNumbers => "n",
            UnitsMeasure::Off => "-1",
        }
    }
}

/// Builder for DuckDuckGo URL search parameters.
///
/// Construct using [`SearchParams::new()`] or [`SearchParams::default()`], then call
/// the fluent setter methods to configure the parameters you need.
///
/// Call [`to_query_pairs()`](SearchParams::to_query_pairs) to obtain a list of
/// `(&'static str, String)` pairs that can be appended to a request URL.
///
/// # Example
/// ```
/// use duckduckgo::params::{SearchParams, Region, SafeSearch, Theme};
///
/// let params = SearchParams::new()
///     .region(Region::FrFr)
///     .safe_search(SafeSearch::Moderate)
///     .theme(Theme::Dark)
///     .source("my_app");
///
/// let pairs = params.to_query_pairs();
/// assert!(pairs.iter().any(|(k, v)| *k == "kl" && v == "fr-fr"));
/// assert!(pairs.iter().any(|(k, v)| *k == "kp" && v == "-1"));
/// assert!(pairs.iter().any(|(k, v)| *k == "kae" && v == "d"));
/// assert!(pairs.iter().any(|(k, v)| *k == "t" && v == "my_app"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    // Result settings
    region: Option<Region>,
    safe_search: Option<SafeSearch>,
    open_instant_answers: Option<Toggle>,
    auto_load_images: Option<Toggle>,
    auto_load_results: Option<Toggle>,
    new_window: Option<Toggle>,
    favicons: Option<Favicons>,
    full_urls: Option<Toggle>,
    auto_suggest: Option<Toggle>,

    // Privacy settings
    redirect: Option<Toggle>,
    https: Option<Toggle>,
    address_bar: Option<AddressBar>,
    video_playback: Option<VideoPlayback>,

    // Colour settings - stored as raw strings to allow custom hex codes
    header_color: Option<String>,
    url_color: Option<String>,
    background_color: Option<String>,
    text_color: Option<String>,
    link_color: Option<String>,
    visited_link_color: Option<String>,

    // Look & feel settings
    theme: Option<Theme>,
    size: Option<Size>,
    width: Option<Width>,
    placement: Option<Placement>,
    link_font: Option<Font>,
    underline: Option<Toggle>,
    text_font: Option<Font>,

    // Interface settings
    header_behavior: Option<HeaderBehavior>,
    advertisements: Option<Toggle>,
    page_numbers: Option<PageNumbers>,
    units_measure: Option<UnitsMeasure>,

    // Source identifier
    source: Option<String>,
}

impl SearchParams {
    /// Creates a new, empty `SearchParams`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the region (`kl` parameter).
    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Sets the safe search level (`kp` parameter).
    pub fn safe_search(mut self, safe_search: SafeSearch) -> Self {
        self.safe_search = Some(safe_search);
        self
    }

    /// Controls the instant answers panel (`kz` parameter).
    pub fn open_instant_answers(mut self, toggle: Toggle) -> Self {
        self.open_instant_answers = Some(toggle);
        self
    }

    /// Controls auto-loading of images (`kc` parameter).
    pub fn auto_load_images(mut self, toggle: Toggle) -> Self {
        self.auto_load_images = Some(toggle);
        self
    }

    /// Controls auto-loading of results (`kav` parameter).
    pub fn auto_load_results(mut self, toggle: Toggle) -> Self {
        self.auto_load_results = Some(toggle);
        self
    }

    /// Controls whether links open in a new window (`kn` parameter).
    pub fn new_window(mut self, toggle: Toggle) -> Self {
        self.new_window = Some(toggle);
        self
    }

    /// Sets the favicons display mode (`kf` parameter).
    pub fn favicons(mut self, favicons: Favicons) -> Self {
        self.favicons = Some(favicons);
        self
    }

    /// Controls whether full URLs are displayed (`kaf` parameter).
    pub fn full_urls(mut self, toggle: Toggle) -> Self {
        self.full_urls = Some(toggle);
        self
    }

    /// Controls auto-suggest (`kac` parameter).
    pub fn auto_suggest(mut self, toggle: Toggle) -> Self {
        self.auto_suggest = Some(toggle);
        self
    }

    /// Controls redirect behaviour (`kd` parameter).
    pub fn redirect(mut self, toggle: Toggle) -> Self {
        self.redirect = Some(toggle);
        self
    }

    /// Controls HTTPS enforcement (`kh` parameter).
    pub fn https(mut self, toggle: Toggle) -> Self {
        self.https = Some(toggle);
        self
    }

    /// Sets the address bar method (`kg` parameter).
    pub fn address_bar(mut self, address_bar: AddressBar) -> Self {
        self.address_bar = Some(address_bar);
        self
    }

    /// Sets the video playback preference (`k5` parameter).
    pub fn video_playback(mut self, video_playback: VideoPlayback) -> Self {
        self.video_playback = Some(video_playback);
        self
    }

    /// Sets the header colour (`kj` parameter). Accepts preset codes or hex strings.
    pub fn header_color(mut self, color: impl Into<String>) -> Self {
        self.header_color = Some(color.into());
        self
    }

    /// Sets the URL colour (`kx` parameter).
    pub fn url_color(mut self, color: impl Into<String>) -> Self {
        self.url_color = Some(color.into());
        self
    }

    /// Sets the background colour (`k7` parameter).
    pub fn background_color(mut self, color: impl Into<String>) -> Self {
        self.background_color = Some(color.into());
        self
    }

    /// Sets the text colour (`k8` parameter).
    pub fn text_color(mut self, color: impl Into<String>) -> Self {
        self.text_color = Some(color.into());
        self
    }

    /// Sets the link colour (`k9` parameter).
    pub fn link_color(mut self, color: impl Into<String>) -> Self {
        self.link_color = Some(color.into());
        self
    }

    /// Sets the visited link colour (`kaa` parameter).
    pub fn visited_link_color(mut self, color: impl Into<String>) -> Self {
        self.visited_link_color = Some(color.into());
        self
    }

    /// Sets the visual theme (`kae` parameter).
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Sets the result size (`ks` parameter).
    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the page width (`kw` parameter).
    pub fn width(mut self, width: Width) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the content placement (`km` parameter).
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = Some(placement);
        self
    }

    /// Sets the link font (`ka` parameter).
    pub fn link_font(mut self, font: Font) -> Self {
        self.link_font = Some(font);
        self
    }

    /// Controls link underline (`ku` parameter).
    pub fn underline(mut self, toggle: Toggle) -> Self {
        self.underline = Some(toggle);
        self
    }

    /// Sets the text font (`kt` parameter).
    pub fn text_font(mut self, font: Font) -> Self {
        self.text_font = Some(font);
        self
    }

    /// Sets the header behaviour (`ko` parameter).
    pub fn header_behavior(mut self, behavior: HeaderBehavior) -> Self {
        self.header_behavior = Some(behavior);
        self
    }

    /// Controls whether advertisements are displayed (`k1` parameter).
    pub fn advertisements(mut self, toggle: Toggle) -> Self {
        self.advertisements = Some(toggle);
        self
    }

    /// Controls page number display (`kv` parameter).
    pub fn page_numbers(mut self, page_numbers: PageNumbers) -> Self {
        self.page_numbers = Some(page_numbers);
        self
    }

    /// Controls units of measure display (`kaj` parameter).
    pub fn units_measure(mut self, units: UnitsMeasure) -> Self {
        self.units_measure = Some(units);
        self
    }

    /// Sets the source identifier (`t` parameter).
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Converts the configured parameters into a list of `(&'static str, String)` key-value pairs
    /// suitable for use as URL query parameters.
    ///
    /// Only parameters that have been explicitly set are included in the output.
    ///
    /// # Example
    /// ```
    /// use duckduckgo::params::{SearchParams, Region};
    ///
    /// let params = SearchParams::new().region(Region::UsEn);
    /// let pairs = params.to_query_pairs();
    /// assert_eq!(pairs, vec![("kl", "us-en".to_string())]);
    /// ```
    pub fn to_query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs: Vec<(&'static str, String)> = Vec::new();

        if let Some(ref r) = self.region {
            pairs.push(("kl", r.as_str().to_string()));
        }
        if let Some(ref s) = self.safe_search {
            pairs.push(("kp", s.as_str().to_string()));
        }
        if let Some(ref t) = self.open_instant_answers {
            pairs.push(("kz", t.as_str().to_string()));
        }
        if let Some(ref t) = self.auto_load_images {
            pairs.push(("kc", t.as_str().to_string()));
        }
        if let Some(ref t) = self.auto_load_results {
            pairs.push(("kav", t.as_str().to_string()));
        }
        if let Some(ref t) = self.new_window {
            pairs.push(("kn", t.as_str().to_string()));
        }
        if let Some(ref f) = self.favicons {
            pairs.push(("kf", f.as_str().to_string()));
        }
        if let Some(ref t) = self.full_urls {
            pairs.push(("kaf", t.as_str().to_string()));
        }
        if let Some(ref t) = self.auto_suggest {
            pairs.push(("kac", t.as_str().to_string()));
        }
        if let Some(ref t) = self.redirect {
            pairs.push(("kd", t.as_str().to_string()));
        }
        if let Some(ref t) = self.https {
            pairs.push(("kh", t.as_str().to_string()));
        }
        if let Some(ref ab) = self.address_bar {
            pairs.push(("kg", ab.as_str().to_string()));
        }
        if let Some(ref vp) = self.video_playback {
            pairs.push(("k5", vp.as_str().to_string()));
        }
        if let Some(ref c) = self.header_color {
            pairs.push(("kj", c.clone()));
        }
        if let Some(ref c) = self.url_color {
            pairs.push(("kx", c.clone()));
        }
        if let Some(ref c) = self.background_color {
            pairs.push(("k7", c.clone()));
        }
        if let Some(ref c) = self.text_color {
            pairs.push(("k8", c.clone()));
        }
        if let Some(ref c) = self.link_color {
            pairs.push(("k9", c.clone()));
        }
        if let Some(ref c) = self.visited_link_color {
            pairs.push(("kaa", c.clone()));
        }
        if let Some(ref t) = self.theme {
            pairs.push(("kae", t.as_string()));
        }
        if let Some(ref s) = self.size {
            pairs.push(("ks", s.as_str().to_string()));
        }
        if let Some(ref w) = self.width {
            pairs.push(("kw", w.as_str().to_string()));
        }
        if let Some(ref p) = self.placement {
            pairs.push(("km", p.as_str().to_string()));
        }
        if let Some(ref f) = self.link_font {
            pairs.push(("ka", f.as_string()));
        }
        if let Some(ref t) = self.underline {
            pairs.push(("ku", t.as_str().to_string()));
        }
        if let Some(ref f) = self.text_font {
            pairs.push(("kt", f.as_string()));
        }
        if let Some(ref h) = self.header_behavior {
            pairs.push(("ko", h.as_str().to_string()));
        }
        if let Some(ref t) = self.advertisements {
            pairs.push(("k1", t.as_str().to_string()));
        }
        if let Some(ref p) = self.page_numbers {
            pairs.push(("kv", p.as_str().to_string()));
        }
        if let Some(ref u) = self.units_measure {
            pairs.push(("kaj", u.as_str().to_string()));
        }
        if let Some(ref s) = self.source {
            pairs.push(("t", s.clone()));
        }

        pairs
    }
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
