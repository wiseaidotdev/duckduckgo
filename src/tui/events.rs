// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # TUI Event Loop
//!
//! Drives the main event loop, dispatches keyboard input to the appropriate
//! handler, and executes DuckDuckGo searches or page navigations based on
//! user actions.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::Terminal;
use ratatui::backend::Backend;
use ratatui_image::picker::Picker;
use tui_input::backend::crossterm::EventHandler as InputHandler;

use crate::tui::app::App;
use crate::tui::renderer;
use crate::tui::settings::SettingsState;
use crate::tui::types::{ImageResult, InputMode, InstantResult, NewsResult, Tab, WebResult};
use crate::tui::ui::ui;
use crate::user_agents::get;

/// Runs the core event loop: draws each frame then processes the next keyboard
/// event until the user quits.
///
/// # Errors
/// Propagates any I/O or DuckDuckGo API error encountered during a search.
pub async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    image_picker: Option<Picker>,
) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    let mut app = App::new(image_picker);
    loop {
        if app.should_quit {
            return Ok(());
        }

        terminal.draw(|frame| ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match app.input_mode {
                InputMode::Normal => {
                    handle_normal_key(&mut app, terminal, key.code, key.modifiers).await?;
                }
                InputMode::Editing => {
                    handle_editing_key(&mut app, terminal, key.code).await?;
                }
            }
        }

        if app.selected_tab == Tab::Settings && app.settings.editing_text {
            continue;
        }
    }
}

