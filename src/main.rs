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

        // println!("cmd:{}", input);

        // next look for cmd here I don't know but cmd and enum I feel it is the good option here
        // lets just follow the AI  agent here
        //

        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }

        let mut parts = input.trim().split_whitespace();
        let cmd = parts.next();
        let arg = parts.next();

        // mkdir
        // touch
        //
        //cd
        // pwd
        // tree
        // rm

        match (cmd, arg) {
            (Some("ls"), None) => {
                root.ls();
            }
            (Some("mkdir"), Some(name)) => {
                root.mkdir(name.to_string());
            }
            (Some("touch"), Some(name)) => {
                root.touch(name.to_string());
            }
            (Some("cd"), Some(name)) => {
                root.cd(name.to_string());
            }
            (Some("pwd"), None) => {
                let pwd = root.pwd();
                println!("{}", pwd);
            }
            (Some("tree"), None) => {
                root.tree();
            }
            (Some("rm"), Some(name)) => {
                root.rm(name);
            }
            (Some("exit"), _) => break,
            (None, _) => {}
            _ => println!("unknown command"),
        }
    }
}
