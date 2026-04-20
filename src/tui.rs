// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # TUI Module
//!
//! Provides the full-screen interactive terminal interface for querying
//! DuckDuckGo from the command line. The TUI is modelled after the
//! four main DuckDuckGo search backends:
//!
//! | Tab | Backend |
//! |-----|---------|
//! | 🔍 Web | [`crate::browser::Browser::lite_search`] |
//! | 🖼️ Images | [`crate::browser::Browser::images`] |
//! | 📰 News | [`crate::browser::Browser::news`] |
//! | ⚡ Instant | [`crate::browser::Browser::get_api_response`] |
//!
//! ## Key Bindings
//!
//! | Key | Action |
//! |-----|--------|
//! | `e` | Enter editing mode to type a search query |
//! | `Enter` | Execute the search for the active tab |
//! | `Esc` | Return to Normal mode |
//! | `◄ ► / a d` | Switch between search tabs |
//! | `↑ ↓ / j k` | Scroll results |
//! | `PgUp / PgDn` | Scroll results faster |
//! | `q` | Quit the TUI |
//!
//! ## See Also
//!
//! - [DuckDuckGo Lite](https://lite.duckduckgo.com/lite/)
//! - [DuckDuckGo Images API](https://duckduckgo.com/i.js)
//! - [DuckDuckGo News API](https://duckduckgo.com/news.js)
//! - [DuckDuckGo Instant Answer API](https://duckduckgo.com/duckduckgo-help-pages/open-source/instant-answer-interface/)

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Constraint::Max, Position, Stylize},
    style::{Color, Modifier, Style, palette::tailwind},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
};
use strum::IntoEnumIterator;
use strum_macros::{Display as DeriveDisplay, EnumIter as DeriveEnumIter};
use tui_input::{Input, backend::crossterm::EventHandler as InputHandler};

use crate::browser::Browser;
use crate::user_agents::get;
use std::io;

/// Selectable tab within the TUI, corresponding to a DuckDuckGo search backend.
#[derive(Debug, Clone, DeriveEnumIter, DeriveDisplay, PartialEq)]
enum Tab {
    #[strum(to_string = "🔍 Web")]
    Web,
    #[strum(to_string = "🖼️  Images")]
    Images,
    #[strum(to_string = "📰 News")]
    News,
    #[strum(to_string = "⚡ Instant")]
    Instant,
}

impl Tab {
    /// Returns the tab that follows this one, wrapping around.
    fn next(self) -> Self {
        let mut iter = Tab::iter().cycle();
        while let Some(tab) = iter.next() {
            if tab == self {
                return iter.next().unwrap_or(self);
            }
        }
        self
    }

    /// Returns the tab that precedes this one, wrapping around.
    fn previous(self) -> Self {
        let tabs: Vec<_> = Tab::iter().collect();
        let idx = tabs.iter().position(|t| *t == self).unwrap_or(0);
        if idx == 0 {
            tabs.last().cloned().unwrap_or(self)
        } else {
            tabs[idx - 1].clone()
        }
    }
}

/// Whether the terminal is in search-bar editing mode or normal navigation mode.
#[derive(Debug, Clone, PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

/// A single web search result from the DuckDuckGo Lite endpoint.
#[derive(Debug, Clone)]
struct WebResult {
    title: String,
    url: String,
    snippet: String,
}

/// A single image result from the DuckDuckGo Images endpoint.
#[derive(Debug, Clone)]
struct ImageResult {
    title: String,
    page_url: String,
    image_url: String,
    source: String,
}

/// A single news article result from the DuckDuckGo News endpoint.
#[derive(Debug, Clone)]
struct NewsResult {
    date: String,
    title: String,
    url: String,
    source: String,
    body: String,
}

/// Structured data from the DuckDuckGo Instant Answer API.
#[derive(Debug, Clone, Default)]
struct InstantResult {
    heading: String,
    abstract_text: String,
    abstract_source: String,
    abstract_url: String,
    answer: String,
    definition: String,
    entity: String,
    result_type: String,
}

/// Full application state for the TUI event loop.
struct App {
    input_mode: InputMode,
    selected_tab: Tab,
    search_input: Input,
    browser: Browser,
    web_results: Vec<WebResult>,
    image_results: Vec<ImageResult>,
    news_results: Vec<NewsResult>,
    instant_result: Option<InstantResult>,
    selected_result: usize,
    scroll: u16,
    status: String,
}

impl Default for App {
    fn default() -> Self {
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
                "Ready  -  Press 'e' to type a query, Tab to switch backends, 'q' to quit",
            ),
        }
    }
}

