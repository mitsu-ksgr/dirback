//!
//! # Dirback TUI Application
//!

use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::usecase::dto::Target;

use crossterm::event::{Event, KeyEvent};

pub enum Panel {
    TargetList,
    TargetInfo,
}

/// # App
///
/// Manages the application state and handles each events.
pub struct App {
    repo: FileStorageTargetRepository,
    pub targets: Vec<Target>,

    // UI Info
    pub active_panel: Panel,
}

impl App {
    pub fn new(basedir: &std::path::Path) -> Self {
        Self {
            repo: FileStorageTargetRepository::new(basedir),
            targets: Vec::new(),
            active_panel: Panel::TargetList,
        }
    }

    pub fn fetch_targets() {
        // TODO
    }

    pub fn set_active_target(&mut self, target_id: &str) {
        // TODO
    }

    pub fn handle_key_events(&mut self, key: KeyEvent) {
        // TODO
    }
}
