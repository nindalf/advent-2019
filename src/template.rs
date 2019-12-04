use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Tree {

}

impl Tree {
    fn new() -> Tree {
        Tree{}
    }
}

#[allow(dead_code)]
fn simple() {

}

fn get_regex_example() {
    lazy_static! {
        static ref RE: Regex = Regex::new("Step (?P<source>[A-Z]) must be  (?P<destination>[A-Z]) can begin.").unwrap();
    }
    let caps = RE.captures("Step A must be X can begin").unwrap();
    let source = caps["source"];
    let destination = caps["destination"];
}

mod tests {
    #[test]
    fn test_simple() {
        super::get_regex_example();
        let input = INPUT.lock().unwrap();
    }

    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    #[allow(dead_code)]
    static INPUT: Lazy<Mutex<Vec<u32>>> = Lazy::new(|| {
        Mutex::new(vec![])
    });
}