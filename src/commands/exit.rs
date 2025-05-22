use std::process::ExitCode;
use super::command::*;
use super::*;

pub struct Exit;

impl Command for Exit {
    fn name(&self) -> &str {
        "exit"
    }

    fn usage(&self) -> &str {
        ""
    }

    fn run(&self, _: &[String], _: &State, _: &mut ShellIO) -> Result<CommandResult,ShellError> {
        Ok(CommandResult::Exit(ExitCode::SUCCESS))
    }
}