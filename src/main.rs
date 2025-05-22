mod commands;
mod state;
mod error;
mod shell_io;

use shell_io::*;
use error::*;
use commands::command::*;
use std::process::ExitCode;
use state::{Program, State};


fn main() -> Result<ExitCode, ShellError> {

    let mut state = State::new();
    let mut io = ShellIO::new();
    register_commands(&mut state);

    loop {
        let (cmd, args) = io.read_input()?;


        let result = if let Some(command) = state.find_command(&cmd) {
            command.run(&args, &state, &mut io)
        } else if let Some(p) = state.find_program(&cmd) {
            run_program(&p, &args, &mut io)
        } else {
            Err(ShellError::CommandNotFound(cmd))
        };

        match result {
            Ok(CommandResult::Success) => continue,
            Ok(CommandResult::Exit(code)) => return Ok(code),
            Err(e) => io.error(e)?,
        }
    }
}


fn run_program(p: &Program, args: &[String], io: &mut ShellIO) -> Result<CommandResult, ShellError> {
    use std::process::Command;
    
    let output = Command::new(&p.name)
        .args(args)
        .output()?; // Runs and waits for it to finish

    io.write_program_output(output)?;

    Ok(CommandResult::Success)
}