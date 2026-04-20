// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # API Response Types
//!
//! This module contains the deserialization types for every response format
//! supported by the DuckDuckGo search backends.
//!
//! ## Response Types
//!
//! | Type | Backend | Description |
//! |------|---------|-------------|
//! | [`Response`] | Instant Answer API | Full structured API response |
//! | [`LiteSearchResult`] | DuckDuckGo Lite | Single HTML text result |
//! | [`ImageResult`] | Images API | Single image result |
//! | [`NewsResult`] | News API | Single news article |
//!
//! ## Instant Answer API Response Structure
//!
//! The [`Response`] struct maps directly to the JSON returned by
//! `api.duckduckgo.com`. Key fields include:
//!
//! - [`Response::heading`] - topic or entity heading.
//! - [`Response::abstract_text`] - Wikipedia-style article summary.
//! - [`Response::answer`] - direct answer, e.g. for calculations.
//! - [`Response::related_topics`] - list of [`crate::topic::Topic`] entries.
//! - [`Response::meta`] - [`Meta`] struct describing the Instant Answer plugin.
//!
//! ## See Also
//!
//! - [DuckDuckGo Instant Answer API](https://duckduckgo.com/duckduckgo-help-pages/open-source/instant-answer-interface/)
//! - [Instant Answer API Playground](https://api.duckduckgo.com/?q=rust&format=json&pretty=1)
//! - [DuckDuckGo Open Source Instant Answers](https://duck.co/ia)
//! - [DuckDuckGo Help Pages](https://duckduckgo.com/duckduckgo-help-pages/)

use crate::topic::Topic;
use serde::Deserialize;
use serde_json::Value;

/// A developer entry inside the `meta.developer` array.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Developer {
    /// Developer's display name.
    pub name: Option<String>,
    /// Developer type (e.g. `"ddg"`).
    #[serde(rename = "type")]
    pub developer_type: Option<String>,
    /// Developer URL.
    pub url: Option<String>,
}

/// Maintainer information inside `meta.maintainer`.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Maintainer {
    /// GitHub handle of the maintainer.
    pub github: Option<String>,
}

/// Source options inside `meta.src_options`.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct SrcOptions {
    /// Source directory.
    pub directory: Option<String>,
    /// Whether the source is fanon content.
    pub is_fanon: Option<Value>,
    /// Whether the source is a MediaWiki.
    pub is_mediawiki: Option<Value>,
    /// Whether the source is Wikipedia.
    pub is_wikipedia: Option<Value>,
    /// Source language code.
    pub language: Option<String>,
    /// Minimum abstract length.
    pub min_abstract_length: Option<String>,
    /// Skip abstract flag.
    pub skip_abstract: Option<Value>,
    /// Skip abstract parenthetical flag.
    pub skip_abstract_paren: Option<Value>,
    /// Skip end of abstract string.
    pub skip_end: Option<String>,
    /// Skip icon flag.
    pub skip_icon: Option<Value>,
    /// Skip image name flag.
    pub skip_image_name: Option<Value>,
    /// Skip QR code flag.
    pub skip_qr: Option<String>,
    /// Source skip pattern.
    pub source_skip: Option<String>,
    /// Additional source info string.
    pub src_info: Option<String>,
}

/// Metadata about the Instant Answer source, returned in the `meta` field of the API response.
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Meta {
    /// Attribution text, if any.
    pub attribution: Option<String>,
    /// Block group identifier.
    pub blockgroup: Option<String>,
    /// Date the Instant Answer was created.
    pub created_date: Option<String>,
    /// Human-readable description of the source.
    pub description: Option<String>,
    /// Designer name, if any.
    pub designer: Option<String>,
    /// Development date.
    pub dev_date: Option<String>,
    /// Development milestone (e.g. `"live"`).
    pub dev_milestone: Option<String>,
    /// List of developers.
    pub developer: Option<Vec<Developer>>,
    /// An example query for this Instant Answer.
    pub example_query: Option<String>,
    /// Unique identifier for the Instant Answer plugin.
    pub id: Option<String>,
    /// Whether the source is a Stack Exchange site.
    pub is_stackexchange: Option<Value>,
    /// JavaScript callback name.
    pub js_callback_name: Option<String>,
    /// Date the Instant Answer went live.
    pub live_date: Option<String>,
    /// Maintainer information.
    pub maintainer: Option<Maintainer>,
    /// Display name of the source.
    pub name: Option<String>,
    /// Perl module implementing this Instant Answer.
    pub perl_module: Option<String>,
    /// Producer name, if any.
    pub producer: Option<String>,
    /// Production state (e.g. `"online"`).
    pub production_state: Option<String>,
    /// Repository type (e.g. `"fathead"`).
    pub repo: Option<String>,
    /// Identifier used for signal tracking.
    pub signal_from: Option<String>,
    /// Domain of the primary source.
    pub src_domain: Option<String>,
    /// Numeric identifier of the primary source.
    pub src_id: Option<i64>,
    /// Name of the primary source.
    pub src_name: Option<String>,
    /// Detailed options for the source.
    pub src_options: Option<SrcOptions>,
    /// URL of the primary source.
    pub src_url: Option<String>,
    /// Status string (e.g. `"live"`).
    pub status: Option<String>,
    /// Tab category for the Instant Answer.
    pub tab: Option<String>,
    /// Topic tags.
    pub topic: Option<Vec<String>>,
    /// Whether the Instant Answer is flagged as unsafe.
    #[serde(rename = "unsafe")]
    pub unsafe_flag: Option<Value>,
}

