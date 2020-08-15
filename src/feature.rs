use std::process::Command;
use std::sync::mpsc::Sender;
use std::time::Duration;

pub struct Feature {
    pub id: u32,
    pub command: Command,
    pub sender: Sender<i32>,
    pub time: Duration,
    pub data: String,
}

impl Feature {
    pub fn new(id: u32, path: &str, time: Duration,sender: Sender<i32>) -> Self {
        Feature {
            id: id,
            command: Command::new(path),
            sender: sender,
            time: time,
            data: String::new(),
        }
    }

    pub fn update(&mut self) {
        let output = self.command.output().unwrap();
        let mut data = String::from_utf8(output.stdout).unwrap();
        self.data = data.trim().to_string();
    }

}