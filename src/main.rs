#[allow(unused_imports)]
use regex::Regex;
use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let command = input.trim();

        let re_exit = Regex::new(r"exit (?<code>\w+)").unwrap();

        if let Some(code) = re_exit.captures(command) {
            let code_point: i32 = code["code"].parse().unwrap();
            exit(code_point);
        } else {
            println!("{}: command not found", command);
        }
    }
}
