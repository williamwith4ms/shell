use std::io::{Write, stdin, stdout};
use std::process::Command;

use builtins::get_builtins;

mod builtins;
mod utils;

use utils::ShellError;

fn main() {
    const SHELL_PREFIX: &str = "> ";

    loop {
        print!("{} {SHELL_PREFIX}", utils::format_working_dir());
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command_line = match CommandLine::parse(&input) {
            Ok(command) => command,
            Err(err) => {
                eprintln!("{}", err);
                continue;
            }
        };

        match launch(&command_line) {
            Ok(Some(status)) if !status.success() => {
                eprintln!("{}", ShellError::CommandFailed(status));
            }
            Ok(_) => {}
            Err(err) => {
                eprintln!("rshell: {}", err);
            }
        }
    }
}

struct CommandLine {
    program: String,
    args: Vec<String>,
}

impl CommandLine {
    fn parse(input: &str) -> Result<Self, ShellError> {
        let mut parts = input.split_whitespace();
        let program = parts.next().ok_or(ShellError::ParseError)?;
        let args = parts.map(|s| s.to_string()).collect();
        Ok(CommandLine {
            program: program.to_string(),
            args,
        })
    }
}

fn launch(command: &CommandLine) -> Result<Option<std::process::ExitStatus>, ShellError> {
    let builtins = get_builtins();

    if let Some(builtin) = builtins.get(command.program.as_str()) {
        builtin(&command.args);
        Ok(None)
    } else {
        match execute(command) {
            Ok(status) => Ok(Some(status)),
            Err(ShellError::IoError(ref e)) if e.kind() == std::io::ErrorKind::NotFound => {
                Err(ShellError::CommandNotFound(command.program.clone()))
            }
            other => other.map(Some),
        }
    }
}

fn execute(command: &CommandLine) -> Result<std::process::ExitStatus, ShellError> {
    let mut child = Command::new(&command.program).args(&command.args).spawn()?;

    let status = child.wait()?;
    Ok(status)
}
