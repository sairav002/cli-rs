use std::env;

use super::command::*;
use super::*;

pub struct Cd;

impl Command for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn usage(&self) -> &str {
        ""
    }

    fn run(&self, args: &[String], state: &State, _: &mut ShellIO) -> Result<CommandResult,ShellError> {
        let target_path = if let Some(path) = args.first() {
            if path.starts_with('/') {
                path.to_string()
            } else if let Some(p) = path.strip_prefix('~') {
                let home = state.get_home().ok_or(ShellError::Message("HOME not set".to_string()))?;
                format!("{home}{p}")
            } else {
                let current_dir = state.get_cwd().ok_or(ShellError::Message("Failed to get current directory".to_string()))?;
                format!("{}/{}", current_dir.display(), args[0])
            } 
        } else {
            state.get_home().ok_or(ShellError::Message("HOME not set".to_string()))?
        };

        env::set_current_dir(&target_path).map_err(|_| ShellError::DirectoryNotFound(format!("cd: {target_path}")))?;

        Ok(CommandResult::Success)
    }
}