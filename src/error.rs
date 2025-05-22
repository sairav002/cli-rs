use std::{io, process::ExitCode};

#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("{0}: command not found")]
    CommandNotFound(String),
    #[error("{0}: bad usage")]
    BadUsage(String),
    #[error("{0}: No such file or directory")]
    DirectoryNotFound(String),
    #[error("{0}: I/O error")]
    Io(io::Error),
    #[error("{0}")]
    Message(String),
}

impl From<io::Error> for ShellError {
    fn from(err: io::Error) -> Self {
        ShellError::Io(err)
    }
}

pub enum CommandResult {
    Success,
    Exit(ExitCode),
}