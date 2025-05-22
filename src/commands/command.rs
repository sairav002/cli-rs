
use crate::{state::*, error::*, ShellIO};
use super::*;

pub trait Command {
    fn name(&self) -> &str;
    fn usage(&self) -> &str;
    fn run(&self, args: &[String], state: &State, io: &mut ShellIO) -> Result<CommandResult, ShellError>;
}

pub fn register_commands(state: &mut State) {
    state.register_command(Echo);
    state.register_command(Exit);
    state.register_command(Type);
    state.register_command(Pwd);
    state.register_command(Cd);
}