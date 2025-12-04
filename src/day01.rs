//Advent of Code 2025, Rust Edition|j4nbob|December 2025
use crate::util;

pub fn parse_replace(input: &str) -> String {
    input.replace('R', "").replace('L', "-")
}

pub fn solve() {
    println!("Day 01:");
    println!("part1/daSum={}", part1());
    part2();
}

fn part1() -> i32 {
    let input = util::read_input("inputs/day01_part1.txt");
    let parsed = parse_replace(&input);
    let numbers = util::parse_string_to_ints(&parsed);
    let mut dial = 50;
    let mut result: i32 = 0;
    for current in &numbers {
        dial = (dial + current).rem_euclid(100);
        if dial == 0 {
            result += 1;    
        }    
    }
    return result;
}

fn part2() {
   // let input = util::read_input("inputs/day01_part2.txt");
   // println!("  Part 2: Read {} bytes", input.len());
}
