//Advent of Code 2025, Rust Edition|j4nbob|December 2025
use crate::util;

//YIKES
fn add_invalids(_begin: i64, _end: i64) -> i64 {
    let numbers: Vec<i64> = (_begin..=_end).collect();
    let mut invalids: i64 = 0;

    for number in numbers {
        let num_str = number.to_string();
        let len = num_str.len();        
        if len == 2{  
            let chars: Vec<char> = num_str.chars().collect();
            if chars.len() == 2 && chars[0] == chars[1] {
                invalids += number;
            }
        }else{          
            if len % 2 != 0 {
                continue;   //odd len can't have repeating patterns according to Advent of Code rules
            }
            let pattern_size = len / 2;
            let pattern = &num_str[0..pattern_size];
            let mut is_repeating = true;
            // i hate chunking
            for chunk_start in (pattern_size..len).step_by(pattern_size) {
                if &num_str[chunk_start..chunk_start + pattern_size] != pattern {
                    is_repeating = false;
                    break;
                }
            }            
            if is_repeating {
                invalids += number;
                //println!("Found repeating pattern number: {} in {}", pattern, num_str) ;
            }
        }
    }
    return invalids;
}

fn second_add_invalids(_begin: i64, _end: i64) -> i64 {
    let numbers: Vec<i64> = (_begin..=_end).collect();
    let mut invalids: i64 = 0;

    for number in numbers {
        let num_str = number.to_string();
        let len = num_str.len();        
        
        if len == 2{    //simple case for length 2
            let chars: Vec<char> = num_str.chars().collect();
            if chars.len() == 2 && chars[0] == chars[1] {
                //println!("Found repeating pattern number: {} in {}", chars[0], num_str) ;
                invalids += number;
            }
        }else{        
            // Try all possible pattern sizes
            for pattern_size in 1..=len/2 {
                if len % pattern_size != 0 {
                    continue;  
                }
                let pattern = &num_str[0..pattern_size];
                let mut is_repeating = true;
                for chunk_start in (pattern_size..len).step_by(pattern_size) {
                    if &num_str[chunk_start..chunk_start + pattern_size] != pattern {
                        is_repeating = false;
                        break;
                    }
                }
                if is_repeating {
                    invalids += number;
                    //println!("Found repeating pattern number: {} in {}", pattern, num_str);
                    break; 
                }
            }
        }
    }
    return invalids;
}


fn part1() -> i64 {
    let input = util::read_input("inputs/day02_part1.txt");
    let parts: Vec<&str> = input.split(',').collect();
    let mut invalids = 0;   
    for part in parts {
        //println!("Processing : {}", part);
        let range: Vec<&str> = part.split('-').collect();
        let begin: i64 = range[0].parse().unwrap();
        let end: i64 = range[1].parse().unwrap();
        //this'll be ugly
        invalids += add_invalids(begin, end);
    }
    return invalids;   
}

fn part2() -> i64 {
    let input = util::read_input("inputs/day02_part1.txt");
    let parts: Vec<&str> = input.split(',').collect();
    let mut invalids = 0;   
    for part in parts {
        let range: Vec<&str> = part.split('-').collect();
        let begin: i64 = range[0].parse().unwrap();
        let end: i64 = range[1].parse().unwrap();
        invalids += second_add_invalids(begin, end);
    }
    return invalids;   
}




pub fn solve() {
    println!("Day 02:");
    println!("part1/daSum={}", part1());
    println!("part2/daSum={}", part2());
}