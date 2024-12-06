#[allow(unused_imports)]
use regex::Regex;
use std::{
    env,
    io::{self, Write},
    process::exit,
};

mod arg;
mod cmds;

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // let parts: Vec<String> = parse_arguments(input.trim());
        let parts = arg::Argument::try_from(input.trim()).unwrap();
        let command = &parts[0];
        let args = parts[1..].to_vec();

        match command.as_str() {
            "exit" => {
                let code = args[0].parse().unwrap();
                exit(code);
            }
            "echo" => {
                println!("{}", args.join(" "));
            }
            "type" => {
                cmds::sh_type(&args[0]);
            }
            "pwd" => {
                let current_dir = env::current_dir().unwrap();
                println!("{}", current_dir.display());
            }
            "cd" => {
                cmds::cd(&args[0]);
            }
            comm => {
                if let Some(path) = cmds::search_in_path(comm) {
                    cmds::run_exec(path, args);
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}
