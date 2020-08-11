use std::process::Command;

pub struct Generic {
    command: Command,
}

impl Generic {
    pub fn new(path: String) -> Self {
        Generic { command: Command::new(path) }
    }

    pub fn execute(&mut self) -> String {
        let output = self.command.output().unwrap();
        let mut data = String::from_utf8(output.stdout).unwrap();
        data.trim().to_string()
    }
}