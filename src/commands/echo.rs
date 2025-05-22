use super::command::*;
use super::*;

pub struct Echo;

impl Command for Echo {
    fn name(&self) -> &str {
        "echo"
    }

    fn usage(&self) -> &str {
        ""
    }

    fn run(&self, args: &[String], _: &State, io: &mut ShellIO) -> Result<CommandResult,ShellError> {
        if args.is_empty() {
            return Err(ShellError::BadUsage(self.usage().to_string()))
        } 

        let output = args.join(" ");
        io.write(&output)?;
        Ok(CommandResult::Success)
    }
}