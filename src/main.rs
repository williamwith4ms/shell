use std::io::{Write, stdin, stdout};
use std::process::{Command, Stdio};

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
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let cmds: Vec<CommandLine> = input
            .split('|')
            .map(str::trim)
            .map(CommandLine::parse)
            .collect::<Result<_, _>>().expect("TODO");

        if cmds.len() > 1 {
            if let Err(err) = pipeline(&cmds) {
                eprintln!("rshell: {}", err);
            }
        } else {
            match launch(&cmds[0]) {
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
}

fn pipeline(cmds: &[CommandLine]) -> Result<(), ShellError> {
    let mut processes: Vec<std::process::Child> = Vec::new();

    for (i, cmd_line) in cmds.iter().enumerate() {
        let mut cmd = Command::new(&cmd_line.program);
        cmd.args(&cmd_line.args);

        if i == 0 {
            cmd.stdout(Stdio::piped());
        } else if i == cmds.len() -1 {
            cmd.stdin(processes[i - 1].stdout.take().unwrap());
        } else {
            cmd.stdin(processes[i - 1].stdout.take().unwrap())
               .stdout(Stdio::piped());
        }

        processes.push(cmd.spawn()?);
    }

    for mut child in processes {
        child.wait()?;
    }

    Ok(())
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
