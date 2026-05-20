// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # TUI UI Rendering
//!
//! All frame-level render functions. Each function is responsible for a
//! single panel or widget, keeping rendering logic separated from state and
//! event-handling concerns.

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Constraint::Max, Position, Stylize};
use ratatui::style::{Color, Modifier, Style, palette::tailwind};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    Tabs, Wrap,
};
use ratatui_image::StatefulImage;
use strum::IntoEnumIterator;

use crate::tui::app::App;
use crate::tui::emoji::safe_str;
use crate::tui::settings::SettingsState;
use crate::tui::types::{InputMode, RenderedPage, Tab};

/// Renders the entire TUI for one frame, routing to the appropriate sub-render
/// based on whether a page is open or which tab is active.
pub fn ui(frame: &mut Frame, app: &mut App) {
    if app.has_image_viewer() {
        render_image_viewer(frame, app);
        return;
    }

    if app.is_page_open() {
        render_page_view_layout(frame, app);
        return;
    }

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
        Tab::Settings => render_settings_tab(frame, app, vertical[2]),
    }

    render_footer(frame, app, vertical[3]);
}

fn render_page_view_layout(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(area);

    render_page_domain_bar(frame, app, vertical[0]);
    render_page_url_bar(frame, app, vertical[1]);
    render_page_view(frame, app, vertical[2]);
    render_page_footer(frame, app, vertical[3]);
}

fn render_page_domain_bar(frame: &mut Frame, app: &App, area: Rect) {
    let domain = app
        .page_view
        .as_ref()
        .map(|p| p.domain.as_str())
        .unwrap_or("");
    let label = format!("  {}  {}  {}", safe_str("🌐"), domain, safe_str("🌐"));
    let line = Line::from(Span::styled(
        label,
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightCyan)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);
    frame.render_widget(line, area);
}

fn render_page_url_bar(frame: &mut Frame, app: &App, area: Rect) {
    let url = app.page_view.as_ref().map(|p| p.url.as_str()).unwrap_or("");
    let line = Line::from(vec![
        Span::styled("  URL: ", Style::default().fg(Color::DarkGray)),
        Span::styled(url, Style::default().fg(Color::Gray)),
    ]);
    frame.render_widget(line, area);
}

fn render_page_view(frame: &mut Frame, app: &mut App, area: Rect) {
    let Some(page) = app.page_view.as_ref() else {
        return;
    };

    let has_images = !page.image_urls.is_empty()
        && app
            .page_images
            .iter()
            .any(|(u, _)| page.image_urls.first().map(|f| f == u).unwrap_or(false));
    let links = page.links.clone();
    let lines = enrich_page_lines_with_link_highlights(page, app.page_selected_link);

    let (content_area, image_area) = if has_images {
        let horiz = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(1), Constraint::Length(32)])
            .split(area);
        (horiz[0], Some(horiz[1]))
    } else {
        (area, None)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(format!(" {} Page ({} links) ", safe_str("📄"), links.len()));

    let total_lines = lines.len() as u16;
    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.page_scroll, 0));
    frame.render_widget(para, content_area);

    let visible_height = content_area.height.saturating_sub(2);
    if total_lines > visible_height {
        let mut scrollbar_state =
            ScrollbarState::new(total_lines as usize).position(app.page_scroll as usize);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        frame.render_stateful_widget(scrollbar, content_area, &mut scrollbar_state);
    }

    if let (Some(img_area), Some(picker)) = (image_area, app.image_picker.as_mut()) {
        let image_url = app
            .page_view
            .as_ref()
            .and_then(|p| p.image_urls.first())
            .cloned();

        if let Some(url) = image_url {
            let proto_opt = app.page_images.iter_mut().find(|(u, _)| u == &url);
            if let Some((_, proto)) = proto_opt {
                let img_widget = StatefulImage::default();
                frame.render_stateful_widget(img_widget, img_area, proto);
            } else {
                let placeholder = Paragraph::new(Line::from(Span::styled(
                    format!(" {} Loading image...", safe_str("🖼️")),
                    Style::default().fg(Color::DarkGray),
                )));
                frame.render_widget(placeholder, img_area);
                let _ = (picker, url);
            }
        }
    }
}

fn enrich_page_lines_with_link_highlights(
    page: &RenderedPage,
    selected_link: usize,
) -> Vec<Line<'static>> {
    let mut lines = page.lines.clone();
    for (idx, link) in page.links.iter().enumerate() {
        let label = if idx == selected_link {
            format!("  {} [{}] {}", safe_str("▶"), idx + 1, link.text)
        } else {
            format!("  [{}] {}", idx + 1, link.text)
        };
        let style = if idx == selected_link {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::UNDERLINED)
        };
        lines.push(Line::from(Span::styled(label, style)));
    }
    lines
}

