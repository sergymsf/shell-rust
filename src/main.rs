use std::process;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn main() {

    let builtins = ["echo", "exit", "type"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        if command.is_empty() {
            continue;
        }
        if command == "exit 0" {
            process::exit(0);
        }
        if command.starts_with("echo ") {
            println!("{}", &command[5..]);
            continue;
        }

        if command.starts_with("type ") {
            let to_check = &command[5..];
            if builtins.contains(&to_check) {
                println!("{} is a shell builtin", to_check);
            } else {
                if let Some(path) = find_in_path(to_check) {
                    println!("{} is {}", to_check, path.display());
                } else {
                    println!("{}: not found", to_check);
                }
            }
            continue;
        }

        println!("{}: command not found", command);
    }
}

fn find_in_path(command: &str) -> Option<std::path::PathBuf> {
    if let Ok(path_var) = env::var("PATH") {
        for path in path_var.split(':') {
            let full_path = Path::new(path).join(command);
            if full_path.is_file() && is_executable(&full_path) {
                return Some(full_path);
            }
        }
    }
    None
}

fn is_executable(path: &Path) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
    } else {
        false
    }
}

