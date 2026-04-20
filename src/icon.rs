use serde::Deserialize;
use serde_json::Value;

/// A struct representing an icon associated with a DuckDuckGo search result.
#[derive(Debug, Deserialize)]
pub struct Icon {
    /// The height of the icon.
    #[serde(rename = "Height")]
    pub height: Value,

    /// The URL pointing to the icon image.
    #[serde(rename = "URL")]
    pub url: String,

    /// The width of the icon.
    #[serde(rename = "Width")]
    pub width: Value,
}

impl Icon {
    /// Creates a new instance of `Icon` with the specified height, URL, and width.
    ///
    /// # Arguments
    /// * `height` - The height of the icon.
    /// * `url` - The URL pointing to the icon image.
    /// * `width` - The width of the icon.
    ///
    /// # Examples
    /// ```
    /// use serde_json::Value;
    /// use duckduckgo::icon::Icon;
    ///
    /// let icon = Icon {
    ///     height: Value::Number(400.into()),
    ///     url: String::from("https://placehold.co/600x400"),
    ///     width: Value::Number(600.into()),
    /// };
    /// ```
    pub fn new(height: Value, url: String, width: Value) -> Self {
        Icon { height, url, width }
    }
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
