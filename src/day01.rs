use std::cmp;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn total_fuel_required(masses: &[i32]) -> i32 {
    masses.iter().map(|mass| fuel_required(*mass)).sum()
}

#[aoc(day1, part2)]
pub fn next_level_total_fuel_required(masses: &[i32]) -> i32 {
    masses
        .iter()
        .map(|mass| next_level_fuel_required(*mass))
        .sum()
}

fn fuel_required(mass: i32) -> i32 {
    cmp::max(mass / 3 - 2, 0)
}

fn next_level_fuel_required(mass: i32) -> i32 {
    let mut result = 0;
    let mut fuel = cmp::max(mass / 3 - 2, 0);
    while fuel > 0 {
        result += fuel;
        fuel = cmp::max(fuel / 3 - 2, 0);
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fuel_required() {
        assert_eq!(super::fuel_required(12), 2);
        assert_eq!(super::fuel_required(14), 2);
        assert_eq!(super::fuel_required(1969), 654);
        assert_eq!(super::fuel_required(100756), 33583);
    }

    #[test]
    fn test_next_level_fuel_required() {
        assert_eq!(super::next_level_fuel_required(12), 2);
        assert_eq!(super::next_level_fuel_required(14), 2);
        assert_eq!(super::next_level_fuel_required(1969), 966);
        assert_eq!(super::next_level_fuel_required(100756), 50346);
    }
}
