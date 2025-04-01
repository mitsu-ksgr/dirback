//!
//! # cmd - dirback commands
//!

use dirback::infra::app_path;

mod commands;

fn usage() {
    let s = r#"Usage: dirback <command> [args...]

Path to store backups:
    if DIRBACK_STORE_DIR is set, use it's value.
    if not set, use default directory.

    Linux: ~/.local/share/dirback
    Windows: TODO

Commands:
    help, -h
        Print help.

    list
        Print target list.

    register [TARGET_PATH]
        Register new target.

    show [TARGET_ID]
        Show target information.

    backup [TARGET_ID] [NOTE]
        Take a backup of the target.

    restore [TARGET_ID] [BACKUP_ID]
        Restore from the specified backup.

    delete [TARGET_ID] [BACKUP_ID]
        Delete the backup.
        This action cannnot be undone.

    delete-target [TARGET_ID]
        Delete the target.
        The target's backups will also be deleted.
        This action cannnot be undone.
"#;

    println!("{s}");
}

fn get_basedir_path() -> anyhow::Result<std::path::PathBuf> {
    // If DIRBACK_STORE_DIR is set, use it's value.
    // If not set, use app_path::data_dir.
    let basedir = std::env::var("DIRBACK_STORE_DIR")
        .ok()
        .map(std::path::PathBuf::from)
        .or_else(app_path::data_dir)
        .ok_or_else(|| anyhow::anyhow!("Failed to get path to directory for application data."))?;

    Ok(basedir)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let basedir = get_basedir_path().unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    let params = cmd::CmdParams::build(&args, &basedir).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        usage();
        std::process::exit(1);
    });

    if params.command == "-h" || params.command == "help" {
        usage();
        std::process::exit(0);
    }

    let mut invoker = cmd::CommandInvoker::new();
    invoker.register("list", Box::new(commands::ListTargets));
    invoker.register("register", Box::new(commands::RegisterTarget));
    invoker.register("show", Box::new(commands::ShowTarget));
    invoker.register("backup", Box::new(commands::BackupTarget));
    invoker.register("restore", Box::new(commands::RestoreTarget));
    invoker.register("delete", Box::new(commands::DeleteBackup::new()));
    invoker.register("delete-target", Box::new(commands::DeleteTarget::new()));

    if let Err(e) = invoker.execute(&params) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
