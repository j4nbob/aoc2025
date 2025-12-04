//Advent of Code 2025, Rust Edition|j4nbob|December 2025
use std::fs;

pub fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect(&format!("Failed to read file: {}", file_path))
}

pub fn parse_string_to_ints(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}
