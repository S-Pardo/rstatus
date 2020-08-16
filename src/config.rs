
pub const DELIMITER: &'static str = " | ";


pub fn scripts() -> Vec<(&'static str, i32)> {
    vec![
        ("volume", 0),
        ("xbacklight -get", 0),
        ("battery", 5),
        ("cpu", 10),
        ("internet", 5),
        ("clock", 60),
    ]
}
