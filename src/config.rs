use crate::feature::Generic;

pub const DELIMITER: &'static str = " | ";

pub fn get_scripts() -> Vec<Generic> {
    let scripts = scripts();
    let mut features: Vec<Generic> = Vec::new();
    for script in scripts {
        features.push(Generic::new(script.to_string()));
    }
    features
}

fn scripts() -> Vec<&'static str> {
    vec![
        "volume",
        "cpu",
        "battery",
        "clock",
    ]
}