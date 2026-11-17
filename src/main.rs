mod file_system;
mod fs_rc;

use std::io::{self, Write};

use crate::fs_rc::FileSystem;

fn main() {
    let mut root = FileSystem::new();
    let mut input = String::new();

    loop {
        print!("{}> ", root.pwd());
        io::stdout().flush().unwrap();
        input.clear();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        match parse(&input) {
            Ok(Command::Ls) => root.ls(),
            Ok(Command::Mkdir(n)) => root.mkdir(n),
            Ok(Command::Touch(n)) => root.touch(n),
            Ok(Command::Pwd) => {
                let pwd = root.pwd();
                println!("{}", pwd);
            }
            Ok(Command::Cd(name)) => root.cd(name),
            Ok(Command::Tree) => root.tree(),
            Ok(Command::Rm(name)) => root.rm(&name),
            Ok(Command::Exit) => break,
            Err(e) => println!("{e}"),
        }
    }
}

enum Command {
    Ls,
    Pwd,
    Tree,
    Mkdir(String),
    Touch(String),
    Cd(String),
    Rm(String),
    Exit,
}

fn parse(line: &str) -> Result<Command, String> {
    let mut parts = line.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some("ls"), None) => Ok(Command::Ls),
        (Some("mkdir"), Some(name)) => Ok(Command::Mkdir(name.to_string())),
        (Some("touch"), Some(name)) => Ok(Command::Touch(name.to_string())),
        (Some("cd"), Some(name)) => Ok(Command::Cd(name.to_string())),
        (Some("pwd"), None) => Ok(Command::Pwd),
        (Some("tree"), None) => Ok(Command::Tree),
        (Some("rm"), Some(name)) => Ok(Command::Rm(name.to_string())),
        (Some("exit"), _) => Ok(Command::Exit),
        (None, _) => Err("empty line".to_string()),
        _ => Err("unknown command".to_string()),
    }
}
