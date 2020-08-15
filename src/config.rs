
pub const DELIMITER: &'static str = " | ";


pub fn scripts() -> Vec<(&'static str, i32)> {
    vec![
        ("volume", 0),
        ("cpu", 1),
        ("battery", 5),
        ("clock", 30),
    ]
}