// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Wikipedia Fallback
//!
//! This module provides a fallback search mechanism using Wikipedia's public
//! APIs. It is primarily used in WebAssembly environments where direct
//! DuckDuckGo API access may be restricted by CORS policies.
//!
//! The fallback uses:
//! 1. **OpenSearch API**: To find the best-matching Wikipedia article title.
//! 2. **Extracts API**: To fetch a plain-text summary of the matched article.

use crate::response::Response;
use crate::topic::Topic;

/// Wikipedia API endpoint for OpenSearch and extracts.
pub const WIKIPEDIA_API: &str = "https://en.wikipedia.org/w/api.php";

/// Queries Wikipedia using its public API (which supports `origin=*`) and
/// maps the result to a DuckDuckGo-compatible [`Response`].
///
/// Because Wikipedia's API always returns `Access-Control-Allow-Origin: *`
/// when `&origin=*` is included, this call succeeds in **any** browser
/// environment, including local development and production deployments.
pub async fn search(client: &reqwest::Client, query: &str) -> anyhow::Result<Response> {
    let search_url = format!(
        "{}?action=opensearch&search={}&limit=1&namespace=0&format=json&origin=*",
        WIKIPEDIA_API,
        percent_encode_url(query)
    );

    let search_text = client.get(&search_url).send().await?.text().await?;

    let search_json: serde_json::Value = serde_json::from_str(&search_text)?;
    let title = search_json
        .get(1)
        .and_then(|t| t.get(0))
        .and_then(|t| t.as_str())
        .unwrap_or(query)
        .to_string();

    let wiki_url = search_json
        .get(3)
        .and_then(|u| u.get(0))
        .and_then(|u| u.as_str())
        .unwrap_or("")
        .to_string();

    let extract_url = format!(
        "{}?action=query&prop=extracts&exintro=1&explaintext=1&exsentences=6\
         &titles={}&format=json&origin=*",
        WIKIPEDIA_API,
        percent_encode_url(&title)
    );

    let extract_text = client.get(&extract_url).send().await?.text().await?;

    let extract_json: serde_json::Value = serde_json::from_str(&extract_text)?;
    let extract = extract_json
        .pointer("/query/pages")
        .and_then(|pages| pages.as_object())
        .and_then(|pages| pages.values().next())
        .and_then(|page| page.get("extract"))
        .and_then(|e| e.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    if extract.is_empty() {
        anyhow::bail!("Wikipedia returned no extract for '{}'", title);
    }

    let topic = Topic {
        first_url: Some(wiki_url.clone()),
        text: Some(format!("{}, Wikipedia", title)),
        icon: None,
        result: None,
        url: Some(wiki_url.clone()),
    };

    Ok(Response {
        r#abstract: Some(extract.clone()),
        abstract_source: Some("Wikipedia".to_string()),
        abstract_text: Some(extract),
        abstract_url: Some(wiki_url),
        answer: None,
        answer_type: None,
        definition: None,
        definition_source: None,
        definition_url: None,
        entity: None,
        heading: Some(title),
        image: None,
        image_height: serde_json::Value::String(String::new()),
        image_is_logo: serde_json::Value::String(String::new()),
        image_width: serde_json::Value::String(String::new()),
        info_box: None,
        redirect: None,
        related_topics: vec![topic],
        results: vec![],
        r#type: "A".to_string(),
        meta: None,
    })
}

/// Percent-encodes a URL string so it can be safely embedded as a query
/// parameter value.
///
/// All bytes except ASCII unreserved characters (`A-Z a-z 0-9 - _ . ~`) are
/// encoded as `%XX`.
pub fn percent_encode_url(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            b => {
                out.push('%');
                out.push(
                    char::from_digit((b >> 4) as u32, 16)
                        .unwrap_or('0')
                        .to_ascii_uppercase(),
                );
                out.push(
                    char::from_digit((b & 0xf) as u32, 16)
                        .unwrap_or('0')
                        .to_ascii_uppercase(),
                );
            }
        }
    }
    out
}

/// Decodes a percent-encoded string (e.g. `What%20is%20Rust` → `What is Rust`).
/// Unknown or invalid sequences are passed through as-is.
pub fn percent_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hi = (bytes[i + 1] as char).to_digit(16);
            let lo = (bytes[i + 2] as char).to_digit(16);
            if let (Some(h), Some(l)) = (hi, lo) {
                out.push((((h << 4) | l) as u8) as char);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            out.push(' ');
            i += 1;
            continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}
