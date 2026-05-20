// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Page Renderer
//!
//! Converts raw HTML into [`RenderedPage`], a collection of styled
//! [`ratatui::text::Line`] values plus extracted links and image URLs,
//! suitable for display in the TUI browser view.

use anyhow::Result;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use scraper::{Html, Selector};

use crate::tui::types::{HistoryEntry, PageLink, RenderedPage};

/// Converts raw HTML bytes into a [`RenderedPage`] ready for display.
///
/// # Arguments
/// * `html` - The raw HTML document string.
/// * `page_url` - The canonical URL of the page, used to resolve relative links.
///
/// # Returns
/// A [`RenderedPage`] containing styled lines, extracted links, and image URLs.
///
/// # Errors
/// Returns an error if URL parsing fails; HTML parse errors are treated as
/// best-effort and do not propagate.
pub fn render_html(html: &str, page_url: &str) -> Result<RenderedPage> {
    let base = url::Url::parse(page_url)?;
    let domain = base.host_str().unwrap_or("unknown").to_string();

    let lines = html_to_lines(html);

    let document = Html::parse_document(html);
    let links = extract_links(&document, &base);
    let image_urls = extract_images(&document, &base);

    Ok(RenderedPage {
        url: page_url.to_string(),
        domain,
        lines,
        links,
        image_urls,
    })
}

/// Fetches a URL via `reqwest` and turns the HTML body into a [`RenderedPage`].
///
/// This is an async operation that blocks on network I/O.  It should be
/// invoked from an async context (the TUI event loop).
///
/// # Arguments
/// * `client` - A shared `reqwest::Client` to reuse connection pools.
/// * `page_url` - The URL to fetch.
///
/// # Returns
/// A [`RenderedPage`] on success.
///
/// # Errors
/// Propagates HTTP errors and parse failures.
pub async fn fetch_and_render(client: &reqwest::Client, page_url: &str) -> Result<RenderedPage> {
    let response = client
        .get(page_url)
        .header("User-Agent", "Mozilla/5.0 (compatible; ddg-tui/1.0)")
        .header("Accept", "text/html,application/xhtml+xml")
        .send()
        .await?;

    let final_url = response.url().to_string();
    let html = response.text().await?;
    render_html(&html, &final_url)
}

/// Constructs a synthetic [`HistoryEntry`] for the given URL.
pub fn make_history_entry(url: &str) -> HistoryEntry {
    HistoryEntry {
        url: url.to_string(),
    }
}

fn html_to_lines(html: &str) -> Vec<Line<'static>> {
    let text = html2text::from_read(html.as_bytes(), 120).unwrap_or_default();
    let mut lines: Vec<Line<'static>> = Vec::new();

    for raw_line in text.lines() {
        let line: String = raw_line.to_string();
        if line.starts_with('#') {
            let trimmed = line.trim_start_matches('#').trim().to_string();
            lines.push(Line::from(Span::styled(
                trimmed,
                Style::default()
                    .fg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD),
            )));
        } else if line.starts_with("**") && line.ends_with("**") {
            let inner = line.trim_matches('*').to_string();
            lines.push(Line::from(Span::styled(
                inner,
                Style::default().add_modifier(Modifier::BOLD),
            )));
        } else if line.contains("](") {
            lines.push(styled_link_line(line));
        } else if line.trim().is_empty() {
            lines.push(Line::from(""));
        } else {
            lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::White),
            )));
        }
    }

    lines
}

fn styled_link_line(line: String) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut remaining = line.as_str();

    while let Some(bracket_open) = remaining.find('[') {
        if bracket_open > 0 {
            spans.push(Span::styled(
                remaining[..bracket_open].to_string(),
                Style::default().fg(Color::White),
            ));
        }
        remaining = &remaining[bracket_open..];

        let Some(bracket_close) = remaining.find("](") else {
            break;
        };
        let link_text = remaining[1..bracket_close].to_string();

        let after_open = &remaining[bracket_close + 2..];
        let Some(paren_close) = after_open.find(')') else {
            break;
        };

        spans.push(Span::styled(
            link_text,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::UNDERLINED),
        ));

        remaining = &after_open[paren_close + 1..];
    }

    if !remaining.is_empty() {
        spans.push(Span::styled(
            remaining.to_string(),
            Style::default().fg(Color::White),
        ));
    }

    Line::from(spans)
}

fn extract_links(document: &Html, base: &url::Url) -> Vec<PageLink> {
    let selector = Selector::parse("a[href]").unwrap_or_else(|_| Selector::parse("a").unwrap());
    document
        .select(&selector)
        .filter_map(|el| {
            let href = el.value().attr("href")?;
            let absolute = base.join(href).ok()?.to_string();
            if absolute.starts_with("http://") || absolute.starts_with("https://") {
                let text = el.text().collect::<String>().trim().to_string();
                let label = if text.is_empty() {
                    absolute.clone()
                } else {
                    text
                };
                Some(PageLink {
                    text: label,
                    url: absolute,
                })
            } else {
                None
            }
        })
        .collect()
}

fn extract_images(document: &Html, base: &url::Url) -> Vec<String> {
    let selector = Selector::parse("img[src]").unwrap_or_else(|_| Selector::parse("img").unwrap());
    document
        .select(&selector)
        .filter_map(|el| {
            let src = el.value().attr("src")?;
            let absolute = base.join(src).ok()?.to_string();
            if absolute.starts_with("http://") || absolute.starts_with("https://") {
                Some(absolute)
            } else {
                None
            }
        })
        .take(10)
        .collect()
}

/// Downloads an image from `url` and decodes it into a [`image::DynamicImage`].
///
/// # Errors
/// Propagates HTTP and image decode errors.
pub async fn fetch_image(client: &reqwest::Client, url: &str) -> Result<image::DynamicImage> {
    let bytes = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (compatible; ddg-tui/1.0)")
        .header("Accept", "image/*,*/*")
        .send()
        .await?
        .bytes()
        .await?;
    Ok(image::load_from_memory(&bytes)?)
}
