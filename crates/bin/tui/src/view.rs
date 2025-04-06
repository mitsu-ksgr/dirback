//!
//! # Dirback TUI View
//!

use crate::app;
use tracing::{debug, info};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

#[derive(Default)]
pub struct View {
    pub target_list_offset: usize,
    pub backup_list_offset: usize,
}

impl View {
    pub fn draw(&mut self, frame: &mut Frame, app: &app::App) {
        let has_notify = app.status.is_some();
        let status_bar_len = if has_notify { 3 } else { 0 };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(status_bar_len),
                Constraint::Length(8),
            ])
            .split(frame.area());

        // Header
        let header = make_header_panel("Dirback - v0.1.0");
        frame.render_widget(header, chunks[0]);

        // Main: target-list panel.
        let target_list = make_target_list_panel(self, &app, chunks[1]);
        frame.render_widget(target_list, chunks[1]);

        // State.
        if has_notify {
            let status_bar = make_status_bar_panel(&app);
            frame.render_widget(status_bar, chunks[2]);
        }

        // Footer.
        let footer = make_footer_panel(&app);
        frame.render_widget(footer, chunks[3]);
    }
}

//-----------------------------------------------------------------------------
//  Main panel
//-----------------------------------------------------------------------------
fn make_target_list_panel<'a>(ui: &'a mut View, app: &'a app::App, chunk: Rect) -> List<'a> {
    let mut title = String::from(" Targets ");
    let mut list_items = Vec::<ListItem>::new();

    if app.targets.len() == 0 {
        list_items.push(ListItem::new(Line::from(vec![Span::raw(
            "No targets registered yet.",
        )])));
        list_items.push(ListItem::new(Line::from(vec![
            Span::raw("Press the '"),
            Span::styled("r", Style::default().fg(Color::Yellow)),
            Span::raw("' key to register a new target!"),
        ])));
    } else if chunk.height <= 3 {
        // Too low
        info!("There is no space to list the targets.");
    } else {
        let height = usize::from(chunk.height) - 2;
        let list_len = app.targets.len();

        // Update offset
        if app.cursor_target < ui.target_list_offset {
            ui.target_list_offset = app.cursor_target;
        } else if app.cursor_target >= ui.target_list_offset + height {
            ui.target_list_offset = app.cursor_target - (height - 1);
        }

        // Visibles
        let start = ui.target_list_offset;
        let end = std::cmp::min(ui.target_list_offset + height, list_len);
        let visible_targets = &app.targets[start..end];

        // Format
        let mut max_name_len = 0;
        for target in app.targets.iter() {
            if target.name.len() > max_name_len {
                max_name_len = target.name.len();
            }
        }
        let name_width = max_name_len + 3;

        // Update Title
        title.push_str(&format!("[{:>2} ~ {:>2} / {list_len}]", start + 1, end));

        // Build list items.
        for (i, target) in visible_targets.iter().enumerate() {
            let target = target.clone();
            let idx = i + start;
            let cursor = if idx == app.cursor_target {
                String::from(" > ")
            } else {
                String::from("   ")
            };

            let name = format!("{:<name_width$}", target.name);
            let path = target.path.display().to_string();

            list_items.push(ListItem::new(Line::from(vec![
                Span::from(cursor),
                Span::from(name),
                Span::raw("  "),
                Span::from(path),
            ])));
        }
    }

    // Render view.
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default());

    List::new(list_items).block(block)
}

fn make_status_bar_panel(app: &app::App) -> Paragraph {
    let (title, color) = match app.status {
        Some(app::Status::Info) => (String::from(" Notice "), Color::Blue),
        Some(app::Status::Warn) => (String::from(" Warning "), Color::Yellow),
        Some(app::Status::Error) => (String::from(" ERROR "), Color::Red),
        None => (String::from(""), Color::Black), // <-- unexpected!
    };

    let mut title_style = Style::default().fg(color).add_modifier(Modifier::BOLD);

    if let Some(app::Status::Info) = app.status {
        // Do nothing.
    } else {
        title_style = title_style.bg(Color::DarkGray);
    }

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(title_style);

    let msg = app.message.as_ref().unwrap().clone();
    let msg_style = Style::default()
        .fg(Color::Reset)
        .remove_modifier(Modifier::BOLD);

    Paragraph::new(Text::styled(msg, msg_style)).block(block)
}

//-----------------------------------------------------------------------------
//  Header / Footer
//-----------------------------------------------------------------------------
fn make_header_panel(title: &str) -> Paragraph {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    Paragraph::new(Text::styled(title, Style::default().fg(Color::Green))).block(block)
}

fn make_footer_key_line(name: &str, keys: Vec<&str>) -> Line<'static> {
    let mut spans = vec![Span::raw(format!("{name:<25}")), Span::raw(" : ")];

    for (i, key) in keys.iter().enumerate() {
        let key = key.to_string();

        if i > 0 {
            spans.push(Span::raw(", "));
        }

        if key.len() == 1 {
            spans.push(Span::raw("'"));
            spans.push(Span::styled(key, Style::default().fg(Color::Yellow)));
            spans.push(Span::raw("'"));
        } else {
            spans.push(Span::styled(key, Style::default().fg(Color::Yellow)));
        }
    }

    Line::from(spans)
}

fn make_footer_panel(app: &app::App) -> Paragraph {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let mut lines = {
        match app.current_panel {
            app::Panel::TargetList => vec![
                Line::from(vec![
                    Span::styled("Target selection", title_style.clone()),
                    Span::raw(":"),
                ]),
                make_footer_key_line("  Register new target", vec!["n"]),
                make_footer_key_line("  Select a target", vec!["ArrowKeys", "k", "j", "Enter"]),
                make_footer_key_line("  Delete a target", vec!["d"]),
            ],
            app::Panel::TargetInfo => vec![
                Line::from(vec![
                    Span::styled("Target actions", title_style.clone()),
                    Span::raw(":"),
                ]),
                make_footer_key_line("  Take a new backup", vec!["n"]),
                make_footer_key_line("  Select a backup", vec!["ArrowKeys", "k", "j", "Enter"]),
                make_footer_key_line("  Back", vec!["b", "ESC", "BACKSPACE"]),
            ],
        }
    };

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("Quit with "),
        Span::styled("<Ctrl+c>", Style::default().fg(Color::Red)),
        Span::raw(" or '"),
        Span::styled("q", Style::default().fg(Color::Red)),
        Span::raw("'"),
    ]));

    Paragraph::new(lines).block(block)
}
