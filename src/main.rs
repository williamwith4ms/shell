use std::io::{stdin, stdout, Write};
use std::process::Command;

use builtins::get_builtins;

mod builtins;

fn main() {
    const SHELL_PREFIX: &str = "> ";

    loop {
        print!("{SHELL_PREFIX}");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // everything after first whitespace is args 
        let mut split = input.trim().split_whitespace();
        let program = split.next().unwrap();
        let args = split;

        let status = launch(program, args);

        match status {
            Some(status) => {if !status.success() {println!("Exited with: {}", status)}},
            None => continue,
        } 
            
    }
}


fn launch<'a>(program: &str, args: impl Iterator<Item = &'a str>) -> Option<std::process::ExitStatus> {
    let builtins = get_builtins();

    if let Some(builtin) = builtins.get(program) {
        builtin();
        None 
    } else {
        execute(program, args)
    }
}

fn execute<'a>(program: &str, args: impl Iterator<Item = &'a str>) -> Option<std::process::ExitStatus>{
    let mut child = Command::new(program)
        .args(args)
        .spawn()
        .unwrap();

    Some(child.wait().unwrap())
}

