use std::collections::BTreeSet; // to keep the order
use std::os::unix::fs::PermissionsExt;
use std::process::exit;
use std::{env, fs};

use colored::*;
use suggest::Suggest;

fn find_executables() -> BTreeSet<String> {
    let paths = env::var_os("PATH").unwrap();
    let mut executables = BTreeSet::<String>::new();

    for path in env::split_paths(&paths) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(metadata) = path.metadata() {
                    let permissions = metadata.permissions();
                    if path.is_file() && permissions.mode() & 0o111 != 0 {
                        if let Some(exe_name) = path.file_name() {
                            if let Ok(exe) = exe_name.to_os_string().into_string() {
                                executables.insert(exe);
                            }
                        }
                    }
                }
            }
        }
    }

    executables
}

const NOTFOUND: i32 = 127;

fn main() {
    let err = "Error:".red();

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("{err} command not found: \"\"");
        exit(NOTFOUND);
    }

    let cmd_name = &args[1];
    let cmd_args = &args[2..].join(" ");
    eprintln!("{err} command not found: {} {cmd_args}", cmd_name.red());

    let executables = find_executables();
    if let Some(sugg) = executables.suggest_by(cmd_name, 2) {
        eprintln!("{:>6} Did you mean \"{}\"?", "==>".green(), sugg.green());
    }

    // Always exits with 127 since this binary is supposed
    // to be called only when a command not found.
    exit(NOTFOUND);
}
