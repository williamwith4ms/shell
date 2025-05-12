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

        let command_line = match CommandLine::parse(&input) {
            Some(command) => command,
            None => continue,
        }; 

        let status = launch(&command_line);

        match status {
            Some(status) => {if !status.success() {println!("Exited with: {}", status)}},
            None => continue,
        } 
            
    }
}

struct CommandLine {
    program: String,
    args: Vec<String>,
}

impl CommandLine {
    fn parse(input: &str) -> Option<Self> {
        let mut parts = input.split_whitespace();
        let program = parts.next()?.to_string();
        let args = parts.map(|s| s.to_string()).collect();
        Some(CommandLine { program, args })
    }
}


fn launch(command: &CommandLine) -> Option<std::process::ExitStatus> {
    let builtins = get_builtins();

    if let Some(builtin) = builtins.get(command.program.as_str()) {
        builtin(&command.args);
        None 
    } else {
        execute(command)
    }
}

fn execute(command: &CommandLine) -> Option<std::process::ExitStatus>{
    let mut child = Command::new(&command.program)
        .args(&command.args)
        .spawn()
        .unwrap();

    Some(child.wait().unwrap())
}

