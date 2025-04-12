//!
//! # dirback GUI lib.rs
//!

mod commands;
mod dispatcher;

use crate::commands::CommandType;
use crate::dispatcher::Dispatcher;

use tauri::Manager;

/// # AppState
///
/// ref: https://v2.tauri.app/ja/develop/state-management/
#[derive(Default)]
struct AppState {
    pub datadir: std::path::PathBuf,
}

#[tauri::command]
async fn command_dispatcher(
    state: tauri::State<'_, AppState>,
    cmd: CommandType,
) -> Result<serde_json::Value, String> {
    println!("command_dispatcher: {cmd:#?}");
    let dispatcher = Dispatcher::new(&state.datadir);

    match dispatcher.dispatch(cmd) {
        Ok(result) => Ok(result),
        Err(e) => Err(e.to_string()),
    }
}

pub fn run(datadir: &std::path::Path) -> anyhow::Result<()> {
    let datadir = datadir.to_path_buf();

    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState { datadir });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command_dispatcher])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
