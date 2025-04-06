//!
//! # Dirback TUI View
//!

use crate::app;

use crossterm::event::{Event, KeyCode, KeyEvent};
use tracing::{debug, info};

pub fn handle_key_events(app: &mut app::App, key: KeyEvent) {
    match app.current_panel {
        app::Panel::TargetList => in_target_list_panel(app, key),
        app::Panel::TargetInfo => in_target_info_panel(app, key),
    }
}

fn in_target_list_panel(app: &mut app::App, key: KeyEvent) {
    info!("controller#targe_list: key {key:?}");

    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            app.change_cursor_target(-1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.change_cursor_target(1);
        }
        _ => {}
    }
}

fn in_target_info_panel(app: &mut app::App, key: KeyEvent) {
    info!("controller#targe_info: key {key:?}");
}
