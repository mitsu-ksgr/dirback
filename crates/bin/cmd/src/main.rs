//!
//! # cmd - dirback commands
//!

mod commands;

fn usage() {
    let s = r#"Usage: dirback <command> [args...]

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
"#;

    println!("{s}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let params = cmd::CmdParams::build(&args).unwrap_or_else(|e| {
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

    if let Err(e) = invoker.execute(&params) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
