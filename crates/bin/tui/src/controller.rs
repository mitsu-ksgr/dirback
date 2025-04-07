//!
//! # Dirback TUI View
//!

use crate::app;

use crossterm::event::{Event, KeyCode, KeyEvent};
use tracing::{debug, info};

pub fn handle_key_events(app: &mut app::App, key: KeyEvent) {
    info!("Handlekey: {key:?}");

    if app.current_popup.is_some() {
        match app.current_popup {
            Some(app::Popup::RegisterNewTarget) => in_register_target_popup(app, key),
            _ => {}
        }
    } else {
        match app.current_panel {
            app::Panel::TargetList => in_target_list_panel(app, key),
            app::Panel::TargetInfo => in_target_info_panel(app, key),
        }
    }
}

//-----------------------------------------------------------------------------
// Panels
//-----------------------------------------------------------------------------
fn in_target_list_panel(app: &mut app::App, key: KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            app.change_cursor_target(-1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.change_cursor_target(1);
        }
        KeyCode::Enter => {
            app.switch_panel(app::Panel::TargetInfo);
        }
        KeyCode::Char('r') => {
            // TODO: Register new target.
            app.show_popup(app::Popup::RegisterNewTarget);
        }
        KeyCode::Char('d') => {
            // TODO: Delete a target.
        }
        _ => {}
    }
}

fn in_target_info_panel(app: &mut app::App, key: KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            app.change_cursor_backup(-1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.change_cursor_backup(1);
        }
        KeyCode::Char('n') => {
            // TODO: Take a new backup.
        }
        KeyCode::Char('d') => {
            // TODO: Delete a backup.
        }
        KeyCode::Enter => {
            // TODO: Restore popup.
        }
        KeyCode::Backspace | KeyCode::Esc => {
            app.switch_panel(app::Panel::TargetList);
        }
        _ => {}
    }
}

//-----------------------------------------------------------------------------
// Popups
//-----------------------------------------------------------------------------
fn in_register_target_popup(app: &mut app::App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.hide_popup();
        }
        KeyCode::Tab => {
            app.popup_edit_index = 1 - app.popup_edit_index;
        }
        KeyCode::Char(ch) => {
            if let Some(buf) = app.popup_input_buf.get_mut(app.popup_edit_index) {
                buf.push(ch);
            }
        }
        KeyCode::Backspace => {
            if let Some(buf) = app.popup_input_buf.get_mut(app.popup_edit_index) {
                buf.pop();
            }
        }
        KeyCode::Enter => {
            // TODO: Submit.
            app.popup_errors.clear();
            let name = app.popup_input_buf.get(0).unwrap_or(&String::new()).clone();
            let path = app.popup_input_buf.get(1).unwrap_or(&String::new()).clone();

            // Check input.
            if name.is_empty() {
                app.popup_errors.push(String::from("Name is empty!"));
            }
            if path.is_empty() {
                app.popup_errors.push(String::from("Target path is empty!"));
            }
            if !app.popup_errors.is_empty() {
                return;
            }

            // Check path.
            let path = std::path::PathBuf::from(path);
            if !path.exists() {
                app.popup_errors
                    .push(String::from("Target path is invalid!"));
            }
            if !app.popup_errors.is_empty() {
                return;
            }

            // TODO: Submit!
            match app.register_target(&name, &path) {
                Ok(()) => {
                    app.hide_popup();
                    app.fetch_targets();
                    app.set_status(
                        app::Status::Info,
                        &format!("New target '{}' registered!", name),
                    );
                }
                Err(e) => {
                    app.popup_errors.push(e.to_string());
                }
            }
        }
        _ => {}
    }
}
