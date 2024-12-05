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
        let re = Regex::new(r#"(?<arg4>\\.)|'(?<arg2>.*?)'|"(?<arg3>.*?)"|(?<arg1>([^\\\s]+))"#)
            .unwrap();

        let mut command_split = re.captures_iter(input.trim());

        let command = command_split.next().unwrap()["arg1"].to_string();
        let args: Vec<String> = command_split
            .map(|x| {
                (String::new()
                    + x.name("arg1").map_or("", |x| x.as_str())
                    + x.name("arg2").map_or("", |x| x.as_str())
                    + x.name("arg3").map_or("", |x| x.as_str())
                    + x.name("arg4").map_or("", |x| x.as_str().split_at(1).1))
                .to_string()
            })
            .collect();

        match command.as_str() {
            "exit" => {
                let code = args[0].parse().unwrap();
                exit(code);
            }
            "echo" => {
                for i in 0..args.len() {
                    print!("{}", args[i]);
                    if i < args.len() - 1 && args[i + 1] != " " {
                        print!(" ");
                    }
                }
                println!();
                // println!("{}", args.join(" "));
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
