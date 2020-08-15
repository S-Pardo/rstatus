#![feature(duration_zero)]

use crate::feature::Feature;
use std::sync::mpsc::{Sender, channel};
use std::{thread, process};
use std::time::Duration;
use std::thread::JoinHandle;
use std::ops::Deref;
use std::process::Command;
use std::io::Read;

mod feature;
mod xset;
mod config;

pub fn run() {
    let (sender, receiver) = channel();
    let mut features = initialize_features(config::scripts(), &sender);
    let mut timers = Vec::new();

    for feature in features.iter() {
        println!("{} time: {}", feature.id, feature.time.is_zero());
        if !feature.time.is_zero() {
            timers.push(create_timer(feature.id as i32, feature.sender.clone(), feature.time));
        }
    }
    thread::spawn(move || {
        loop {
            listen_audio(0, sender.clone());
            thread::sleep(Duration::from_secs(2));
        }
    });
    println!("timer creados");
    while let Ok(message) = receiver.recv() {
        match message {
            _ => update_features(message, &mut features),
        }
    }
}

fn listen_audio(id: u32, sender: Sender<i32>) {
    let mut command = Command::new("stdbuf");
    command.args(&["-oL", "alsactl", "monitor", "pulse"]);

    let mut monitor = command.stdout(process::Stdio::piped())
        .spawn().unwrap().stdout.unwrap();

    let mut buffer = [0; 1024];
    loop {
        if let Ok(bytes) = monitor.read(&mut buffer) {
            // reader has reached end-of-life -> thread gets killed
            if bytes == 0 {
                break;
            }

            sender.send(id as i32);
        }

        thread::sleep(Duration::from_millis(100));
    };
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
            println!("{}", message);
            thread::sleep(time);
        }
    })
}

/*fn get_outputs(features: Vec<Feature>) -> String {
    let mut data = String::new();
    for mut feature in features {
        data += &format!("{}{}",feature.execute(), config::DELIMITER);
    }
    data.truncate(data.len()-config::DELIMITER.len());
    data
}*/

fn initialize_features(config: Vec<(&str, i32)>, sender: &Sender<i32>) -> Vec<Feature> {
    let mut features = Vec::new();
    for (i, feature) in config.iter().enumerate() {
        let feature = *feature;
        let path = feature.0;
        let time = feature.1;
        let mut aux = Feature::new(i as u32, path, Duration::from_secs(time as u64), sender.clone());
        aux.update();
        features.push(aux);
    }
    features
}