async fn handle_normal_key<B: Backend>(
    app: &mut App,
    terminal: &mut Terminal<B>,
    code: KeyCode,
    modifiers: KeyModifiers,
) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    if app.is_page_open() {
        return handle_page_key(app, terminal, code).await;
    }

    if app.has_image_viewer() {
        return handle_image_viewer_key(app, code);
    }

    if app.selected_tab == Tab::Settings {
        return handle_settings_key(app, code);
    }

    match code {
        KeyCode::Char('q') if modifiers.is_empty() => {
            std::process::exit(0);
        }
        KeyCode::Char('e') => {
            app.input_mode = InputMode::Editing;
        }
        KeyCode::Right | KeyCode::Char('d') => {
            app.selected_tab = app.selected_tab.clone().next();
            app.scroll = 0;
            app.selected_result = 0;
        }
        KeyCode::Left | KeyCode::Char('a') => {
            app.selected_tab = app.selected_tab.clone().previous();
            app.scroll = 0;
            app.selected_result = 0;
        }
        KeyCode::Tab => {
            app.selected_tab = app.selected_tab.clone().next();
            app.scroll = 0;
            app.selected_result = 0;
        }
        KeyCode::BackTab => {
            app.selected_tab = app.selected_tab.clone().previous();
            app.scroll = 0;
            app.selected_result = 0;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.selected_result = app.selected_result.saturating_sub(1);
            app.scroll = app.scroll.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.clamp_selected_result();
            let max = result_count(app);
            if app.selected_result + 1 < max {
                app.selected_result += 1;
                app.scroll = app.scroll.saturating_add(1);
            }
        }
        KeyCode::PageUp => {
            app.scroll = app.scroll.saturating_sub(5);
            app.selected_result = app.selected_result.saturating_sub(5);
        }
        KeyCode::PageDown => {
            app.clamp_selected_result();
            let max = result_count(app);
            let step = 5;
            app.selected_result = (app.selected_result + step).min(max.saturating_sub(1));
            app.scroll = app.scroll.saturating_add(step as u16);
        }
        KeyCode::Enter => match app.selected_tab {
            Tab::Web => {
                if let Some(url) = app.selected_web_url().map(|s| s.to_string()) {
                    open_page(app, terminal, &url).await;
                }
            }
            Tab::News => {
                if let Some(url) = app.selected_news_url().map(|s| s.to_string()) {
                    open_page(app, terminal, &url).await;
                }
            }
            Tab::Images => {
                if let Some(r) = app.image_results.get(app.selected_result) {
                    let img_url = r.image_url.clone();
                    let page_url = r.page_url.clone();
                    let title = r.title.clone();
                    open_image_viewer(app, terminal, &img_url, &title, &page_url).await;
                }
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}

async fn handle_page_key<B: Backend>(
    app: &mut App,
    terminal: &mut Terminal<B>,
    code: KeyCode,
) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    match code {
        KeyCode::Backspace | KeyCode::Char('b') => {
            let back_url = app.go_back();
            if let Some(url) = back_url {
                open_page(app, terminal, &url).await;
            }
        }
        KeyCode::Char('f') => {
            let fwd_url = app.go_forward();
            if let Some(url) = fwd_url {
                open_page(app, terminal, &url).await;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.page_scroll = app.page_scroll.saturating_sub(1);
            if app.page_selected_link > 0 {
                app.page_selected_link -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.page_scroll = app.page_scroll.saturating_add(1);
            let link_count = app.page_view.as_ref().map(|p| p.links.len()).unwrap_or(0);
            if app.page_selected_link + 1 < link_count {
                app.page_selected_link += 1;
            }
        }
        KeyCode::PageUp => {
            app.page_scroll = app.page_scroll.saturating_sub(10);
        }
        KeyCode::PageDown => {
            app.page_scroll = app.page_scroll.saturating_add(10);
        }
        KeyCode::Enter => {
            let link_url = app.selected_page_link().map(|l| l.url.clone());
            if let Some(url) = link_url {
                open_page(app, terminal, &url).await;
            }
        }
        KeyCode::Esc | KeyCode::Char('q') => {
            app.close_page();
        }
        _ => {}
    }
    Ok(())
}

fn handle_settings_key(app: &mut App, code: KeyCode) -> Result<()> {
    if app.settings.editing_text {
        match code {
            KeyCode::Esc => {
                app.settings.editing_text = false;
            }
            KeyCode::Enter => {
                if let Some(text_field) = app.settings.text_row_mut(app.settings.focus_idx) {
                    *text_field = app.settings_text_input.value().to_string();
                }
                app.settings.editing_text = false;
            }
            other => {
                let ev = Event::Key(crossterm::event::KeyEvent::new(
                    other,
                    KeyModifiers::empty(),
                ));
                app.settings_text_input.handle_event(&ev);
            }
        }
        return Ok(());
    }

    match code {
        KeyCode::Tab => {
            app.selected_tab = app.selected_tab.clone().next();
        }
        KeyCode::BackTab => {
            app.selected_tab = app.selected_tab.clone().previous();
        }
        KeyCode::Esc => {
            app.selected_tab = Tab::Web;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.settings.focus_idx = app.settings.focus_idx.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('j')
            if app.settings.focus_idx + 1 < SettingsState::row_count() =>
        {
            app.settings.focus_idx += 1;
        }
        KeyCode::Right | KeyCode::Enter | KeyCode::Char(' ') => {
            let idx = app.settings.focus_idx;
            if SettingsState::is_text_row(idx) {
                let current = app
                    .settings
                    .text_row_mut(idx)
                    .map(|s| s.clone())
                    .unwrap_or_default();
                app.settings_text_input = tui_input::Input::default().with_value(current);
                app.settings.editing_text = true;
            } else {
                app.settings.cycle_row_next(idx);
            }
        }
        KeyCode::Left => {
            let idx = app.settings.focus_idx;
            if !SettingsState::is_text_row(idx) {
                app.settings.cycle_row_prev(idx);
            }
        }
        KeyCode::Char('s') => {
            let _ = app.apply_settings();
            app.status = String::from("Settings applied.");
        }
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        _ => {}
    }
    Ok(())
}

async fn handle_editing_key<B: Backend>(
    app: &mut App,
    terminal: &mut Terminal<B>,
    code: KeyCode,
) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    match code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Enter => {
            let query = app.search_input.value().to_string();
            if !query.is_empty() {
                app.status = format!("Searching for \"{}\"...", query);
                terminal.draw(|frame| ui(frame, app))?;
                app.scroll = 0;
                app.selected_result = 0;
                execute_search(app, &query).await;
            }
            app.input_mode = InputMode::Normal;
        }
        _ => {
            app.search_input
                .handle_event(&Event::Key(crossterm::event::KeyEvent::new(
                    code,
                    KeyModifiers::empty(),
                )));
        }
    }
    Ok(())
}

async fn open_page<B: Backend>(app: &mut App, terminal: &mut Terminal<B>, url: &str)
where
    B::Error: Send + Sync + 'static,
{
    app.status = format!("Loading {}...", url);
    let _ = terminal.draw(|frame| ui(frame, app));

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; ddg-tui/1.0)")
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .unwrap_or_default();

    match renderer::fetch_and_render(&client, url).await {
        Ok(page) => {
            let domain = page.domain.clone();
            let first_img = page.image_urls.first().cloned();
            app.push_page(page);
            app.status = format!("Browsing {}", domain);
            if let Some(img_url) = first_img
                && let Ok(dyn_img) = renderer::fetch_image(&client, &img_url).await
                && let Some(picker) = &mut app.image_picker
            {
                let proto = picker.new_resize_protocol(dyn_img);
                app.page_images = vec![(img_url, proto)];
            }
        }
        Err(err) => {
            app.status = format!("Failed to load page: {err}");
        }
    }
}

async fn execute_search(app: &mut App, query: &str) {
    let user_agent = get(&app.settings.user_agent_name)
        .or_else(|| get("firefox"))
        .unwrap_or("");
    let limit = Some(app.settings.max_results);

    match app.selected_tab {
        Tab::Web => {
            match app
                .browser
                .lite_search(query, app.settings.region.as_str(), limit, user_agent)
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
                    app.status = format!("{} web results for \"{}\"", app.web_results.len(), query);
                }
                Err(err) => {
                    app.web_results.clear();
                    app.status = format!("Error: {err}");
                }
            }
        }
        Tab::Images => {
            let safesearch = matches!(app.settings.safe_search, crate::params::SafeSearch::On);
            match app
                .browser
                .images(
                    query,
                    app.settings.region.as_str(),
                    safesearch,
                    limit,
                    user_agent,
                )
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
                        "{} image results for \"{}\"",
                        app.image_results.len(),
                        query
                    );
                }
                Err(err) => {
                    app.image_results.clear();
                    app.status = format!("Error: {err}");
                }
            }
        }
        Tab::News => {
            let safesearch = matches!(app.settings.safe_search, crate::params::SafeSearch::On);
            match app
                .browser
                .news(
                    query,
                    app.settings.region.as_str(),
                    safesearch,
                    limit,
                    user_agent,
                )
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
                    app.status =
                        format!("{} news results for \"{}\"", app.news_results.len(), query);
                }
                Err(err) => {
                    app.news_results.clear();
                    app.status = format!("Error: {err}");
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
                    app.status = format!("Instant answer for \"{}\"", query);
                }
                Err(err) => {
                    app.instant_result = None;
                    app.status = format!("Error: {err}");
                }
            }
        }
        Tab::Settings => {}
    }
}

