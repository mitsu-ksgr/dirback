//!
//! # Dirback TUI View
//!

use crate::app;

use crossterm::event::{KeyCode, KeyEvent};
use tracing::info;

pub fn handle_key_events(app: &mut app::App, key: KeyEvent) {
    if app.current_popup.is_some() {
        match app.current_popup {
            Some(app::Popup::RegisterNewTarget) => in_register_target_popup(app, key),
            Some(app::Popup::DeleteTarget) => in_delete_target_popup(app, key),
            Some(app::Popup::TakeBackup) => in_take_backup_popup(app, key),
            Some(app::Popup::DeleteBackup) => in_delete_backup_popup(app, key),
            Some(app::Popup::Restore) => in_restore_popup(app, key),
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
            app.show_popup(app::Popup::RegisterNewTarget);
        }
        KeyCode::Char('d') => {
            app.show_popup(app::Popup::DeleteTarget);
        }
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
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
        KeyCode::Char('n') | KeyCode::Char('b') => {
            app.show_popup(app::Popup::TakeBackup);
        }
        KeyCode::Char('d') => {
            app.show_popup(app::Popup::DeleteBackup);
        }
        KeyCode::Enter => {
            app.show_popup(app::Popup::Restore);
        }
        KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('q') => {
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
            app.popup_errors.clear();
            let name = app
                .popup_input_buf
                .first()
                .unwrap_or(&String::new())
                .clone();
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

            // Submit
            match app.register_target(&name, &path) {
                Ok(()) => app.hide_popup(),
                Err(e) => app.popup_errors.push(e.to_string()),
            }
        }
        _ => {}
    }
}

fn in_delete_target_popup(app: &mut app::App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.hide_popup();
        }
        KeyCode::Char(ch) => {
            if let Some(buf) = app.popup_input_buf.get_mut(0) {
                buf.push(ch);
            }
        }
        KeyCode::Backspace => {
            if let Some(buf) = app.popup_input_buf.get_mut(0) {
                buf.pop();
            }
        }
        KeyCode::Enter => {
            app.popup_errors.clear();
            let confirm = app
                .popup_input_buf
                .first()
                .unwrap_or(&String::new())
                .clone();
            let target_name = app.current_target.as_ref().unwrap().name.clone();

            // Check input.
            if confirm != target_name {
                app.popup_errors.push(String::from("Confirmation failed!"));
            }
            if !app.popup_errors.is_empty() {
                return;
            }

            // Submit
            match app.delete_current_target() {
                Ok(()) => app.hide_popup(),
                Err(e) => app.popup_errors.push(e.to_string()),
            }
        }
        _ => {}
    }
}

fn in_take_backup_popup(app: &mut app::App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.hide_popup();
        }
        KeyCode::Char(ch) => {
            if let Some(buf) = app.popup_input_buf.get_mut(0) {
                buf.push(ch);
            }
        }
        KeyCode::Backspace => {
            if let Some(buf) = app.popup_input_buf.get_mut(0) {
                buf.pop();
            }
        }
        KeyCode::Enter => {
            app.popup_errors.clear();
            let note = app
                .popup_input_buf
                .first()
                .unwrap_or(&String::new())
                .clone();
            match app.take_backup_of_current_target(&note) {
                Ok(()) => app.hide_popup(),
                Err(e) => app.popup_errors.push(e.to_string()),
            }
        }
        _ => {}
    }
}

fn in_delete_backup_popup(app: &mut app::App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('q') | KeyCode::Char('n') => {
            app.hide_popup();
        }
        KeyCode::Char('y') => {
            app.popup_errors.clear();
            match app.delete_current_backup() {
                Ok(()) => app.hide_popup(),
                Err(e) => app.popup_errors.push(e.to_string()),
            }
        }
        _ => {}
    }
}

fn in_restore_popup(app: &mut app::App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('q') | KeyCode::Char('n') => {
            app.hide_popup();
        }
        KeyCode::Char('y') => {
            app.popup_errors.clear();
            match app.restore_target_with_current_backup() {
                Ok(()) => app.hide_popup(),
                Err(e) => app.popup_errors.push(e.to_string()),
            }
        }
        _ => {}
    }
}
