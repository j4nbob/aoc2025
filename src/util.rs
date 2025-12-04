use std::fs;

pub fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect(&format!("Failed to read file: {}", file_path))
}