/// Initialises the terminal, runs the TUI event loop, and restores the terminal on exit.
///
/// # Errors
///
/// Returns an [`anyhow::Error`] if terminal setup or any crossterm operation fails.
pub async fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = run_app(&mut terminal).await;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = result {
        eprintln!("TUI error: {err:?}");
    }
    Ok(())
}

/// Core event loop: draws each frame then processes the next keyboard event.
///
/// The loop continues until the user presses `q` in Normal mode.
///
/// # Errors
///
/// Propagates any I/O or DuckDuckGo API error that occurs during a search.
async fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::default();
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        app.selected_tab = app.selected_tab.next();
                        app.scroll = 0;
                        app.selected_result = 0;
                    }
                    KeyCode::Left | KeyCode::Char('a') => {
                        app.selected_tab = app.selected_tab.previous();
                        app.scroll = 0;
                        app.selected_result = 0;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.selected_result = app.selected_result.saturating_sub(1);
                        app.scroll = app.scroll.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.selected_result = app.selected_result.saturating_add(1);
                        app.scroll = app.scroll.saturating_add(1);
                    }
                    KeyCode::PageUp => {
                        app.scroll = app.scroll.saturating_sub(5);
                        app.selected_result = app.selected_result.saturating_sub(5);
                    }
                    KeyCode::PageDown => {
                        app.scroll = app.scroll.saturating_add(5);
                        app.selected_result = app.selected_result.saturating_add(5);
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Enter => {
                        let query = app.search_input.value().to_string();
                        if !query.is_empty() {
                            app.status = format!("\u{1F504} Searching for \"{}\"\u{2026}", query);
                            terminal.draw(|frame| ui(frame, &mut app))?;
                            app.scroll = 0;
                            app.selected_result = 0;
                            execute_search(&mut app, &query).await;
                        }
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {
                        app.search_input.handle_event(&Event::Key(key));
                    }
                },
            }
        }
    }
    Ok(())
}

/// Dispatches a search to the appropriate backend based on [`App::selected_tab`]
/// and stores the results back into `app`.
async fn execute_search(app: &mut App, query: &str) {
    let user_agent = get("firefox").unwrap_or("");
    match app.selected_tab {
        Tab::Web => match app
            .browser
            .lite_search(query, "wt-wt", Some(20), user_agent)
            .await
        {
            Ok(results) => {
                app.web_results = results
                    .into_iter()
                    .map(|r| WebResult {
                        title: r.title,
                        url: r.url,
                        snippet: r.snippet,
                    })
                    .collect();
                app.status = format!(
                    "\u{2705} {} web results for \"{}\"",
                    app.web_results.len(),
                    query
                );
            }
            Err(err) => {
                app.web_results.clear();
                app.status = format!("❌ {err}");
            }
        },
        Tab::Images => {
            match app
                .browser
                .images(query, "wt-wt", true, Some(20), user_agent)
                .await
            {
                Ok(results) => {
                    app.image_results = results
                        .into_iter()
                        .map(|r| ImageResult {
                            title: r.title,
                            page_url: r.url,
                            image_url: r.image,
                            source: r.source,
                        })
                        .collect();
                    app.status = format!(
                        "\u{2705} {} image results for \"{}\"",
                        app.image_results.len(),
                        query
                    );
                }
                Err(err) => {
                    app.image_results.clear();
                    app.status = format!("❌ {err}");
                }
            }
        }
        Tab::News => {
            match app
                .browser
                .news(query, "wt-wt", true, Some(20), user_agent)
                .await
            {
                Ok(results) => {
                    app.news_results = results
                        .into_iter()
                        .map(|r| NewsResult {
                            date: r.date,
                            title: r.title,
                            url: r.url,
                            source: r.source,
                            body: r.body,
                        })
                        .collect();
                    app.status = format!(
                        "\u{2705} {} news results for \"{}\"",
                        app.news_results.len(),
                        query
                    );
                }
                Err(err) => {
                    app.news_results.clear();
                    app.status = format!("❌ {err}");
                }
            }
        }
        Tab::Instant => {
            let path = format!("?q={}", urlencoding::encode(query));
            match app.browser.get_api_response(&path, None).await {
                Ok(resp) => {
                    app.instant_result = Some(InstantResult {
                        heading: resp.heading.unwrap_or_default(),
                        abstract_text: resp.abstract_text.unwrap_or_default(),
                        abstract_source: resp.abstract_source.unwrap_or_default(),
                        abstract_url: resp.abstract_url.unwrap_or_default(),
                        answer: resp.answer.unwrap_or_default(),
                        definition: resp.definition.unwrap_or_default(),
                        entity: resp.entity.unwrap_or_default(),
                        result_type: resp.r#type,
                    });
                    app.status = format!("\u{2705} Instant answer for \"{}\"", query);
                }
                Err(err) => {
                    app.instant_result = None;
                    app.status = format!("❌ {err}");
                }
            }
        }
    }
}

