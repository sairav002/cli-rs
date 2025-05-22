use std::{io::{self, Stderr, Stdin, Stdout, Write}, process::Output};

use crate::error::ShellError;

pub struct ShellIO {
    pub stdin:  Stdin,
    pub stdout: Stdout,
    pub stderr: Stderr,
}


impl ShellIO {
    pub fn new() -> Self {
        ShellIO {
            stdin: io::stdin(),
            stdout: io::stdout(),
            stderr: io::stderr(),
        }
    } 

    pub fn read_input(&mut self) -> Result<(String, Vec<String>), ShellError> {

        write!(self.stdout, "$ ")?;
            self.stdout.flush()?;
    
        let mut input = String::new();
        self.stdin.read_line(&mut input)?;
    
        let mut parts = input.split_whitespace();
    
        let cmd = match parts.next() {
            Some(s) => s.to_string(),
            None => return Ok((String::new(), vec![]))
        };
    
        let args: Vec<String> = parts.map(|s| s.to_string()).collect();
    
    
        Ok((cmd, args))
    }

    pub fn write(&mut self, output: &str) -> Result<(), ShellError> {
        writeln!(self.stdout, "{output}")?;
        Ok(())
    }

    pub fn error(&mut self, e: ShellError) -> Result<(), ShellError> {
        writeln!(self.stderr, "{e}")?;
        Ok(())
    }

    pub fn write_program_output(&mut self, output: Output) -> Result<(), ShellError>{
        if !output.stdout.is_empty() {
            self.stdout.write_all(&output.stdout)?;
        }

        if !output.stderr.is_empty() {
            self.stderr.write_all(&output.stderr)?;
        }

        Ok(())
    }
}