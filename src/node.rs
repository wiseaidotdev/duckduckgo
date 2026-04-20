// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Node.js Bindings
//!
//! Exposes the `duckduckgo` library to Node.js via [`napi-derive`].
//! Every type and function is gated behind the `node` cargo feature.
//!
//! The bindings provide **synchronous** wrappers around the async Rust API by
//! driving a temporary, single-threaded [`tokio`] runtime inside each call.
//! This keeps the JavaScript API simple and avoids Promise boilerplate for
//! one-off search calls.
//!
//! # See Also
//!
//! - [DuckDuckGo Instant Answer API](https://duckduckgo.com/duckduckgo-help-pages/open-source/instant-answer-interface/)
//! - [DuckDuckGo URL Parameters](https://duckduckgo.com/duckduckgo-help-pages/settings/params/)
//! - [`napi-rs` documentation](https://napi.rs/)

use crate::browser::Browser;
use crate::params::SearchParams;
use napi_derive::napi;

fn block_on<F, T, E>(future: F) -> napi::Result<T>
where
    F: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| napi::Error::from_reason(e.to_string()))?
        .block_on(future)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
}

fn parse_region(code: &str) -> napi::Result<crate::params::Region> {
    use crate::params::Region;
    match code {
        "xa-ar" => Ok(Region::XaAr),
        "xa-en" => Ok(Region::XaEn),
        "ar-es" => Ok(Region::ArEs),
        "au-en" => Ok(Region::AuEn),
        "at-de" => Ok(Region::AtDe),
        "be-fr" => Ok(Region::BeFr),
        "be-nl" => Ok(Region::BeNl),
        "br-pt" => Ok(Region::BrPt),
        "bg-bg" => Ok(Region::BgBg),
        "ca-en" => Ok(Region::CaEn),
        "ca-fr" => Ok(Region::CaFr),
        "ct-ca" => Ok(Region::CtCa),
        "cl-es" => Ok(Region::ClEs),
        "cn-zh" => Ok(Region::CnZh),
        "co-es" => Ok(Region::CoEs),
        "hr-hr" => Ok(Region::HrHr),
        "cz-cs" => Ok(Region::CzCs),
        "dk-da" => Ok(Region::DkDa),
        "ee-et" => Ok(Region::EeEt),
        "fi-fi" => Ok(Region::FiFi),
        "fr-fr" => Ok(Region::FrFr),
        "de-de" => Ok(Region::DeDe),
        "gr-el" => Ok(Region::GrEl),
        "hk-tzh" => Ok(Region::HkTzh),
        "hu-hu" => Ok(Region::HuHu),
        "in-en" => Ok(Region::InEn),
        "id-id" => Ok(Region::IdId),
        "id-en" => Ok(Region::IdEn),
        "ie-en" => Ok(Region::IeEn),
        "il-he" => Ok(Region::IlHe),
        "it-it" => Ok(Region::ItIt),
        "jp-jp" => Ok(Region::JpJp),
        "kr-kr" => Ok(Region::KrKr),
        "lv-lv" => Ok(Region::LvLv),
        "lt-lt" => Ok(Region::LtLt),
        "xl-es" => Ok(Region::XlEs),
        "my-ms" => Ok(Region::MyMs),
        "my-en" => Ok(Region::MyEn),
        "mx-es" => Ok(Region::MxEs),
        "nl-nl" => Ok(Region::NlNl),
        "nz-en" => Ok(Region::NzEn),
        "no-no" => Ok(Region::NoNo),
        "pe-es" => Ok(Region::PeEs),
        "ph-en" => Ok(Region::PhEn),
        "ph-tl" => Ok(Region::PhTl),
        "pl-pl" => Ok(Region::PlPl),
        "pt-pt" => Ok(Region::PtPt),
        "ro-ro" => Ok(Region::RoRo),
        "ru-ru" => Ok(Region::RuRu),
        "sg-en" => Ok(Region::SgEn),
        "sk-sk" => Ok(Region::SkSk),
        "sl-sl" => Ok(Region::SlSl),
        "za-en" => Ok(Region::ZaEn),
        "es-es" => Ok(Region::EsEs),
        "se-sv" => Ok(Region::SeSv),
        "ch-de" => Ok(Region::ChDe),
        "ch-fr" => Ok(Region::ChFr),
        "ch-it" => Ok(Region::ChIt),
        "tw-tzh" => Ok(Region::TwTzh),
        "th-th" => Ok(Region::ThTh),
        "tr-tr" => Ok(Region::TrTr),
        "ua-uk" => Ok(Region::UaUk),
        "uk-en" => Ok(Region::UkEn),
        "us-en" => Ok(Region::UsEn),
        "ue-es" => Ok(Region::UeEs),
        "ve-es" => Ok(Region::VeEs),
        "vn-vi" => Ok(Region::VnVi),
        "wt-wt" => Ok(Region::WtWt),
        other => Err(napi::Error::from_reason(format!(
            "Unknown region code: '{other}'. See https://duckduckgo.com/duckduckgo-help-pages/settings/params/ for valid values."
        ))),
    }
}

