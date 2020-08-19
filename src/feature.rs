use std::process::Command;
use std::sync::mpsc::Sender;
use std::time::Duration;

pub struct Feature {
    pub name: String,
    pub id: u32,
    pub command: Command,
    pub sender: Sender<i32>,
    pub time: Duration,
    pub data: String,
}

impl Feature {
    pub fn new(id: u32, path: &str, time: Duration,sender: Sender<i32>) -> Self {
        let mut path: Vec<&str> = path.split(" ").collect();
        let mut command = Command::new(path. get(0).unwrap());
        let mut name: String = path.get(0).unwrap().to_string();
        if path.len() > 1 {
            name = path.remove(0).to_string();
            for arg in path {
                command.arg(arg);
            }
        }
        Feature {
            name: name,
            id: id,
            command: command,
            sender: sender,
            time: time,
            data: String::new(),
        }
    }

    pub fn update(&mut self) {
        let output = self.command.output().unwrap();
        self.data = String::from_utf8(output.stdout).unwrap().trim().to_string();
    }

}