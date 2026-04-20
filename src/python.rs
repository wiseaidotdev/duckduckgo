// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Python Bindings
//!
//! Exposes the `duckduckgo` library to Python via [`pyo3`].
//! Every type and function is gated behind the `python` cargo feature.
//!
//! The bindings provide **synchronous** wrappers around the async Rust API by
//! driving a temporary, single-threaded [`tokio`] runtime inside each call,
//! making the API feel native and straightforward for Python callers.
//!
//! # See Also
//!
//! - [DuckDuckGo Instant Answer API](https://duckduckgo.com/duckduckgo-help-pages/open-source/instant-answer-interface/)
//! - [DuckDuckGo URL Parameters](https://duckduckgo.com/duckduckgo-help-pages/settings/params/)

use crate::browser::Browser;
use crate::params::SearchParams;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

fn block_on<F, T, E>(future: F) -> PyResult<T>
where
    F: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?
        .block_on(future)
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))
}

fn parse_region(code: &str) -> PyResult<crate::params::Region> {
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
        other => Err(PyRuntimeError::new_err(format!(
            "Unknown region code: '{other}'. See https://duckduckgo.com/duckduckgo-help-pages/settings/params/ for valid values."
        ))),
    }
}

/// A single result returned by the DuckDuckGo Lite text search.
///
/// All fields are read-only once the object is constructed by a
/// :meth:`Browser.lite_search` call.
///
/// # See Also
///
/// - [DuckDuckGo Lite](https://lite.duckduckgo.com/lite/)
#[pyclass(name = "LiteSearchResult", frozen, skip_from_py_object)]
#[derive(Debug, Clone)]
pub struct PyLiteSearchResult {
    /// The title of the search result.
    #[pyo3(get)]
    pub title: String,
    /// The URL linked to by the result.
    #[pyo3(get)]
    pub url: String,
    /// A short text snippet extracted from the result page.
    #[pyo3(get)]
    pub snippet: String,
}

#[pymethods]
impl PyLiteSearchResult {
    pub fn __repr__(&self) -> String {
        format!(
            "LiteSearchResult(title={:?}, url={:?})",
            self.title, self.url
        )
    }
}

/// A single image result returned by a DuckDuckGo image search.
///
/// All fields are read-only once the object is constructed by an
/// :meth:`Browser.images` call.
///
/// # See Also
///
/// - [DuckDuckGo Images API](https://duckduckgo.com/i.js)
#[pyclass(name = "ImageResult", frozen, skip_from_py_object)]
#[derive(Debug, Clone)]
pub struct PyImageResult {
    /// The title or description of the image.
    #[pyo3(get)]
    pub title: String,
    /// The direct URL to the full-sized image.
    #[pyo3(get)]
    pub image: String,
    /// The URL to the thumbnail preview.
    #[pyo3(get)]
    pub thumbnail: String,
    /// The URL of the page hosting the image.
    #[pyo3(get)]
    pub url: String,
    /// Image height in pixels.
    #[pyo3(get)]
    pub height: u32,
    /// Image width in pixels.
    #[pyo3(get)]
    pub width: u32,
    /// The source or provider of the image.
    #[pyo3(get)]
    pub source: String,
}

#[pymethods]
impl PyImageResult {
    pub fn __repr__(&self) -> String {
        format!("ImageResult(title={:?}, url={:?})", self.title, self.url)
    }
}

/// A single news article returned by a DuckDuckGo news search.
///
/// All fields are read-only once the object is constructed by a
/// :meth:`Browser.news` call.
///
/// # See Also
///
/// - [DuckDuckGo News API](https://duckduckgo.com/news.js)
#[pyclass(name = "NewsResult", frozen, skip_from_py_object)]
#[derive(Debug, Clone)]
pub struct PyNewsResult {
    /// The publication date in ISO 8601 format.
    #[pyo3(get)]
    pub date: String,
    /// The headline of the news article.
    #[pyo3(get)]
    pub title: String,
    /// A short excerpt from the article body.
    #[pyo3(get)]
    pub body: String,
    /// The URL of the full article.
    #[pyo3(get)]
    pub url: String,
    /// An optional image URL associated with the article.
    #[pyo3(get)]
    pub image: Option<String>,
    /// The publisher or source of the article.
    #[pyo3(get)]
    pub source: String,
}

#[pymethods]
impl PyNewsResult {
    pub fn __repr__(&self) -> String {
        format!(
            "NewsResult(title={:?}, url={:?}, date={:?})",
            self.title, self.url, self.date
        )
    }
}