/// A single result returned by the DuckDuckGo Lite text search.
#[napi(object)]
pub struct LiteSearchResult {
    /// The title of the search result.
    pub title: String,
    /// The URL linked to by the result.
    pub url: String,
    /// A short text snippet extracted from the result page.
    pub snippet: String,
}

/// A single image result returned by a DuckDuckGo image search.
#[napi(object)]
pub struct ImageResult {
    /// The title or description of the image.
    pub title: String,
    /// The direct URL to the full-sized image.
    pub image: String,
    /// The URL to a smaller thumbnail preview.
    pub thumbnail: String,
    /// The URL of the page hosting the image.
    pub url: String,
    /// Image height in pixels.
    pub height: u32,
    /// Image width in pixels.
    pub width: u32,
    /// The source or provider of the image.
    pub source: String,
}

/// A single news article returned by a DuckDuckGo news search.
#[napi(object)]
pub struct NewsResult {
    /// The publication date in ISO 8601 format.
    pub date: String,
    /// The headline of the news article.
    pub title: String,
    /// A short excerpt from the article body.
    pub body: String,
    /// The URL of the full article.
    pub url: String,
    /// An optional image URL associated with the article.
    pub image: Option<String>,
    /// The publisher or source of the article.
    pub source: String,
}

/// A topic entry from the DuckDuckGo Instant Answer API response.
#[napi(object)]
pub struct RelatedTopic {
    /// The plain-text description of the topic.
    pub text: Option<String>,
    /// The primary URL for this topic.
    pub first_url: Option<String>,
    /// An alternative URL for this topic.
    pub url: Option<String>,
    /// The raw HTML result string.
    pub result: Option<String>,
}

/// The structured response from the DuckDuckGo Instant Answer API.
#[napi(object)]
pub struct InstantAnswerResponse {
    /// The heading or title of the answer.
    pub heading: Option<String>,
    /// The full abstract text.
    pub abstract_text: Option<String>,
    /// The source of the abstract.
    pub abstract_source: Option<String>,
    /// The URL of the abstract source.
    pub abstract_url: Option<String>,
    /// A direct answer to the query.
    pub answer: Option<String>,
    /// The type classification of the answer.
    pub answer_type: Option<String>,
    /// A definition associated with the query.
    pub definition: Option<String>,
    /// The source of the definition.
    pub definition_source: Option<String>,
    /// The URL of the definition source.
    pub definition_url: Option<String>,
    /// The entity type of the result.
    pub entity: Option<String>,
    /// An image URL associated with the result.
    pub image: Option<String>,
    /// A redirect URL when the result is a disambiguation redirect.
    pub redirect: Option<String>,
    /// The response type code (e.g. ``"A"`` for article, ``"D"`` for disambiguation).
    pub response_type: String,
    /// Related topic entries.
    pub related_topics: Vec<RelatedTopic>,
}

