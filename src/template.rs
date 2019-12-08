use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(dayX)]
pub fn input_generator(input: &str) -> Vec<i32> {
    vec![]
}

#[aoc(dayX, part1)]
pub fn part1(input: &[i32]) -> i32 {
    0
}

#[aoc(dayX, part2)]
pub fn part2(input: &[i32]) -> i32 {
    0
}

fn helper() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_helper() {
        
    }

    #[test]
    fn test_part1() {
        
    }

    #[test]
    fn test_part2() {
        
    }
}


use once_cell::sync::Lazy;
use std::sync::Mutex;

#[allow(dead_code)]
static INPUT: Lazy<Mutex<Vec<u32>>> = Lazy::new(|| Mutex::new(vec![]));

fn get_regex_example() {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("Step (?P<source>[A-Z]) must be  (?P<destination>[A-Z]) can begin.")
                .unwrap();
    }
    let caps = RE.captures("Step A must be X can begin").unwrap();
    let source = caps["source"];
    let destination = caps["destination"];
}