/// A fluent builder for DuckDuckGo URL search parameters.
///
/// Construct with ``SearchParams()`` and chain setter methods to configure
/// the region, safe-search level, visual theme, colours, and other DDG
/// settings. The configured instance can be passed to
/// :meth:`Browser.instant_answer`.
///
/// Each setter returns a **new** ``SearchParams`` instance so calls can be
/// chained freely.
///
/// # See Also
///
/// - [DuckDuckGo URL Parameters](https://duckduckgo.com/duckduckgo-help-pages/settings/params/)
/// - [DuckDuckGo Settings Overview](https://duckduckgo.com/settings)
#[pyclass(name = "SearchParams", skip_from_py_object)]
#[derive(Debug, Clone, Default)]
pub struct PySearchParams {
    pub(crate) inner: SearchParams,
}

#[pymethods]
impl PySearchParams {
    /// Create a new, empty ``SearchParams`` with all options at their defaults.
    #[new]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the region / language code for the ``kl`` parameter.
    ///
    /// Args:
    ///     code: A DDG region code such as ``"us-en"``, ``"fr-fr"``, or
    ///           ``"wt-wt"`` (worldwide). For the full list see the
    ///           `DuckDuckGo params docs`_.
    ///
    /// Raises:
    ///     RuntimeError: If the supplied code is not a recognised DDG region.
    ///
    /// .. _DuckDuckGo params docs: https://duckduckgo.com/duckduckgo-help-pages/settings/params/
    pub fn region(&self, code: String) -> PyResult<PySearchParams> {
        let region = parse_region(&code)?;
        Ok(Self {
            inner: self.inner.clone().region(region),
        })
    }

