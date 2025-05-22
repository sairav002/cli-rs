use super::command::*;
use super::*;

pub struct Type;

impl Command for Type {
    fn name(&self) -> &str {
        "type"
    }

    fn usage(&self) -> &str {
        ""
    }

    fn run(&self, args: &[String], state: &State, io: &mut ShellIO) -> Result<CommandResult,ShellError> {
        if args.len() != 1 {
            return Err(ShellError::BadUsage(self.usage().to_string()))
        } 

        let cmd = &args[0];

        if state.find_command(cmd).is_some() {
            let output = format!("{} is a shell builtin", cmd);
            io.write(&output)?;
            return Ok(CommandResult::Success)
        }
        
        if let Some(p) = state.find_program(cmd) {
            let output = format!("{} is {}", cmd, p.path);
            io.write(&output)?;
            return Ok(CommandResult::Success)
        }
          
        Err(ShellError::Message(format!("{}: not found", cmd)))
    }
}