/// A struct representing the response received from the DuckDuckGo Instant Answer API.
#[derive(Debug, Deserialize)]
pub struct Response {
    /// The abstract text associated with the search result (may be empty).
    #[serde(rename = "Abstract")]
    pub r#abstract: Option<String>,

    /// The source of the abstract, if available.
    #[serde(rename = "AbstractSource")]
    pub abstract_source: Option<String>,

    /// The detailed abstract text associated with the search result.
    #[serde(rename = "AbstractText")]
    pub abstract_text: Option<String>,

    /// The URL associated with the abstract, if available.
    #[serde(rename = "AbstractURL")]
    pub abstract_url: Option<String>,

    /// The direct answer to the query, if available.
    #[serde(rename = "Answer")]
    pub answer: Option<String>,

    /// The type of answer provided.
    #[serde(rename = "AnswerType")]
    pub answer_type: Option<String>,

    /// The definition associated with the search result.
    #[serde(rename = "Definition")]
    pub definition: Option<String>,

    /// The source of the definition, if available.
    #[serde(rename = "DefinitionSource")]
    pub definition_source: Option<String>,

    /// The URL associated with the definition, if available.
    #[serde(rename = "DefinitionURL")]
    pub definition_url: Option<String>,

    /// The entity associated with the search result.
    #[serde(rename = "Entity")]
    pub entity: Option<String>,

    /// The heading or title of the search result.
    #[serde(rename = "Heading")]
    pub heading: Option<String>,

    /// The URL of the image associated with the search result.
    #[serde(rename = "Image")]
    pub image: Option<String>,

    /// The height of the image (may be a number or empty string).
    #[serde(rename = "ImageHeight")]
    pub image_height: Value,

    /// Indicates whether the image is a logo (may be a number or empty string).
    #[serde(rename = "ImageIsLogo")]
    pub image_is_logo: Value,

    /// The width of the image (may be a number or empty string).
    #[serde(rename = "ImageWidth")]
    pub image_width: Value,

    /// The infobox associated with the search result, if present.
    #[serde(rename = "Infobox")]
    pub info_box: Option<Value>,

    /// The redirect URL, if the result is a redirect.
    #[serde(rename = "Redirect")]
    pub redirect: Option<String>,

    /// The list of related topics.
    #[serde(rename = "RelatedTopics")]
    pub related_topics: Vec<Topic>,

    /// The list of direct result links.
    #[serde(rename = "Results")]
    pub results: Vec<Value>,

    /// The response type code (e.g. `"D"` for disambiguation, `"A"` for article).
    #[serde(rename = "Type")]
    pub r#type: String,

    /// Full metadata about the Instant Answer plugin that produced this response.
    #[serde(rename = "meta")]
    pub meta: Option<Meta>,
}

/// Enum representing different output formats for DuckDuckGo search results.
pub enum ResultFormat {
    /// Display search results in a condensed list format.
    List,
    /// Display search results with full details.
    Detailed,
}

/// Represents a single image search result from DuckDuckGo.
pub struct ImageResult {
    /// The title or description of the image.
    pub title: String,
    /// The direct URL to the full-sized image.
    pub image: String,
    /// The URL to the image thumbnail (smaller preview).
    pub thumbnail: String,
    /// The URL of the page hosting the image.
    pub url: String,
    /// The height of the image in pixels.
    pub height: u32,
    /// The width of the image in pixels.
    pub width: u32,
    /// The source or provider of the image.
    pub source: String,
}

/// Represents a single news article result from DuckDuckGo.
pub struct NewsResult {
    /// The publication date of the news article in ISO-8601 format.
    pub date: String,
    /// The headline or title of the news article.
    pub title: String,
    /// A short excerpt or summary of the news article.
    pub body: String,
    /// The URL linking to the full news article.
    pub url: String,
    /// Optional URL of an image associated with the news article.
    pub image: Option<String>,
    /// The source or publisher of the news article.
    pub source: String,
}

/// Represents a single search result from DuckDuckGo Lite search.
#[derive(Debug, Clone)]
pub struct LiteSearchResult {
    /// The title or headline of the search result.
    pub title: String,
    /// The URL linked by the search result.
    pub url: String,
    /// A short snippet or preview text from the search result.
    pub snippet: String,
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