fn render_page_footer(frame: &mut Frame, app: &App, area: Rect) {
    let history_info = format!(
        "History: {}  Forward: {}",
        app.history.len(),
        app.history_forward.len()
    );
    let rows = Layout::new(Direction::Vertical, [Max(1), Max(1)]).split(area);

    let keybinds = Line::raw(
        "Backspace/b: back  |  f: forward  |  j/k: scroll  |  Enter: open link  |  Esc/q: close page",
    )
    .centered()
    .style(Style::default().fg(Color::DarkGray));

    let status_bar = Line::from(vec![
        Span::styled(
            format!("  {} ddg  ", safe_str("🦆")),
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightYellow)
                .bold(),
        ),
        Span::raw("  "),
        Span::styled(
            format!("{}  |  {}", app.status, history_info),
            Style::default().fg(Color::White),
        ),
    ])
    .bg(tailwind::SLATE.c700);

    frame.render_widget(keybinds, rows[0]);
    frame.render_widget(status_bar, rows[1]);
}

fn render_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = Tab::iter()
        .map(|t| {
            Line::from(Span::styled(
                format!("  {t}  "),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
        })
        .collect();

    let selected = Tab::iter().position(|t| t == app.selected_tab).unwrap_or(0);

    let tabs = Tabs::new(titles)
        .select(selected)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("  {}  DuckDuckGo  ", safe_str("🦆"))),
        )
        .highlight_style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        );
    frame.render_widget(tabs, area);
}

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
                .title(format!("  {}  Search  ", safe_str("🔍"))),
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

fn render_web_results(frame: &mut Frame, app: &App, area: Rect) {
    let title = format!(
        "  {}  Web Results ({})  ",
        safe_str("🔍"),
        app.web_results.len()
    );
    let block = Block::default().borders(Borders::ALL).title(title);

    if app.web_results.is_empty() {
        let placeholder = Paragraph::new(empty_hint(&safe_str("🔍"), "web results"))
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
                    format!("  {} {}", safe_str("🌐"), r.url),
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

fn render_image_results(frame: &mut Frame, app: &App, area: Rect) {
    let title = format!(
        "  {}  Image Results ({})  ",
        safe_str("🖼️"),
        app.image_results.len()
    );
    let block = Block::default().borders(Borders::ALL).title(title);

    if app.image_results.is_empty() {
        let placeholder = Paragraph::new(empty_hint(&safe_str("🖼️"), "image results"))
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
                Line::from(Span::styled(
                    format!("  {}  {}", safe_str("🖼️"), r.title),
                    title_style,
                )),
                Line::from(Span::styled(
                    format!("  {} Page: {}", safe_str("🌐"), r.page_url),
                    Style::default().fg(Color::Green),
                )),
                Line::from(Span::styled(
                    format!("  {} Image: {}", safe_str("🔗"), r.image_url),
                    Style::default().fg(Color::Cyan),
                )),
                Line::from(Span::styled(
                    format!("  {} Source: {}", safe_str("📌"), r.source),
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

fn render_news_results(frame: &mut Frame, app: &App, area: Rect) {
    let title = format!(
        "  {}  News Results ({})  ",
        safe_str("📰"),
        app.news_results.len()
    );
    let block = Block::default().borders(Borders::ALL).title(title);

    if app.news_results.is_empty() {
        let placeholder = Paragraph::new(empty_hint(&safe_str("📰"), "news results"))
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
                Line::from(Span::styled(
                    format!("  {} {}", safe_str("📰"), r.title),
                    title_style,
                )),
                Line::from(Span::styled(
                    format!(
                        "  {}  {}  •  {} {}",
                        safe_str("📅"),
                        r.date,
                        safe_str("📌"),
                        r.source
                    ),
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(Span::styled(
                    format!("  {} {}", safe_str("🌐"), r.url),
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

fn render_instant_result(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("  {}  Instant Answer  ", safe_str("⚡")));

    let Some(result) = &app.instant_result else {
        let placeholder = Paragraph::new(empty_hint(&safe_str("⚡"), "an instant answer"))
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(placeholder, area);
        return;
    };

    let mut lines = vec![Line::from("")];

    if !result.heading.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  {} {}", safe_str("📌"), result.heading),
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )));
        if !result.entity.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("  {}  {}", safe_str("🏷️"), result.entity),
                Style::default().fg(Color::DarkGray),
            )));
        }
        lines.push(Line::from(""));
    }

    if !result.answer.is_empty() {
        lines.push(section_header(&safe_str("⚡"), "Answer", Color::Yellow));
        lines.push(Line::from(Span::styled(
            format!("  {}", result.answer),
            Style::default().fg(Color::White),
        )));
        lines.push(Line::from(""));
    }

    if !result.abstract_text.is_empty() {
        lines.push(section_header(&safe_str("📖"), "Summary", Color::Cyan));
        for part in result.abstract_text.split(". ") {
            lines.push(Line::from(Span::styled(
                format!("  {}.", part),
                Style::default().fg(Color::Gray),
            )));
        }
        lines.push(Line::from(""));
    }

    if !result.definition.is_empty() {
        lines.push(section_header(&safe_str("📚"), "Definition", Color::Green));
        lines.push(Line::from(Span::styled(
            format!("  {}", result.definition),
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(""));
    }

    if !result.abstract_source.is_empty() {
        lines.push(Line::from(Span::styled(
            format!(
                "  {} Source: {}  -  {}",
                safe_str("🌐"),
                result.abstract_source,
                result.abstract_url
            ),
            Style::default().fg(Color::Green),
        )));
    }

    if !result.result_type.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("  {}  Type: {}", safe_str("🏷️"), result.result_type),
            Style::default().fg(Color::DarkGray),
        )));
    }

    let para = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0));
    frame.render_widget(para, area);
}