/// A fluent builder for DuckDuckGo URL search parameters.
///
/// Construct with ``new SearchParams()`` and chain setter methods to configure
/// the region, safe-search level, visual theme, colours, and other DDG
/// settings. Pass the configured instance to ``Browser.instantAnswer()``.
///
/// Each setter returns ``this`` to allow method chaining.
///
/// See the [DuckDuckGo URL Parameters docs](https://duckduckgo.com/duckduckgo-help-pages/settings/params/)
/// for the full list of supported parameters.
#[napi(js_name = "SearchParams")]
pub struct NapiSearchParams {
    pub(crate) inner: SearchParams,
}

#[napi]
impl NapiSearchParams {
    /// Create a new, empty ``SearchParams`` with all options at their defaults.
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: SearchParams::default(),
        }
    }

    /// Set the region / language code for the ``kl`` parameter.
    ///
    /// Accepts a DDG region code such as ``"us-en"``, ``"fr-fr"``, or
    /// ``"wt-wt"`` (worldwide). See the DuckDuckGo params docs for the
    /// full list of valid codes.
    #[napi]
    pub fn region(&self, code: String) -> napi::Result<NapiSearchParams> {
        let region = parse_region(&code)?;
        Ok(Self {
            inner: self.inner.clone().region(region),
        })
    }

    /// Set the safe-search level for the ``kp`` parameter.
    ///
    /// Accepted values: ``"on"``, ``"moderate"``, ``"off"``.
    #[napi]
    pub fn safe_search(&self, level: String) -> napi::Result<NapiSearchParams> {
        use crate::params::SafeSearch;
        let safe = match level.as_str() {
            "on" => SafeSearch::On,
            "moderate" => SafeSearch::Moderate,
            "off" => SafeSearch::Off,
            other => {
                return Err(napi::Error::from_reason(format!(
                    "Unknown safe search level: '{other}'. Expected 'on', 'moderate', or 'off'."
                )));
            }
        };
        Ok(Self {
            inner: self.inner.clone().safe_search(safe),
        })
    }

    /// Set the visual theme for the ``kae`` parameter.
    ///
    /// Accepted values: ``"default"``, ``"contrast"``, ``"retro"``,
    /// ``"dark"``, ``"terminal"``, or a custom hex colour code.
    #[napi]
    pub fn theme(&self, name: String) -> NapiSearchParams {
        use crate::params::Theme;
        let theme = match name.as_str() {
            "default" => Theme::Default,
            "contrast" => Theme::Contrast,
            "retro" => Theme::Retro,
            "dark" => Theme::Dark,
            "terminal" => Theme::Terminal,
            other => Theme::Custom(other.to_string()),
        };
        Self {
            inner: self.inner.clone().theme(theme),
        }
    }

    /// Set the source identifier sent as the ``t`` parameter.
    #[napi]
    pub fn source(&self, src: String) -> NapiSearchParams {
        Self {
            inner: self.inner.clone().source(src),
        }
    }

    /// Set the header background colour (``kj`` parameter).
    ///
    /// Accepts a hex colour code, e.g. ``"2d4f67"``.
    #[napi]
    pub fn header_color(&self, color: String) -> NapiSearchParams {
        Self {
            inner: self.inner.clone().header_color(color),
        }
    }

    /// Set the URL text colour (``kx`` parameter).
    #[napi]
    pub fn url_color(&self, color: String) -> NapiSearchParams {
        Self {
            inner: self.inner.clone().url_color(color),
        }
    }

    /// Set the page background colour (``k7`` parameter).
    #[napi]
    pub fn background_color(&self, color: String) -> NapiSearchParams {
        Self {
            inner: self.inner.clone().background_color(color),
        }
    }

    /// Set the body text colour (``k8`` parameter).
    #[napi]
    pub fn text_color(&self, color: String) -> NapiSearchParams {
        Self {
            inner: self.inner.clone().text_color(color),
        }
    }

    /// Set the link colour (``k9`` parameter).
    #[napi]
    pub fn link_color(&self, color: String) -> NapiSearchParams {
        Self {
            inner: self.inner.clone().link_color(color),
        }
    }

    /// Set the visited-link colour (``kaa`` parameter).
    #[napi]
    pub fn visited_link_color(&self, color: String) -> NapiSearchParams {
        Self {
            inner: self.inner.clone().visited_link_color(color),
        }
    }

    /// Return the configured parameters as an array of ``[key, value]`` pairs.
    ///
    /// Only parameters that were explicitly set are included.
    #[napi]
    pub fn to_query_pairs(&self) -> Vec<Vec<String>> {
        self.inner
            .to_query_pairs()
            .into_iter()
            .map(|(k, v)| vec![k.to_string(), v])
            .collect()
    }
}

