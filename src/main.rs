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

        match command {
            "exit" => {
                let code = args[0].parse().unwrap();
                exit(code);
            }
            "echo" => {
                println!("{}", args.join(" "));
            }
            "type" => {
                sh_type(args[0]);
            }
            _ => {
                println!("{}: command not found", command);
            }
        }
    }
}

fn sh_type(arg: &str) {
    let builtin_commads = ["exit", "echo", "type"];
    if builtin_commads.contains(&arg) {
        println!("{} is a shell builtin", arg);
    } else {
        println!("{}: not found", arg);
    }
}
