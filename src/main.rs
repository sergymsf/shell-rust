use std::io::{self, Write};
use std::process;

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
                println!("{} not found", to_check);
            }
            continue;
        }

        println!("{}: command not found", command);
    }
}
