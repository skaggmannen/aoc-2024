use std::fs::read_to_string;

pub fn to_lines(data: &str) -> Vec<String> {
    data.lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn read_input(path: &str) -> String {
    read_to_string(path).unwrap()
}

pub fn trim_space(s: &str) -> Option<&str> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}