use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(dayX)]
pub fn input_generator(input: &str) -> Vec<u32> {
    vec![]
}

#[aoc(dayX, part1)]
pub fn part1(input: &[u32]) -> u32 {
    0
}

#[aoc(dayX, part2)]
pub fn part2(input: &[u32]) -> u32 {
    0
}

fn helper() {}

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_helper() {
        super::get_regex_example();
        let input = INPUT.lock().unwrap();
    }

    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    #[allow(dead_code)]
    static INPUT: Lazy<Mutex<Vec<u32>>> = Lazy::new(|| Mutex::new(vec![]));
}
