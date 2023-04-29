use std::collections::BTreeSet; // to keep the order
use std::env;
use std::fs::read_dir;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::exit;

use colored::*;
use suggest::Suggest;

const PATH: &str = "PATH";

#[inline]
fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = path.metadata() {
        metadata.permissions().mode() & 0o111 != 0
    } else {
        false
    }
}

#[inline]
fn is_executable_file(path: &Path) -> bool {
    path.is_file() && is_executable(path)
}

fn find_executables() -> BTreeSet<String> {
    let mut executables = BTreeSet::<String>::new();

    if let Some(paths) = env::var_os(PATH) {
        for path in env::split_paths(&paths) {
            if let Ok(entries) = read_dir(path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if is_executable_file(&path) {
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
    let err_msg = format!("{} command not found:", "Error:".red());

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("{err_msg} \"\"");
        exit(NOTFOUND);
    }

    let cmd_name = &args[1];
    let cmd_args = &args[2..].join(" ");
    eprintln!("{err_msg} {} {cmd_args}", cmd_name.red());

    let executables = find_executables();
    if let Some(sugg) = executables.suggest_by(cmd_name, 2) {
        eprintln!("{:>6} Did you mean \"{}\"?", "==>".green(), sugg.green());
    }

    // Always exits with 127 since this binary is supposed
    // to be called only when a command not found.
    exit(NOTFOUND);
}
