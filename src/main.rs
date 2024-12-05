#[allow(unused_imports)]
use regex::Regex;
use std::{
    env,
    io::{self, Write},
    path::{Path, PathBuf},
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
        // let re = Regex::new(r#"(?<arg4>\\.)|'(?<arg2>.*?)'|"(?<arg3>.*?)"|(?<arg1>([^\\\s]+))"#)
        // .unwrap();

        // let mut command_split = re.captures_iter(input.trim());

        let parts: Vec<String> = parse_arguments(input.trim());
        let command = &parts[0];
        let args = parts[1..].to_vec();

        match command.as_str() {
            "exit" => {
                let code = args[0].parse().unwrap();
                exit(code);
            }
            "echo" => {
                for i in 0..args.len() {
                    print!("{}", args[i]);
                    if args[i] != " " && i < args.len() - 1 && args[i + 1] != " " {
                        print!(" ");
                    }
                }
                println!();
            }
            "type" => {
                sh_type(&args[0]);
            }
            "pwd" => {
                let current_dir = env::current_dir().unwrap();
                println!("{}", current_dir.display());
            }
            "cd" => {
                cd(&args[0]);
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

fn parse_arguments(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escape_next = false;

    for c in input.chars() {
        if escape_next {
            current.push(c);
            escape_next = false;
            continue;
        }
        match c {
            '\\' if !in_single_quotes && !in_double_quotes => {
                // Escape next character inside double quotes
                escape_next = true;
            }
            '"' if in_double_quotes => {
                // End double-quoted argument
                in_double_quotes = false;
                args.push(current);
                current = String::new();
            }
            '"' if !in_single_quotes => {
                // Start double-quoted argument
                in_double_quotes = true;
            }
            '\'' if in_single_quotes => {
                // End single-quoted argument
                in_single_quotes = false;
                args.push(current);
                current = String::new();
            }
            '\'' if !in_double_quotes => {
                // Start single-quoted argument
                in_single_quotes = true;
            }
            ' ' if !in_single_quotes && !in_double_quotes => {
                // Space outside quotes ends the current argument
                if !current.is_empty() {
                    args.push(current);
                    current = String::new();
                }
            }
            _ => {
                // Add character to the current argument
                current.push(c);
            }
        }
    }
    // Push the last argument if there's any
    if !current.is_empty() {
        args.push(current);
    }
    args
}

fn cd(arg: &String) {
    let path = if arg == "~" {
        env::var("HOME").unwrap()
    } else {
        arg.to_string()
    };
    let path = Path::new(&path);
    if path.exists() {
        env::set_current_dir(path).unwrap();
    } else {
        println!("cd: {}: No such file or directory", arg);
    }
}

fn run_exec(path: PathBuf, args: Vec<String>) {
    let mut c = Command::new(path).args(args).spawn().unwrap();
    c.wait().unwrap();
}

fn sh_type(comm: &String) {
    let builtin_commads = ["exit", "echo", "type", "pwd"];

    if builtin_commads.contains(&comm.as_str()) {
        println!("{} is a shell builtin", comm);
        return;
    }

    if let Some(path) = search_in_path(comm) {
        println!("{} is {}", comm, path.display());
        return;
    }

    println!("{}: not found", comm);
}

fn search_in_path(comm: &str) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap();
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
