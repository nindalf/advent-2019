use crate::computer::Computer;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn test_systems(input: &[i64]) -> i64 {
    let mut computer = Computer::new(&input);
    let mut output = computer.compute(&vec![1]).unwrap();
    
    while let Some(new_output) = computer.compute(&vec![]) {
        println!("Day 5 needs additional loops");
        output = new_output;
    }
    output
}

#[aoc(day5, part2)]
pub fn test_aircon(input: &[i64]) -> i64 {
    let mut computer = Computer::new(&input);
    computer.compute(&vec![5]).unwrap()
}
