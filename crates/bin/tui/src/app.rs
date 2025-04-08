//!
//! # Dirback TUI Application
//!

use dirback::adapter::GetTargetAdapter;
use dirback::adapter::ListTargetsAdapter;
use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::infra::service::targz_backup_service::TargzBackupService;
use dirback::usecase::backup::BackupUsecase;
use dirback::usecase::delete_backup::DeleteBackupUsecase;
use dirback::usecase::delete_target::DeleteTargetUsecase;
use dirback::usecase::dto::Target;
use dirback::usecase::register_target::RegisterTargetUsecase;

#[derive(Debug, PartialEq)]
pub enum Panel {
    TargetList,
    TargetInfo,
}

#[derive(Debug, PartialEq)]
pub enum Popup {
    RegisterNewTarget,
    DeleteTarget,
    TakeBackup,
    DeleteBackup,
    Restore,
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Info,
    Warn,
    Error,
}

/// # App
///
/// Manages the application state and handles each events.
pub struct App {
    repo: FileStorageTargetRepository,
    pub targets: Vec<Target>,
    pub current_target: Option<Target>,

    // UI Info
    pub current_panel: Panel,
    pub cursor_target: usize,
    pub cursor_backup: usize,
    pub quit_request: bool,

    pub status: Option<Status>,
    pub message: Option<String>,

    // Popup
    pub current_popup: Option<Popup>,
    pub popup_input_buf: Vec<String>,
    pub popup_edit_index: usize,
    pub popup_errors: Vec<String>,
}

impl App {
    pub fn new(basedir: &std::path::Path) -> Self {
        Self {
            repo: FileStorageTargetRepository::new(basedir),

            // Targets
            targets: Vec::new(),
            current_target: None,

            // UI
            current_panel: Panel::TargetList,
            cursor_target: 0,
            cursor_backup: 0,
            quit_request: false,
            status: None,
            message: None,

            // Popup
            current_popup: None,
            popup_input_buf: Vec::new(),
            popup_edit_index: 0,
            popup_errors: Vec::new(),
        }
    }

    //-------------------------------------------------------------------------
    // Dirback
    //-------------------------------------------------------------------------
    pub fn fetch_targets(&mut self) {
        let list_targets = ListTargetsAdapter::new(&self.repo);
        match list_targets.execute() {
            Ok(targets) => {
                self.targets = targets.clone();
                self.set_status(Status::Info, &format!("{} targets loaded.", targets.len()));
            }
            Err(e) => {
                self.set_status(Status::Error, &format!("Failed to load targets: {e}"));
            }
        }
    }

    // TODO: 要らない？
    pub fn select_target(&mut self, target_id: &str) -> Option<()> {
        let adapter = GetTargetAdapter::new(&self.repo);
        match adapter.execute(&target_id) {
            Some(target) => {
                self.current_target = Some(target.clone());
                //self.switch_panel(Panel::TargetInfo);
                Some(())
            }
            None => {
                self.set_status(Status::Error, "Failed to load target: {target_id}");
                None
            }
        }
    }

    pub fn register_target(&mut self, name: &str, path: &std::path::Path) -> anyhow::Result<()> {
        if !path.exists() {
            anyhow::bail!("Target path is invalid: '{}'", path.to_string_lossy());
        }

        let path = std::fs::canonicalize(&path)?;
        let mut usecase = RegisterTargetUsecase::new(&mut self.repo);
        let _ = usecase.execute(&name, &path)?;

        self.fetch_targets();
        self.set_status(Status::Info, &format!("New target '{}' registered!", name));

        Ok(())
    }

    pub fn delete_current_target(&mut self) -> anyhow::Result<()> {
        if self.current_target.is_none() {
            anyhow::bail!("Target is none.");
        }

        let target = self.current_target.as_ref().unwrap().clone();
        let mut usecase = DeleteTargetUsecase::new(&mut self.repo);
        let dt = usecase.execute(&target.id)?;

        self.fetch_targets();
        self.set_status(
            Status::Info,
            &format!("The target '{}' has been deleted.", dt.name),
        );

        Ok(())
    }

