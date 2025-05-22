use super::command::*;
use super::*;

pub struct Pwd;

impl Command for Pwd {
    fn name(&self) -> &str {
        "pwd"
    }

    fn usage(&self) -> &str {
        ""
    }

    fn run(&self, _: &[String], _: &State, io: &mut ShellIO) -> Result<CommandResult,ShellError> {
        let output = std::env::current_dir()?.to_string_lossy().to_string();
        io.write(&output)?;
        Ok(CommandResult::Success)
    }
}