//!
//! # Dirback TUI View
//!

use crate::app;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn draw(frame: &mut Frame, app: &app::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Header
    let header = make_header("Dirback - v0.1.0");
    frame.render_widget(header, chunks[0]);

    // Main panel.

    // Footer.
}

fn make_header(title: &str) -> Paragraph {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(title, Style::default().fg(Color::Green))).block(block);

    title
}
