//!
//! # cmd - dirback commands
//!

mod commands;

fn usage() {
    let s = r"Usage: dirback <command> [args...]";

    println!("{s}");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Error: No command specified.");
        usage();
        std::process::exit(1);
    }

    if args[0] == "-h" || args[0] == "help" {
        usage();
        std::process::exit(0);
    }

    let cmd = &args[0];
    let args = args[1..].to_vec();

    let mut invoker = cmd::CommandInvoker::new();
    invoker.register("list-targets", Box::new(commands::ListTargets));

    if let Err(e) = invoker.execute(cmd, args) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