/// An HTTP client for executing DuckDuckGo searches.
///
/// Construct with ``new Browser()`` for a zero-configuration default, or
/// supply optional arguments to customise the User-Agent, cookie store, or
/// proxy. All network methods are **synchronous** on the JavaScript side;
/// they drive an internal single-threaded Tokio runtime per call.
///
/// See the [DuckDuckGo Instant Answer API docs](https://duckduckgo.com/duckduckgo-help-pages/open-source/instant-answer-interface/)
/// and the [URL Parameters reference](https://duckduckgo.com/duckduckgo-help-pages/settings/params/)
/// for details on available query options.
#[napi(js_name = "Browser")]
pub struct NapiBrowser {
    inner: Browser,
}

#[napi]
impl NapiBrowser {
    /// Create a new ``Browser``.
    ///
    /// ``userAgent``   - Optional User-Agent string override.
    /// ``cookieStore`` - Enable cookie persistence (default ``false``).
    /// ``proxy``       - Optional proxy URL, e.g. ``"socks5://127.0.0.1:9050"``.
    #[napi(constructor)]
    pub fn new(
        user_agent: Option<String>,
        cookie_store: Option<bool>,
        proxy: Option<String>,
    ) -> napi::Result<Self> {
        let mut builder = Browser::builder();
        if let Some(agent) = user_agent {
            builder = builder.user_agent(agent);
        }
        if cookie_store.unwrap_or(false) {
            builder = builder.cookie_store(true);
        }
        if let Some(proxy_url) = proxy {
            builder = builder.proxy(proxy_url);
        }
        let inner = builder
            .build()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        Ok(Self { inner })
    }

    /// Perform a DuckDuckGo Lite text search.
    ///
    /// ``query``     - The search query.
    /// ``region``    - DDG region code, e.g. ``"us-en"`` (default ``"wt-wt"``).
    /// ``limit``     - Maximum number of results (``null`` returns all).
    /// ``userAgent`` - User-Agent header value (default ``""``, uses client default).
    ///
    /// Returns an array of ``LiteSearchResult`` objects.
    ///
    /// See [DuckDuckGo Lite](https://lite.duckduckgo.com/lite/) for details.
    #[napi]
    pub fn lite_search(
        &self,
        query: String,
        region: Option<String>,
        limit: Option<u32>,
        user_agent: Option<String>,
    ) -> napi::Result<Vec<LiteSearchResult>> {
        let region = region.as_deref().unwrap_or("wt-wt");
        let user_agent_str = user_agent.as_deref().unwrap_or("");
        let limit = limit.map(|l| l as usize);
        let results =
            block_on(
                self.inner
                    .lite_search(query.as_str(), region, limit, user_agent_str),
            )?;
        Ok(results
            .into_iter()
            .map(|r| LiteSearchResult {
                title: r.title,
                url: r.url,
                snippet: r.snippet,
            })
            .collect())
    }

