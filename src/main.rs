use std::process;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

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

        if command == "pwd" {
            if let Ok(current_dir) = env::current_dir() {
                if let Some(path) = current_dir.to_str() {
                    println!("{}", path);
                } else {
                    eprintln!("Failed to convert current directory to string");
                }
            } else {
                eprintln!("Failed to get current directory");
            }
            continue;
        }

        if command.starts_with("cd ") {
            let args: Vec<&str> = command.split_whitespace().collect();
            if args.len() != 2 {
                println!("usage: cd <directory>");
                continue;
            }
            let new_dir = args[1];
            let new_dir = if new_dir == "~" {
                match env::var("HOME") {
                    Ok(home) => home,
                    Err(_) => {
                        eprintln!("HOME environment variable not set");
                        continue;
                    }
                }
            } else {
                new_dir.to_string()
            };

            if let Err(_err) = env::set_current_dir(&new_dir) {
                println!("{}: No such file or directory", new_dir);
            }
            continue;
        }        

        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let cmd_name = parts[0];
        let cmd_args = &parts[1..];

        if let Some(path) = find_in_path(cmd_name) {
            match Command::new(path).args(cmd_args).status() {
                Ok(status) => {
                    if !status.success() {
                        eprintln!("{}: command failed with status {}", cmd_name, status);
                    }
                }
                Err(err) => {
                    eprintln!("{}: failed to execute: {}", cmd_name, err);
                }
            }
        } else {
            println!("{}: command not found", cmd_name);
        }
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