/// Renders the complete TUI for one frame.
///
/// The layout is split into four rows: tab bar, search bar, results area,
/// and a two-line footer.
fn ui(frame: &mut Frame, app: &mut App) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_tabs(frame, app, vertical[0]);
    render_search_bar(frame, app, vertical[1]);

    match app.selected_tab {
        Tab::Web => render_web_results(frame, app, vertical[2]),
        Tab::Images => render_image_results(frame, app, vertical[2]),
        Tab::News => render_news_results(frame, app, vertical[2]),
        Tab::Instant => render_instant_result(frame, app, vertical[2]),
    }

    render_footer(frame, app, vertical[3]);
}

/// Renders the row of tabs at the top of the TUI.
fn render_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = Tab::iter()
        .map(|t| {
            Line::from(Span::styled(
                t.to_string(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
        })
        .collect();

    let selected = app.selected_tab.clone() as usize;
    let tabs = Tabs::new(titles)
        .select(selected)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 🦆 DuckDuckGo  "),
        )
        .highlight_style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        );
    frame.render_widget(tabs, area);
}

/// Renders the search-bar input field and moves the terminal cursor when in Editing mode.
fn render_search_bar(frame: &mut Frame, app: &App, area: Rect) {
    let scroll_offset = app.search_input.visual_scroll(area.width as usize);

    let border_style = match app.input_mode {
        InputMode::Editing => Style::default().fg(Color::LightYellow),
        InputMode::Normal => Style::default().fg(Color::DarkGray),
    };

    let text_style = match app.input_mode {
        InputMode::Editing => Style::default().fg(Color::LightYellow),
        InputMode::Normal => Style::default().fg(Color::White),
    };

    let bar = Paragraph::new(app.search_input.value())
        .style(text_style)
        .scroll((0, scroll_offset as u16))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(" 🔍 Search  "),
        );
    frame.render_widget(bar, area);

    if app.input_mode == InputMode::Editing {
        let cursor_x = area.x
            + ((app.search_input.visual_cursor()).max(scroll_offset) - scroll_offset) as u16
            + 1;
        let cursor_y = area.y + 1;
        frame.set_cursor_position(Position::new(cursor_x, cursor_y));
    }
}

/// Renders the web (Lite) search results panel.
fn render_web_results(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 🔍 Web Results ({}) ", app.web_results.len()));

    if app.web_results.is_empty() {
        let placeholder = Paragraph::new(empty_hint("🔍", "web results"))
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(placeholder, area);
        return;
    }

    let separator = "─".repeat(area.width.saturating_sub(6) as usize);
    let lines: Vec<Line> = app
        .web_results
        .iter()
        .enumerate()
        .flat_map(|(idx, r)| {
            let selected = idx == app.selected_result;
            let title_style = result_title_style(Color::Blue, Color::LightBlue, selected);
            vec![
                Line::from(Span::styled(format!("  {}", r.title), title_style)),
                Line::from(Span::styled(
                    format!("  🌐 {}", r.url),
                    Style::default().fg(Color::Green),
                )),
                Line::from(Span::styled(
                    format!("  {}", r.snippet),
                    Style::default().fg(Color::Gray),
                )),
                Line::from(Span::styled(
                    format!("  {separator}"),
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""),
            ]
        })
        .collect();

    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));
    frame.render_widget(para, area);
}