    pub fn take_backup_of_current_target(&mut self, note: &str) -> anyhow::Result<()> {
        if self.current_target.is_none() {
            anyhow::bail!("Target is none.");
        }

        let target = self.current_target.as_ref().unwrap().clone();
        let note = note.to_string();

        let service = TargzBackupService::new();
        let mut usecase = BackupUsecase::new(&mut self.repo, &service);
        usecase.execute(&target.id, &note)?;

        // Update current-target
        self.fetch_targets();
        if let Some(target) = self.targets.iter().find(|t| t.id == target.id) {
            self.current_target = Some(target.clone());
        }

        self.set_status(
            Status::Info,
            &format!("Target('{}') backup is complete!", target.name),
        );

        Ok(())
    }

    pub fn delete_current_backup(&mut self) -> anyhow::Result<()> {
        if self.current_target.is_none() {
            anyhow::bail!("Target is none.");
        }

        let target = self.current_target.as_ref().unwrap().clone();
        let entry = target.backups.get(self.cursor_backup);
        if entry.is_none() {
            anyhow::bail!("Backup is none.");
        }
        let entry = entry.unwrap();

        let mut usecase = DeleteBackupUsecase::new(&mut self.repo);
        let deleted_entry = usecase.execute(&target.id, entry.id)?;

        // Update current target
        self.fetch_targets();
        if let Some(target) = self.targets.iter().find(|t| t.id == target.id) {
            self.current_target = Some(target.clone());
        }

        self.set_status(
            Status::Info,
            &format!("Backup[{:0>3}] has been deleted.", deleted_entry.id),
        );

        Ok(())
    }

    //-------------------------------------------------------------------------
    // Panel
    //-------------------------------------------------------------------------
    pub fn quit(&mut self) {
        self.quit_request = true;
    }

    pub fn switch_panel(&mut self, to: Panel) -> bool {
        if self.current_panel == to {
            return false; // do nothing.
        }

        match to {
            Panel::TargetList => {
                self.current_panel = Panel::TargetList;
                self.current_target = None;
                self.cursor_backup = 0;
            }
            Panel::TargetInfo => {
                if self.targets.len() == 0 {
                    return false;
                }

                match self.targets.get(self.cursor_target) {
                    Some(target) => {
                        self.current_target = Some(target.clone());
                        self.cursor_backup = 0;
                        self.current_panel = Panel::TargetInfo;
                    }
                    None => return false,
                }
            }
        }

        return true;
    }

    //-------------------------------------------------------------------------
    // Cursor
    //-------------------------------------------------------------------------
    pub fn change_cursor_target(&mut self, changevalue: isize) {
        self.cursor_target = change_cursor(self.cursor_target, changevalue, self.targets.len());
    }

    pub fn change_cursor_backup(&mut self, changevalue: isize) {
        let target = self.current_target.clone();
        if let Some(target) = target {
            self.cursor_backup =
                change_cursor(self.cursor_backup, changevalue, target.backups.len());
        }
    }

    //-------------------------------------------------------------------------
    // Status
    //-------------------------------------------------------------------------
    pub fn set_status(&mut self, status: Status, message: &str) {
        self.status = Some(status);
        self.message = Some(message.to_string());
    }

    pub fn clear_status(&mut self) {
        self.status = None;
        self.message = None;
    }

    //-------------------------------------------------------------------------
    // Popup
    //-------------------------------------------------------------------------
    pub fn hide_popup(&mut self) {
        self.current_popup = None;
        self.popup_input_buf.clear();
        self.popup_edit_index = 0;
        self.popup_errors.clear();
    }

    pub fn show_popup(&mut self, popup: Popup) -> bool {
        if self.current_popup.is_some() {
            return false;
        }

        match popup {
            Popup::DeleteTarget => {
                if let Some(target) = self.targets.get(self.cursor_target) {
                    self.current_target = Some(target.clone());
                } else {
                    return false;
                }
            }
            _ => {}
        }

        self.current_popup = Some(popup);
        self.popup_input_buf.push(String::new());
        self.popup_input_buf.push(String::new());
        return true;
    }
}

