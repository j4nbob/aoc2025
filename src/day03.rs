//Advent of Code 2025, Rust Edition|j4nbob|December 2025
use crate::util;
pub fn solve() {
    println!("Day 03:");
    println!("part1/daSum={}", part1());
    println!("part2/daSum={}", part2());
    
}
fn part1() -> i32 {
    let input = util::read_input("inputs/day03_part1.txt");
    let mut result = 0;
    for line in input.lines() {
        let digits: Vec<char> = line.chars().collect();
        let len = digits.len();
        let mut max_left_digit = '0';
        let mut max_left_pos = 0;
        let mut max_right_digit = '0';
        let mut max_right_pos = 0;
        
        for (i, &digit) in digits.iter().enumerate() {
            if i < len - 1 && digit > max_left_digit {
                max_left_digit = digit;
                max_left_pos = i;
            }
        }
        if max_left_pos + 1 < len {
            for i in (max_left_pos + 1..len).rev() {
                let digit = digits[i];
                if digit >= max_right_digit {
                    max_right_digit = digit;
                    max_right_pos = i;
                }
            }
        }
        let combined = format!("{}{}", max_left_digit, max_right_digit).parse::<i32>().unwrap();
        result += combined;
    }
    return result;
}


fn part2() -> i64 {
    let input = util::read_input("inputs/day03_part1.txt");
    let mut result = 0;
    for line in input.lines() {
        let digits: Vec<char> = line.chars().collect();
        let len = digits.len();
        if len < 12 {
            continue; // Skip if less than 12 digits
        }        
        let mut selected: Vec<char> = Vec::new();
        let mut remaining = 12;
        let mut start_pos = 0;
        while remaining > 0 {
            let mut max_digit = '0';
            let mut max_pos = start_pos;            
            let search_end = len - remaining + 1;
            for i in start_pos..search_end {
                if digits[i] > max_digit {
                    max_digit = digits[i];
                    max_pos = i;
                }
            }
            selected.push(max_digit);
            start_pos = max_pos + 1;
            remaining -= 1;
        }
        let number_str: String = selected.iter().collect();
        let largest_number = number_str.parse::<i64>().unwrap_or(0);
        result += largest_number;
    }
    return result;
}
