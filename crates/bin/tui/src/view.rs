//!
//! # Dirback TUI View
//!

use crate::app;
use dirback::usecase::dto::Target;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};
use tracing::{debug, info};

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
        match app.current_panel {
            app::Panel::TargetList => {
                let target_list = make_target_list_panel(self, &app, chunks[1]);
                frame.render_widget(target_list, chunks[1]);
            }
            app::Panel::TargetInfo => {
                render_target_info_panel(frame, self, &app, chunks[1]);
            }
        }

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
//  Main panel - Target Info panel.
//-----------------------------------------------------------------------------
fn render_target_info_panel(frame: &mut Frame, ui: &mut View, app: &app::App, chunk: Rect) {
    // Target check.
    let target = app.current_target.clone();
    if target.is_none() {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let warn = Paragraph::new("Target information is none.").block(block);
        frame.render_widget(warn, chunk);
        return;
    }
    let target = target.unwrap();

    // Layout.
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunk);
    let left = chunks[0];
    let right = chunks[1];

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(right);
    let right_top = chunks[0];
    let right_bottom = chunks[1];

    // Left part: Target information.
    let target_info_panel = make_target_info_panel(&target);
    frame.render_widget(target_info_panel, left);

    // Right part: Backup lists.
    let backup_list = make_backup_list_panel(ui, &app, &target, right);
    frame.render_widget(backup_list, right_top);

    let backup_info = make_backup_info_panel(&app, &target);
    frame.render_widget(backup_info, right_bottom);
}

fn make_target_info_panel(target: &Target) -> Paragraph {
    let block = Block::default()
        .title(format!(" Target: {} ", target.name))
        .borders(Borders::ALL)
        .style(Style::default());

    let key_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let lines = vec![
        Line::from(vec![
            Span::styled("Name", key_style.clone()),
            Span::raw("    : "),
            Span::from(target.name.clone()),
        ]),
        Line::from(vec![
            Span::styled("ID", key_style.clone()),
            Span::raw("      : "),
            Span::from(target.id.clone()),
        ]),
        Line::from(vec![
            Span::styled("Target", key_style.clone()),
            Span::raw("  : "),
        ]),
        Line::from(vec![
            Span::raw("    "),
            Span::from(target.path.display().to_string()),
        ]),
        Line::from(vec![
            Span::styled("Backups", key_style.clone()),
            Span::raw(" : "),
            Span::from(format!("{}", target.backups.len())),
        ]),
    ];

    Paragraph::new(lines).block(block)
}

fn make_backup_list_panel<'a>(
    ui: &'a mut View,
    app: &'a app::App,
    target: &Target,
    chunk: Rect,
) -> List<'a> {
    let mut title = String::from(" Backups ");
    let mut list_items = Vec::<ListItem>::new();

    if target.backups.is_empty() {
        list_items.push(ListItem::new(Line::from("No backups.")));
    } else {
        let height = usize::from(chunk.height) - 2;
        let list_len = target.backups.len();

        // Update offset
        if app.cursor_backup < ui.backup_list_offset {
            ui.backup_list_offset = app.cursor_backup;
        } else if app.cursor_backup >= ui.backup_list_offset + height {
            ui.backup_list_offset = app.cursor_backup - (height - 1);
        }

        // Visibles
        let start = ui.backup_list_offset;
        let end = std::cmp::min(ui.backup_list_offset + height, list_len);
        let visible_backups = &target.backups[start..end];

        // Update title
        title.push_str(&format!("[{:>2} ~ {:>2} / {list_len}]", start + 1, end));

        // ListItems
        for (i, entry) in visible_backups.iter().enumerate() {
            let entry = entry.clone();
            let idx = i + start;
            let cursor = if idx == app.cursor_backup {
                String::from(" > ")
            } else {
                String::from("   ")
            };

            list_items.push(ListItem::new(Line::from(vec![
                Span::from(cursor),
                Span::from(format!("{:0>3}", entry.id)),
                Span::raw(" - "),
                Span::from(entry.timestamp.to_rfc3339()),
                Span::raw(" : "),
                Span::from(entry.note),
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

fn make_backup_info_panel<'a>(app: &'a app::App, target: &Target) -> Paragraph<'a> {
    let entry = target.backups.get(app.cursor_backup).clone();
    if entry.is_none() {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());
        return Paragraph::new("").block(block);
    }

    let entry = entry.unwrap();

    let block = Block::default()
        .title(format!(" Backup {0:>3} ", entry.id))
        .borders(Borders::ALL)
        .style(Style::default());

    let key_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let lines = vec![
        Line::from(vec![
            Span::styled("ID", key_style.clone()),
            Span::raw("          : "),
            Span::from(format!("{:0>3}", entry.id)),
        ]),
        Line::from(vec![
            Span::styled("Timestamp", key_style.clone()),
            Span::raw("   : "),
            Span::from(entry.timestamp.to_rfc3339()),
        ]),
        Line::from(vec![
            Span::styled("Backup File", key_style.clone()),
            Span::raw(" : "),
        ]),
        Line::from(Span::from(entry.path.display().to_string())),
        Line::from(vec![
            Span::styled("Note ", key_style.clone()),
            Span::raw("       : "),
        ]),
        Line::from(vec![Span::from(entry.note.clone())]),
    ];

    Paragraph::new(lines).block(block).wrap(Wrap { trim: true })
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
