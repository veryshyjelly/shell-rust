#[allow(unused_imports)]
use regex::Regex;
use std::{
    io::{self, Write},
    path::PathBuf,
    process::{exit, Command},
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
            comm => {
                if let Some(path) = search_in_path(comm) {
                    run_exec(path, args);
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}

fn run_exec(path: PathBuf, args: Vec<&str>) {
    let mut c = Command::new(path).args(args).spawn().unwrap();
    c.wait().unwrap();
}

fn sh_type(comm: &str) {
    let builtin_commads = ["exit", "echo", "type"];

    if builtin_commads.contains(&comm) {
        println!("{} is a shell builtin", comm);
        return;
    }

    if let Some(path) = search_in_path(comm) {
        println!("{} is {}", comm, path.as_os_str().to_str().unwrap());
        return;
    }

    println!("{}: not found", comm);
}

fn search_in_path(comm: &str) -> Option<PathBuf> {
    let path = std::env::var("PATH").unwrap();
    let folders: Vec<&str> = path.split(":").collect();
    for folder in folders {
        let dir = std::fs::read_dir(folder).unwrap();
        for f in dir {
            let f = f.unwrap();
            if f.file_name().into_string().unwrap() == comm {
                return Some(f.path());
            }
        }
    }
    return None;
}
