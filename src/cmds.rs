use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

pub fn cd(arg: &String) {
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

pub fn run_exec(path: PathBuf, args: Vec<String>) {
    let mut c = Command::new(path).args(args).spawn().unwrap();
    c.wait().unwrap();
}

pub fn sh_type(comm: &String) {
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

pub fn search_in_path(comm: &str) -> Option<PathBuf> {
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
