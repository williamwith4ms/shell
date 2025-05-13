pub fn format_working_dir() -> String {
    let wd = std::env::current_dir().unwrap();

    let home_path = match dirs::home_dir() {
        Some(home) => home.to_str().unwrap_or("").to_string(),
        None => "".to_string(),
    };

    let formatted = wd.to_str().unwrap_or("");

    if formatted.starts_with(&home_path) {
        formatted.replacen(&home_path, "~", 1)
    } else {
        formatted.to_string()
    }
}

#[derive(Debug)]
pub enum ShellError {
    IoError(std::io::Error),
    ParseError,
    CommandFailed(std::process::ExitStatus),
    CommandNotFound(String),
}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellError::IoError(error) => write!(f, "rshell: IO error: {}", error),
            ShellError::ParseError => write!(f, "rshell: failed to parse command line"),
            ShellError::CommandFailed(exit_status) => {
                write!(f, "rshell: command exited with status: {}", exit_status)
            }
            ShellError::CommandNotFound(command) => {
                write!(f, "rshell: Unknown command: {}", command)
            }
        }
    }
}

impl From<std::io::Error> for ShellError {
    fn from(err: std::io::Error) -> Self {
        ShellError::IoError(err)
    }
}
