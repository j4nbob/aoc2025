//Advent of Code 2025, Rust Edition|j4nbob|December 2025
use crate::util;

pub fn parse_replace(input: &str) -> String {
    input.replace('R', "").replace('L', "-")
}

pub fn solve() {
    println!("Day 01:");
    let (first, second) = part1();
    println!("part1/daSum={}", first);
    println!("part2/daSum={}", second);
}

fn part1() -> (i32,i32) {
    let input = util::read_input("inputs/day01_part1.txt");
    let parsed = parse_replace(&input);
    let numbers = util::parse_string_to_ints(&parsed);
    let mut dial = 50;
    let mut result: i32 = 0;
    let mut count: i32 = 0;

    for current in &numbers {
        let dial_pos = dial + current;
        let previous_dial_pos = dial;
        dial = dial_pos.rem_euclid(100);
        count += dial_pos.abs() / 100;
        if dial_pos <= 0 && previous_dial_pos != 0 {
            count += 1;
        }
        if dial == 0 {
            result += 1;
        }
    }
    return (result,count);
}
