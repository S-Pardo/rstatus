use crate::feature::Generic;
use std::{thread, time};

mod feature;
mod xset;
mod config;

pub fn run() {
    loop {
        println!("{}", get_outputs(config::get_scripts()));
        xset::XWindow::init().unwrap().render(get_outputs(config::get_scripts()));
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn get_outputs(features: Vec<Generic>) -> String {
    let mut data = String::new();
    for mut feature in features {
        data += &format!("{}{}",feature.execute(), config::DELIMITER);
    }
    data.truncate(data.len()-config::DELIMITER.len());
    data
}