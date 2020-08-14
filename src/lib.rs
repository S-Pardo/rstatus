use std::thread;
use std::process::Command;
use std::time::Duration;

mod config;

//Signal based in RTMIN = 34
static mut FEATURES: &[Feature] = &[
    Feature { command: "volume", signal: 44, time: 0 },
    Feature { command: "battery", signal: 0, time: 1 },
    Feature { command: "clock", signal: 0, time: 60 },
    Feature { command: "internet", signal: 37, time: 5 },
];

static mut STATUSBAR: &mut [String] = &mut [
    String::new(),
    String::new(),
    String::new(),
    String::new(),
];

struct Feature {
    command: &'static str,
    signal: u32,
    time: u32,
}

pub fn run() {
    let mut i = 0;
    setup_signals();
    get_cmds(-1);
    loop {
        get_cmds(i);
        set_root();
        thread::sleep(Duration::from_secs_f32(0.5));
        i += 1;
    }
}

fn setup_signals() {
    for feature in get_features() {
        if feature.signal > 0 {
            signal_wrapper(feature.signal, signal_handler);
        }
    }
}

fn set_root() {
    let mut xset = Command::new("xsetroot");
    match xset.arg("-name").arg(get_status()).spawn() {
        Ok(_) => {},
        Err(e) => eprintln!("{}", e),
    }
}

fn get_status() -> String {
    let mut sting = String::new();
    for feature in get_statusbar() {
        sting.push_str(feature);
        sting.push_str(config::DELIMITER);
    }
    sting.truncate(sting.len() - config::DELIMITER.len());
    sting
}

fn get_cmd(feature: &Feature, output: &mut String) {
    let cmd = Command::new(feature.command).output().unwrap();
    let data = String::from_utf8(cmd.stdout).unwrap();
    let data = data.trim();
    String::clear(output);
    //output.trim();
    output.push_str(data);
}

fn get_signal_cmds(signal: u32) {
    for (i, feature) in get_features().iter().enumerate() {
        if feature.signal == signal {
            get_cmd(feature, &mut get_statusbar()[i]);
        }
    }
}

fn get_cmds(time: i32) {
    for (i, feature) in get_features().iter().enumerate() {
        if (feature.time != 0 && time % feature.time as i32 == 0) || time == -1 {
            //println!("{}", feature.command);
            get_cmd(feature, &mut get_statusbar()[i]);
        }
    }
}

fn get_statusbar() -> &'static mut [String] {
    unsafe { STATUSBAR }
}

fn get_features() -> &'static [Feature] {
    unsafe { FEATURES }
}

fn signal_handler(signal: u32) {
    println!("interrupted by {}", signal);
    get_signal_cmds(signal);
    set_root();
    //xset::XWindow::init().unwrap().render(get_outputs(config::get_scripts()));
}

fn signal_wrapper(sig: u32, handler: fn(u32)) -> fn(u32) {
    unsafe {
        signal(sig, handler)
    }
}

extern "C" {
    fn signal(sig: u32, handler: fn(u32)) -> fn(u32);
}