    /// Perform a DuckDuckGo image search.
    ///
    /// ``query``      - The search query.
    /// ``region``     - DDG region code (default ``"wt-wt"``).
    /// ``safesearch`` - Enable safe search filtering (default ``true``).
    /// ``limit``      - Maximum number of results.
    /// ``userAgent``  - User-Agent header value.
    ///
    /// Returns an array of ``ImageResult`` objects.
    #[napi]
    pub fn images(
        &self,
        query: String,
        region: Option<String>,
        safesearch: Option<bool>,
        limit: Option<u32>,
        user_agent: Option<String>,
    ) -> napi::Result<Vec<ImageResult>> {
        let region = region.as_deref().unwrap_or("wt-wt");
        let safesearch = safesearch.unwrap_or(true);
        let user_agent_str = user_agent.as_deref().unwrap_or("");
        let limit = limit.map(|l| l as usize);
        let results =
            block_on(
                self.inner
                    .images(query.as_str(), region, safesearch, limit, user_agent_str),
            )?;
        Ok(results
            .into_iter()
            .map(|r| ImageResult {
                title: r.title,
                image: r.image,
                thumbnail: r.thumbnail,
                url: r.url,
                height: r.height,
                width: r.width,
                source: r.source,
            })
            .collect())
    }

    /// Perform a DuckDuckGo news search.
    ///
    /// ``query``      - The search query.
    /// ``region``     - DDG region code (default ``"wt-wt"``).
    /// ``safesearch`` - Enable safe search (default ``true``).
    /// ``limit``      - Maximum number of results.
    /// ``userAgent``  - User-Agent header value.
    ///
    /// Returns an array of ``NewsResult`` objects.
    #[napi]
    pub fn news(
        &self,
        query: String,
        region: Option<String>,
        safesearch: Option<bool>,
        limit: Option<u32>,
        user_agent: Option<String>,
    ) -> napi::Result<Vec<NewsResult>> {
        let region = region.as_deref().unwrap_or("wt-wt");
        let safesearch = safesearch.unwrap_or(true);
        let user_agent_str = user_agent.as_deref().unwrap_or("");
        let limit = limit.map(|l| l as usize);
        let results =
            block_on(
                self.inner
                    .news(query.as_str(), region, safesearch, limit, user_agent_str),
            )?;
        Ok(results
            .into_iter()
            .map(|r| NewsResult {
                date: r.date,
                title: r.title,
                body: r.body,
                url: r.url,
                image: r.image,
                source: r.source,
            })
            .collect())
    }

    /// Query the DuckDuckGo Instant Answer API.
    ///
    /// ``query``  - The search query, URL-encoded if necessary.
    /// ``params`` - Optional ``SearchParams`` for additional DDG URL parameters.
    ///
    /// Returns an ``InstantAnswerResponse`` object with all Instant Answer
    /// API fields populated.
    ///
    /// See the [Instant Answer API docs](https://duckduckgo.com/duckduckgo-help-pages/open-source/instant-answer-interface/).
    #[napi]
    pub fn instant_answer(
        &self,
        query: String,
        params: Option<&NapiSearchParams>,
    ) -> napi::Result<InstantAnswerResponse> {
        let path = format!("?q={}", urlencoding::encode(&query));
        let search_params = params.map(|p| &p.inner);
        let resp = block_on(self.inner.get_api_response(&path, search_params))?;
        Ok(InstantAnswerResponse {
            heading: resp.heading,
            abstract_text: resp.abstract_text,
            abstract_source: resp.abstract_source,
            abstract_url: resp.abstract_url,
            answer: resp.answer,
            answer_type: resp.answer_type,
            definition: resp.definition,
            definition_source: resp.definition_source,
            definition_url: resp.definition_url,
            entity: resp.entity,
            image: resp.image,
            redirect: resp.redirect,
            response_type: resp.r#type,
            related_topics: resp
                .related_topics
                .into_iter()
                .map(|t| RelatedTopic {
                    text: t.text,
                    first_url: t.first_url,
                    url: t.url,
                    result: t.result,
                })
                .collect(),
        })
    }
}

/// Run the ``ddg`` CLI from an argument list.
///
/// ``args`` - Argument list in the same format as ``process.argv``. The first
///            element (program name) is consumed by the argument parser.
#[napi]
pub fn run_cli(args: Vec<String>) {
    tokio::runtime::Runtime::new()
        .expect("Failed to create tokio runtime")
        .block_on(async move {
            if let Err(e) = crate::app::run_cli_entry(args).await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        });
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