//-------------------------------------------------------------------------
//  Helper functions
//-------------------------------------------------------------------------
fn change_cursor(current: usize, change: isize, len: usize) -> usize {
    let mut cursor = current.checked_add_signed(change).unwrap_or(0);
    let limit = if len == 0 { 0 } else { len - 1 };
    if cursor > limit {
        cursor = limit;
    }

    cursor
}

//-------------------------------------------------------------------------
//  Tests
//-------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use dirback::infra::repository::file_storage::FileStorageTargetRepository;
    use dirback::internal::TargetRepository;
    use dirback::usecase::dto::Target;

    fn make_dummy_app() -> App {
        App::new(std::path::Path::new("./tmp/test"))
    }

    fn make_app(temp: &mktemp::TempDir) -> App {
        App::new(&temp.path())
    }

    fn add_test_targets(app: &mut App) -> Vec<String> {
        let mut ids = Vec::new();

        for i in 1..=3 {
            let target_name = format!("TestTarget{0:>2}", i).to_string();
            let target_path = std::fs::canonicalize(".").unwrap();
            let target = app.repo.add(&target_name, &target_path).unwrap();
            ids.push(target.id.clone());
        }

        ids
    }

    fn add_test_backups(app: &mut App, target_id: &str) -> Target {
        let mut target = app.repo.load(target_id).unwrap();
        let bkdir = app.repo.make_backup_dir_path(&target);
        for i in 1..=3 {
            let entry = target.new_backup_entry(&bkdir, "tar.gz");
            let _ = std::fs::File::create(&entry.path); // make dummy backup file.
            let _ = target.register_backup_entry(entry);
        }
        let target = app.repo.update(&target).unwrap();
        target.into()
    }

    #[test]
    fn test_fetch_targets() {
        let temp = mktemp::TempDir::new().unwrap();
        let mut app = make_app(&temp);
        assert_eq!(app.targets.len(), 0);

        let _ = add_test_targets(&mut app);
        app.fetch_targets();
        assert_eq!(app.targets.len(), 3);
    }

    mod register_target {
        use super::*;

        #[test]
        fn it_works() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut app = make_app(&temp);

            let _ = add_test_targets(&mut app);
            app.fetch_targets();
            assert_eq!(app.targets.len(), 3);

            let name = "RegisterTest";
            let path = std::path::Path::new(".");

            let result = app.register_target(name, path);
            assert!(result.is_ok());
            assert_eq!(app.targets.len(), 4);
        }

        #[test]
        fn it_fails_when_invalid_path() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut app = make_app(&temp);

            let ids = add_test_targets(&mut app);
            app.fetch_targets();
            assert_eq!(app.targets.len(), 3);

            let name = "RegisterTest";
            let path = std::path::Path::new("./invalid-path");

            let result = app.register_target(name, path);
            assert!(result.is_err());
        }
    }

    mod delete_current_target {
        use super::*;

        #[test]
        fn it_works() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut app = make_app(&temp);

            let _ = add_test_targets(&mut app);
            app.fetch_targets();
            assert_eq!(app.targets.len(), 3);

            let del_target = app.targets[1].clone();
            app.current_target = Some(del_target.clone());

            let result = app.delete_current_target();
            assert!(result.is_ok());
            assert_eq!(app.targets.len(), 2);
        }

        #[test]
        fn it_fails_when_current_target_not_set() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut app = make_app(&temp);

            let result = app.delete_current_target();
            assert!(result.is_err());
        }
    }

    mod delete_backup {
        use super::*;

        #[test]
        fn it_works() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut app = make_app(&temp);

            let ids = add_test_targets(&mut app);
            app.fetch_targets();

            let target = add_test_backups(&mut app, &ids[1]);
            assert_eq!(target.backups.len(), 3);

            app.current_target = Some(target.clone());
            app.cursor_backup = 1;

            let result = app.delete_current_backup();
            assert!(result.is_ok());

            let target: Target = app.repo.load(&target.id).unwrap().into();
            assert_eq!(target.backups.len(), 2);
        }

        #[test]
        fn it_fails_when_current_target_not_set() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut app = make_app(&temp);

            let result = app.delete_current_backup();
            assert!(result.is_err());
        }
    }

    mod select_target {
        use super::*;

        #[test]
        fn it_works() {
            let temp = mktemp::TempDir::new().unwrap();
            let mut app = make_app(&temp);

            let ids = add_test_targets(&mut app);
            app.fetch_targets();

            let target_id = &ids[1];

            assert_eq!(app.current_target, None);

            let result = app.select_target(target_id);
            assert!(result.is_some());
            assert!(app.current_target.is_some());

            let target = &app.current_target.unwrap();
            assert_eq!(target.id, *target_id);
        }
    }

    mod quit {
        use super::*;

        #[test]
        fn it_works() {
            let mut app = make_dummy_app();
            assert_eq!(app.quit_request, false);

            app.quit();
            assert_eq!(app.quit_request, true);
        }
    }

    mod switch_panel {
        use super::*;

        #[test]
        fn it_works() {
            let mut app = make_dummy_app();
            assert_eq!(
                app.current_panel,
                Panel::TargetList,
                "Default panel should be TargetList"
            );
        }

        #[test]
        fn back_to_targetlist_panel() {
            let mut app = make_dummy_app();

            app.current_panel = Panel::TargetInfo;
            app.current_target = Some(Target {
                id: String::from("xxx-xxx-xxx"),
                name: String::from("Test Target"),
                path: std::path::PathBuf::from("."),
                backups: Vec::new(),
            });
            app.cursor_target = 10;
            app.cursor_backup = 10;

            app.switch_panel(Panel::TargetList);
            assert_eq!(app.current_panel, Panel::TargetList);
            assert_eq!(
                app.current_target, None,
                "current_target should be reset to None"
            );
            assert_eq!(app.cursor_backup, 0, "cursor_backup should be reset");
        }
    }

    mod status {
        use super::*;

        #[test]
        fn default_statsu_is_none() {
            let app = make_dummy_app();
            assert!(app.status.is_none());
            assert!(app.message.is_none());
        }

        #[test]
        fn it_works() {
            let mut app = make_dummy_app();

            app.set_status(Status::Info, "test message");
            assert_eq!(app.status, Some(Status::Info));
            assert_eq!(app.message, Some(String::from("test message")));

            app.set_status(Status::Warn, "warning");
            assert_eq!(app.status, Some(Status::Warn));
            assert_eq!(app.message, Some(String::from("warning")));

            app.set_status(Status::Error, "error");
            assert_eq!(app.status, Some(Status::Error));
            assert_eq!(app.message, Some(String::from("error")));

            app.clear_status();
            assert!(app.status.is_none());
            assert!(app.message.is_none());
        }
    }

    mod popup {
        use super::*;

        #[test]
        fn it_works() {
            let mut app = make_dummy_app();
            assert_eq!(app.current_popup, None);
            assert_eq!(app.popup_input_buf.len(), 0);
            assert_eq!(app.popup_edit_index, 0);
            assert_eq!(app.popup_errors.len(), 0);

            let result = app.show_popup(Popup::RegisterNewTarget);
            assert!(result);
            assert_eq!(app.current_popup, Some(Popup::RegisterNewTarget));

            app.hide_popup();
            assert_eq!(app.current_popup, None);
            assert_eq!(app.popup_input_buf.len(), 0);
            assert_eq!(app.popup_edit_index, 0);
            assert_eq!(app.popup_errors.len(), 0);
        }

        #[test]
        fn show_popup_returns_false_when_popup_already_shown() {
            let mut app = make_dummy_app();

            let result = app.show_popup(Popup::RegisterNewTarget);
            assert!(result);
            assert_eq!(app.current_popup, Some(Popup::RegisterNewTarget));

            let result = app.show_popup(Popup::DeleteTarget);
            assert!(!result);
            assert_eq!(app.current_popup, Some(Popup::RegisterNewTarget));

            let result = app.show_popup(Popup::RegisterNewTarget);
            assert!(!result);
        }
    }

    #[test]
    fn test_change_cursor() {
        assert_eq!(change_cursor(1, 2, 5), 3);

        // Upper limit
        assert_eq!(change_cursor(1, 2, 3), 2);
        assert_eq!(change_cursor(0, 1, 0), 0);

        // Under limit
        assert_eq!(change_cursor(1, -2, 3), 0);
        assert_eq!(change_cursor(0, -1, 0), 0);
    }
}
