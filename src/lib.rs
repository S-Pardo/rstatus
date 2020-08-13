use crate::feature::Generic;
use std::{thread, time};

mod feature;
mod xset;
mod config;

pub fn run() {
    signal_wrapper(44, interrupt);
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

fn interrupt(_: u8) {
    println!("interrupted");
    xset::XWindow::init().unwrap().render(get_outputs(config::get_scripts()));
}

fn signal_wrapper(sig: u8, handler: fn(u8)) -> fn(u8) {
    unsafe {
        signal(sig, handler)
    }
}

extern "C" {
    fn signal(sig: u8, handler: fn(u8)) -> fn(u8);
}