fn render_settings_tab(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(format!(
        "  {}  Settings  (Enter/Space: toggle · Left/Right: cycle · 's': apply)  ",
        safe_str("⚙️")
    ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let row_count = SettingsState::row_count();
    let visible_height = inner.height as usize;
    let scroll_offset = if app.settings.focus_idx >= visible_height {
        app.settings.focus_idx.saturating_sub(visible_height - 1)
    } else {
        0
    };

    let items: Vec<ListItem> = (scroll_offset..row_count)
        .map(|idx| {
            let (label, value, _, is_text) = app.settings.row_info(idx);
            let selected = idx == app.settings.focus_idx;

            let editing_this = selected && app.settings.editing_text && is_text;
            let display_value = if editing_this {
                format!("[{}]", app.settings_text_input.value())
            } else {
                value
            };

            let row_text = format!("  {:<22} {}", label, display_value);

            let style = if selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::LightYellow)
                    .add_modifier(Modifier::BOLD)
            } else if idx % 2 == 0 {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::Gray)
            };

            ListItem::new(Line::from(Span::styled(row_text, style)))
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);

    let mut scrollbar_state = ScrollbarState::new(row_count).position(app.settings.focus_idx);
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::new(Direction::Vertical, [Max(1), Max(1), Max(1)]).split(area);

    let keybinds = Line::raw(
        "Tab/a/d: switch  |  j/k/arrows: scroll  |  e: edit  |  Enter: search/open  |  Backspace: back  |  q: quit",
    )
    .centered()
    .style(Style::default().fg(Color::DarkGray));

    let status_bar = Line::from(vec![
        Span::styled(
            format!("  {}  ddg  ", safe_str("🦆")),
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

fn result_title_style(normal: Color, selected: Color, is_selected: bool) -> Style {
    if is_selected {
        Style::default()
            .fg(selected)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    } else {
        Style::default().fg(normal).add_modifier(Modifier::BOLD)
    }
}

fn section_header(icon: &str, label: &str, color: Color) -> Line<'static> {
    Line::from(Span::styled(
        format!("  {icon} {label}:"),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
}

fn empty_hint(icon: &str, kind: &str) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {icon}  Press 'e', type a query and press Enter to find {kind}."),
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Tip: use Tab or a / d to switch between Web, Images, News, Instant, and Settings.",
            Style::default().fg(Color::DarkGray),
        )),
    ]
}

fn render_image_viewer(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    let title_bar_text = app
        .image_viewer
        .as_ref()
        .map(|v| format!("  {}  {}  ", safe_str("🖼️"), v.title))
        .unwrap_or_default();

    let title_bar = Line::from(Span::styled(
        title_bar_text,
        Style::default()
            .fg(Color::Black)
            .bg(Color::LightMagenta)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);
    frame.render_widget(title_bar, vertical[0]);

    let url_text = app
        .image_viewer
        .as_ref()
        .map(|v| format!("  Source: {}", v.page_url))
        .unwrap_or_default();
    let url_bar = Line::from(Span::styled(url_text, Style::default().fg(Color::DarkGray)));
    frame.render_widget(url_bar, vertical[1]);

    let image_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta))
        .title(format!("  {}  Image  ", safe_str("🖼️")));

    let inner = image_block.inner(vertical[2]);
    frame.render_widget(image_block, vertical[2]);

    if let Some(viewer) = &mut app.image_viewer {
        let img_widget = StatefulImage::default();
        frame.render_stateful_widget(img_widget, inner, &mut viewer.proto);
    }

    let footer = Line::raw("Esc / Backspace / q: close viewer")
        .centered()
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(footer, vertical[3]);
}
