#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        stdin.read_line(&mut input).unwrap();
        input.pop();
        println!("{}: command not found", input);
        input.clear();
    }
}
