use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use crate::commands::command::*;
use std::env;


pub struct Program {
    pub path: String,
    pub name: String,
}

pub struct State {
    pub commands: HashMap<String, Rc<dyn Command>>
    //pub custom_cmd
}

impl State {
    pub fn new() -> Self {
        State { commands: HashMap::new() }
    }

    pub fn register_command<C: Command + 'static>(&mut self, cmd: C) {
        self.commands.insert(cmd.name().to_string(), Rc::new(cmd));
    }

    pub fn find_command(&self, cmd: &str) -> Option<&dyn Command> {
        if let Some(c) = self.commands.get(cmd).map(|c| c.as_ref()) {
            Some(c)
        } else {
            None
        }
    }

    pub fn find_program(&self, cmd: &str) -> Option<Program> {
        let path_var = env::var("PATH").ok()?;
        let paths = env::split_paths(&path_var);

        for dir in paths {
            let full_path = dir.join(cmd);
            if full_path.is_file() && is_executable(&full_path) {
                return Some(Program { name: cmd.to_string(), path: full_path.to_string_lossy().to_string() })
            }
        }
        
        None
    }

    pub fn get_home(&self) -> Option<String> {
        env::var("HOME").ok()
    }

    pub fn get_cwd(&self) -> Option<PathBuf> {
        env::current_dir().ok()
    }
}


fn is_executable(path: &PathBuf) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}