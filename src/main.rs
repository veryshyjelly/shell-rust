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

        let mut command_split = input.trim().split(" ");

        let command = command_split.next().unwrap();
        let args: Vec<&str> = command_split.collect();

        if command == "exit" {
            let code = args[0].parse().unwrap();
            exit(code);
        } else if command == "echo" {
            println!("{}", args.join(" "));
        } else {
            println!("{}: invalid command", command);
        }
    }
}
