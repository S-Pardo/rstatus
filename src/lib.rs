#![feature(duration_zero)]

use crate::feature::Feature;
use std::sync::mpsc::{Sender, channel};
use std::{thread, process};
use std::time::Duration;
use std::thread::JoinHandle;
use std::process::Command;
use std::io::Read;

mod feature;
mod config;

pub fn run() {
    let (sender, receiver) = channel();
    let mut features = initialize_features(config::scripts(), &sender);
    let mut timers = Vec::new();

    for feature in features.iter() {
        if !feature.time.is_zero() {
            timers.push(create_timer(feature.id as i32, feature.sender.clone(), feature.time));
        }
    }
    listen_audio(0, sender.clone());
    while let Ok(message) = receiver.recv() {
        match message {
            _ => update_features(message, &mut features),
        }
    }
}

fn listen_audio(id: u32, sender: Sender<i32>) {
    thread::spawn(move || {
        let mut command = Command::new("stdbuf");
        command.args(&["-oL", "alsactl", "monitor", "pulse"]);

        let mut monitor = command.stdout(process::Stdio::piped())
            .spawn().unwrap().stdout.unwrap();

        let mut buffer = [0; 1024];
        loop {
            if let Ok(_) = monitor.read(&mut buffer) {
                sender.send(id as i32);
            }
            thread::sleep(Duration::from_millis(100));
        };
    });
}

fn update_features(message: i32, features: &mut Vec<Feature>) {
    match message {
        id if id < features.len() as i32 && id >= 0 => {
            features[id as usize].update();
            update_statusbar(features);
        }
        _ => {}
    }
}

fn update_statusbar(features: &Vec<Feature>) {
    let mut xset = Command::new("xsetroot");
    xset.arg("-name");
    let mut status_str = String::new();

    for feature in features {
        status_str += &format!("{}{}", feature.data, config::DELIMITER);
    }
    status_str.truncate(status_str.len() - config::DELIMITER.len());
    xset.arg(status_str).spawn();
}

fn create_timer(message: i32, sender: Sender<i32>, time: Duration) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            sender.send(message);
            thread::sleep(time);
        }
    })
}

fn initialize_features(config: Vec<(&str, i32)>, sender: &Sender<i32>) -> Vec<Feature> {
    let mut features = Vec::new();
    for (i, feature) in config.iter().enumerate() {
        let feature = *feature;
        let mut aux = Feature::new(i as u32, feature.0, Duration::from_secs(feature.1 as u64), sender.clone());
        aux.update();
        features.push(aux);
    }
    features
}