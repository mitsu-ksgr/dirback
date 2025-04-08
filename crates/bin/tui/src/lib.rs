//!
//! # DirbackTUI lib
//!

use ratatui::crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

mod app;
mod controller;
mod view;

pub fn run(basedir: &std::path::Path) -> anyhow::Result<()> {
    // Setup terminal.
    let mut terminal = ratatui::init();

    // Application loop.
    let mut app = app::App::new(basedir);
    app.fetch_targets();

    let mut view = view::View::default();

    loop {
        terminal.draw(|f| view.draw(f, &app))?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            if is_ctrl_c(key) {
                break;
            }

            controller::handle_key_events(&mut app, key);

            if app.quit_request {
                break;
            }
        }
    }

    // Restore terminal.
    ratatui::restore();
    Ok(())
}

fn is_ctrl_c(key: KeyEvent) -> bool {
    key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)
}