/// Renders the image search results panel.
fn render_image_results(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 🖼️  Image Results ({}) ", app.image_results.len()));

    if app.image_results.is_empty() {
        let placeholder = Paragraph::new(empty_hint("🖼️", "image results"))
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(placeholder, area);
        return;
    }

    let separator = "─".repeat(area.width.saturating_sub(6) as usize);
    let lines: Vec<Line> = app
        .image_results
        .iter()
        .enumerate()
        .flat_map(|(idx, r)| {
            let selected = idx == app.selected_result;
            let title_style = result_title_style(Color::Magenta, Color::LightMagenta, selected);
            vec![
                Line::from(Span::styled(format!("  🖼️  {}", r.title), title_style)),
                Line::from(Span::styled(
                    format!("  🌐 Page: {}", r.page_url),
                    Style::default().fg(Color::Green),
                )),
                Line::from(Span::styled(
                    format!("  🔗 Image: {}", r.image_url),
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(Span::styled(
                    format!("  📌 Source: {}", r.source),
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(Span::styled(
                    format!("  {separator}"),
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""),
            ]
        })
        .collect();

    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));
    frame.render_widget(para, area);
}

/// Renders the news search results panel.
fn render_news_results(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 📰 News Results ({}) ", app.news_results.len()));

    if app.news_results.is_empty() {
        let placeholder = Paragraph::new(empty_hint("📰", "news results"))
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(placeholder, area);
        return;
    }

    let separator = "─".repeat(area.width.saturating_sub(6) as usize);
    let lines: Vec<Line> = app
        .news_results
        .iter()
        .enumerate()
        .flat_map(|(idx, r)| {
            let selected = idx == app.selected_result;
            let title_style = result_title_style(Color::Red, Color::LightRed, selected);
            vec![
                Line::from(Span::styled(format!("  📰 {}", r.title), title_style)),
                Line::from(Span::styled(
                    format!("  📅 {}  •  📌 {}", r.date, r.source),
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(Span::styled(
                    format!("  🌐 {}", r.url),
                    Style::default().fg(Color::Green),
                )),
                Line::from(Span::styled(
                    format!("  {}", r.body),
                    Style::default().fg(Color::Gray),
                )),
                Line::from(Span::styled(
                    format!("  {separator}"),
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""),
            ]
        })
        .collect();

    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));
    frame.render_widget(para, area);
}

/// Renders the Instant Answer API result panel.
fn render_instant_result(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" ⚡ Instant Answer ");

    let Some(result) = &app.instant_result else {
        let placeholder = Paragraph::new(empty_hint("⚡", "an instant answer"))
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(placeholder, area);
        return;
    };

    let mut lines = vec![Line::from("")];

    if !result.heading.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  📌 {}", result.heading),
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )));
        if !result.entity.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("  🏷️  {}", result.entity),
                Style::default().fg(Color::DarkGray),
            )));
        }
        lines.push(Line::from(""));
    }

    if !result.answer.is_empty() {
        lines.push(section_header("⚡", "Answer", Color::Yellow));
        lines.push(Line::from(Span::styled(
            format!("  {}", result.answer),
            Style::default().fg(Color::White),
        )));
        lines.push(Line::from(""));
    }

    if !result.abstract_text.is_empty() {
        lines.push(section_header("📖", "Summary", Color::Cyan));
        for part in result.abstract_text.split(". ") {
            lines.push(Line::from(Span::styled(
                format!("  {}.", part),
                Style::default().fg(Color::Gray),
            )));
        }
        lines.push(Line::from(""));
    }

    if !result.definition.is_empty() {
        lines.push(section_header("📚", "Definition", Color::Green));
        lines.push(Line::from(Span::styled(
            format!("  {}", result.definition),
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(""));
    }

    if !result.abstract_source.is_empty() {
        lines.push(Line::from(Span::styled(
            format!(
                "  🌐 Source: {}  -  {}",
                result.abstract_source, result.abstract_url
            ),
            Style::default().fg(Color::Green),
        )));
    }

    if !result.result_type.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  🏷️  Type: {}", result.result_type),
            Style::default().fg(Color::DarkGray),
        )));
    }

    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));
    frame.render_widget(para, area);
}

/// Renders the two-line footer containing key-binding hints and a status bar.
fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::new(Direction::Vertical, [Max(1), Max(1), Max(1)]).split(area);

    let keybinds = Line::raw(
        "◄ ► / a d: switch tabs  │  ↑ ↓ / j k: scroll  │  e: edit  │  Enter: search  │  Esc: cancel  │  q: quit",
    )
    .centered();

    let status_bar = Line::from(vec![
        Span::styled(
            "  🦆 ddg  ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightYellow)
                .bold(),
        ),
        Span::raw("  "),
        Span::styled(app.status.as_str(), Style::default().fg(Color::White)),
    ])
    .bg(tailwind::SLATE.c700);

    frame.render_widget(keybinds, rows[0]);
    frame.render_widget(status_bar, rows[2]);
}

/// Returns the title [`Style`] for a result entry, brighter when the result is selected.
fn result_title_style(normal: Color, selected: Color, is_selected: bool) -> Style {
    if is_selected {
        Style::default()
            .fg(selected)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    } else {
        Style::default().fg(normal).add_modifier(Modifier::BOLD)
    }
}

/// Returns a styled section-header [`Line`] used inside the Instant Answer panel.
fn section_header(icon: &str, label: &str, color: Color) -> Line<'static> {
    Line::from(Span::styled(
        format!("  {icon} {label}:"),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
}

/// Returns the empty-state hint shown when no results have been fetched yet.
fn empty_hint(icon: &str, kind: &str) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {icon}  Press 'e', type a query and press Enter to find {kind}."),
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Tip: use ◄ ► or a / d to switch between Web, Images, News, and Instant tabs.",
            Style::default().fg(Color::DarkGray),
        )),
    ]
}

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