fn result_count(app: &App) -> usize {
    match app.selected_tab {
        Tab::Web => app.web_results.len(),
        Tab::Images => app.image_results.len(),
        Tab::News => app.news_results.len(),
        Tab::Instant | Tab::Settings => 0,
    }
}

fn handle_image_viewer_key(app: &mut App, code: KeyCode) -> Result<()> {
    match code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Backspace | KeyCode::Char('b') => {
            app.close_image_viewer();
        }
        _ => {}
    }
    Ok(())
}

async fn open_image_viewer<B: Backend>(
    app: &mut App,
    terminal: &mut Terminal<B>,
    img_url: &str,
    title: &str,
    page_url: &str,
) where
    B::Error: Send + Sync + 'static,
{
    app.status = format!("Loading image: {}...", title);
    let _ = terminal.draw(|frame| ui(frame, app));

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; ddg-tui/1.0)")
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .unwrap_or_default();

    match renderer::fetch_image(&client, img_url).await {
        Ok(dyn_img) => {
            if let Some(picker) = &mut app.image_picker {
                let proto = picker.new_resize_protocol(dyn_img);
                app.image_viewer = Some(crate::tui::app::ImageViewerState {
                    title: title.to_string(),
                    image_url: img_url.to_string(),
                    page_url: page_url.to_string(),
                    proto,
                });
                app.status = format!("Viewing: {}", title);
            } else {
                app.status =
                    String::from("Image loaded but terminal does not support inline images.");
            }
        }
        Err(err) => {
            app.status = format!("Failed to load image: {err}");
        }
    }
}