    /// Set the safe-search level for the ``kp`` parameter.
    ///
    /// Args:
    ///     level: One of ``"on"``, ``"moderate"``, or ``"off"``.
    ///
    /// Raises:
    ///     RuntimeError: If the supplied level is not recognised.
    pub fn safe_search(&self, level: String) -> PyResult<PySearchParams> {
        use crate::params::SafeSearch;
        let safe = match level.as_str() {
            "on" => SafeSearch::On,
            "moderate" => SafeSearch::Moderate,
            "off" => SafeSearch::Off,
            other => {
                return Err(PyRuntimeError::new_err(format!(
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
    /// Args:
    ///     name: One of ``"default"``, ``"contrast"``, ``"retro"``,
    ///           ``"dark"``, ``"terminal"``, or a custom hex colour code.
    pub fn theme(&self, name: String) -> PySearchParams {
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
    ///
    /// Args:
    ///     src: An identifier string for your application.
    pub fn source(&self, src: String) -> PySearchParams {
        Self {
            inner: self.inner.clone().source(src),
        }
    }

    /// Set the header background colour (``kj`` parameter).
    ///
    /// Args:
    ///     color: A hex colour code, e.g. ``"2d4f67"``.
    pub fn header_color(&self, color: String) -> PySearchParams {
        Self {
            inner: self.inner.clone().header_color(color),
        }
    }

    /// Set the URL text colour (``kx`` parameter).
    ///
    /// Args:
    ///     color: A hex colour code.
    pub fn url_color(&self, color: String) -> PySearchParams {
        Self {
            inner: self.inner.clone().url_color(color),
        }
    }

    /// Set the page background colour (``k7`` parameter).
    ///
    /// Args:
    ///     color: A hex colour code.
    pub fn background_color(&self, color: String) -> PySearchParams {
        Self {
            inner: self.inner.clone().background_color(color),
        }
    }

    /// Set the body text colour (``k8`` parameter).
    ///
    /// Args:
    ///     color: A hex colour code.
    pub fn text_color(&self, color: String) -> PySearchParams {
        Self {
            inner: self.inner.clone().text_color(color),
        }
    }

    /// Set the link colour (``k9`` parameter).
    ///
    /// Args:
    ///     color: A hex colour code.
    pub fn link_color(&self, color: String) -> PySearchParams {
        Self {
            inner: self.inner.clone().link_color(color),
        }
    }

    /// Set the visited-link colour (``kaa`` parameter).
    ///
    /// Args:
    ///     color: A hex colour code.
    pub fn visited_link_color(&self, color: String) -> PySearchParams {
        Self {
            inner: self.inner.clone().visited_link_color(color),
        }
    }

    /// Return the configured parameters as a list of ``(key, value)`` tuples.
    ///
    /// Only parameters that were explicitly set are included. The tuples can
    /// be used directly to construct a query string.
    pub fn to_query_pairs(&self) -> Vec<(String, String)> {
        self.inner
            .to_query_pairs()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect()
    }

    pub fn __repr__(&self) -> String {
        format!("SearchParams({:?})", self.inner.to_query_pairs())
    }
}

/// An HTTP client for executing DuckDuckGo searches.
///
/// Construct with ``Browser()`` for a zero-configuration default, or supply
/// optional keyword arguments to customise the User-Agent, cookie store, or
/// proxy. All network methods are **synchronous** from Python's perspective;
/// they drive an internal single-threaded Tokio runtime for each call.
///
/// # See Also
///
/// - [DuckDuckGo Instant Answer API](https://duckduckgo.com/duckduckgo-help-pages/open-source/instant-answer-interface/)
/// - [DuckDuckGo URL Parameters](https://duckduckgo.com/duckduckgo-help-pages/settings/params/)
/// - [DuckDuckGo Help Pages](https://duckduckgo.com/duckduckgo-help-pages/)
#[pyclass(name = "Browser")]
pub struct PyBrowser {
    inner: Browser,
}

#[pymethods]
impl PyBrowser {
    /// Create a new ``Browser``.
    ///
    /// Args:
    ///     user_agent:   Optional User-Agent string override. When omitted the
    ///                   default ``reqwest`` user-agent is used.
    ///     cookie_store: Enable cookie persistence across requests
    ///                   (default ``False``).
    ///     proxy:        Optional proxy URL, e.g. ``"socks5://127.0.0.1:9050"``.
    ///
    /// Raises:
    ///     RuntimeError: If the proxy URL is invalid or the HTTP client cannot
    ///                   be constructed.
    #[new]
    #[pyo3(signature = (user_agent=None, cookie_store=false, proxy=None))]
    pub fn new(
        user_agent: Option<String>,
        cookie_store: bool,
        proxy: Option<String>,
    ) -> PyResult<Self> {
        let mut builder = Browser::builder();
        if let Some(agent) = user_agent {
            builder = builder.user_agent(agent);
        }
        if cookie_store {
            builder = builder.cookie_store(true);
        }
        if let Some(proxy_url) = proxy {
            builder = builder.proxy(proxy_url);
        }
        let inner = builder
            .build()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(Self { inner })
    }

    /// Perform a DuckDuckGo Lite text search.
    ///
    /// DuckDuckGo Lite is a minimal, HTML-only interface that works without
    /// JavaScript and is well suited for scraping structured results.
    ///
    /// Args:
    ///     query:      The search query string.
    ///     region:     DDG region code controlling the result language and
    ///                 locale (default ``"wt-wt"`` for worldwide).
    ///     limit:      Maximum number of results to return. When ``None`` all
    ///                 available results are returned.
    ///     user_agent: User-Agent header value. Defaults to the client's
    ///                 configured agent when left empty.
    ///
    /// Returns:
    ///     A list of :class:`LiteSearchResult` objects.
    ///
    /// Raises:
    ///     RuntimeError: On network or parsing errors.
    ///
    /// # See Also
    ///
    /// - [DuckDuckGo Lite](https://lite.duckduckgo.com/lite/)
    #[pyo3(signature = (query, region="wt-wt", limit=None, user_agent=""))]
    pub fn lite_search(
        &self,
        query: String,
        region: &str,
        limit: Option<usize>,
        user_agent: &str,
    ) -> PyResult<Vec<PyLiteSearchResult>> {
        let results = block_on(self.inner.lite_search(&query, region, limit, user_agent))?;
        Ok(results
            .into_iter()
            .map(|r| PyLiteSearchResult {
                title: r.title,
                url: r.url,
                snippet: r.snippet,
            })
            .collect())
    }

    /// Perform a DuckDuckGo image search.
    ///
    /// Images are fetched from the ``/i.js`` endpoint using the ``vqd`` token
    /// obtained from the main search page, following DuckDuckGo's pagination
    /// chain until the result limit is reached or all pages are exhausted.
    ///
    /// Args:
    ///     query:      The search query string.
    ///     region:     DDG region code (default ``"wt-wt"``).
    ///     safesearch: Whether to enable safe search filtering (default ``True``).
    ///     limit:      Maximum number of results to return.
    ///     user_agent: User-Agent header value.
    ///
    /// Returns:
    ///     A list of :class:`ImageResult` objects.
    ///
    /// Raises:
    ///     RuntimeError: On network or JSON parsing errors.
    #[pyo3(signature = (query, region="wt-wt", safesearch=true, limit=None, user_agent=""))]
    pub fn images(
        &self,
        query: String,
        region: &str,
        safesearch: bool,
        limit: Option<usize>,
        user_agent: &str,
    ) -> PyResult<Vec<PyImageResult>> {
        let results = block_on(
            self.inner
                .images(&query, region, safesearch, limit, user_agent),
        )?;
        Ok(results
            .into_iter()
            .map(|r| PyImageResult {
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
    /// News articles are fetched from the ``/news.js`` endpoint using the
    /// ``vqd`` token. Results are paginated automatically until the limit
    /// is reached or all pages are exhausted.
    ///
    /// Args:
    ///     query:      The search query string.
    ///     region:     DDG region code (default ``"wt-wt"``).
    ///     safesearch: Whether to enable safe search (default ``True``).
    ///     limit:      Maximum number of results to return.
    ///     user_agent: User-Agent header value.
    ///
    /// Returns:
    ///     A list of :class:`NewsResult` objects.
    ///
    /// Raises:
    ///     RuntimeError: On network or JSON parsing errors.
    #[pyo3(signature = (query, region="wt-wt", safesearch=true, limit=None, user_agent=""))]
    pub fn news(
        &self,
        query: String,
        region: &str,
        safesearch: bool,
        limit: Option<usize>,
        user_agent: &str,
    ) -> PyResult<Vec<PyNewsResult>> {
        let results = block_on(
            self.inner
                .news(&query, region, safesearch, limit, user_agent),
        )?;
        Ok(results
            .into_iter()
            .map(|r| PyNewsResult {
                date: r.date,
                title: r.title,
                body: r.body,
                url: r.url,
                image: r.image,
                source: r.source,
            })
            .collect())
    }

    /// Query the DuckDuckGo Instant Answer API and return the response as a
    /// Python ``dict``.
    ///
    /// This method calls the ``api.duckduckgo.com`` Instant Answer endpoint
    /// and returns all available fields in a plain dictionary. Supply a
    /// :class:`SearchParams` to add extra DDG URL parameters.
    ///
    /// Args:
    ///     query:  The search query, URL-encoded if necessary.
    ///     params: Optional :class:`SearchParams` for additional URL parameters
    ///             such as region, theme, or safe-search level.
    ///
    /// Returns:
    ///     A ``dict`` containing the Instant Answer API response fields, plus
    ///     a ``"related_topics"`` key with a list of topic dicts.
    ///
    /// Raises:
    ///     RuntimeError: On network, HTTP, or JSON parsing errors.
    ///
    /// # See Also
    ///
    /// - [Instant Answer API docs](https://duckduckgo.com/duckduckgo-help-pages/open-source/instant-answer-interface/)
    #[pyo3(signature = (query, params=None))]
    pub fn instant_answer(
        &self,
        py: Python<'_>,
        query: String,
        params: Option<PyRef<'_, PySearchParams>>,
    ) -> PyResult<Py<PyDict>> {
        let path = format!("?q={}", urlencoding::encode(&query));
        let search_params = params.as_deref().map(|p| &p.inner);
        let resp = block_on(self.inner.get_api_response(&path, search_params))?;

        let d = PyDict::new(py);
        d.set_item("heading", resp.heading)?;
        d.set_item("abstract_text", resp.abstract_text)?;
        d.set_item("abstract_source", resp.abstract_source)?;
        d.set_item("abstract_url", resp.abstract_url)?;
        d.set_item("answer", resp.answer)?;
        d.set_item("answer_type", resp.answer_type)?;
        d.set_item("definition", resp.definition)?;
        d.set_item("definition_source", resp.definition_source)?;
        d.set_item("definition_url", resp.definition_url)?;
        d.set_item("entity", resp.entity)?;
        d.set_item("image", resp.image)?;
        d.set_item("redirect", resp.redirect)?;
        d.set_item("type", resp.r#type)?;

        let topics = PyList::empty(py);
        for topic in resp.related_topics {
            let t = PyDict::new(py);
            t.set_item("text", topic.text)?;
            t.set_item("first_url", topic.first_url)?;
            t.set_item("url", topic.url)?;
            t.set_item("result", topic.result)?;
            topics.append(t)?;
        }
        d.set_item("related_topics", topics)?;

        Ok(d.unbind())
    }
}

/// Register all Python-exposed types and functions into the ``_ddg`` module.
pub fn register_python_module(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyLiteSearchResult>()?;
    m.add_class::<PyImageResult>()?;
    m.add_class::<PyNewsResult>()?;
    m.add_class::<PySearchParams>()?;
    m.add_class::<PyBrowser>()?;
    Ok(())
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
