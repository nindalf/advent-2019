use std::cmp;

#[allow(dead_code)]
fn fuel_required(mass: i32) -> i32 {
    cmp::max(mass / 3 - 2, 0)
}

#[allow(dead_code)]
fn next_level_fuel_required(mass: i32) -> i32 {
    let mut result = 0;
    let mut fuel = cmp::max(mass / 3 - 2, 0);
    while fuel > 0 {
        result += fuel;
        fuel = cmp::max(fuel / 3 - 2, 0);
    }
    result
}

#[allow(dead_code)]
fn total_fuel_required(masses: &[i32], fuel_calculator: &dyn Fn(i32) -> i32) -> i32 {
    masses.iter().map(|mass| fuel_calculator(*mass)).sum()
}

#[cfg(test)]
mod tests {
    use crate::utils;
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

    #[test]
    fn test_total_fuel_required() {
        let lines = utils::read_lines("data/day01.txt").unwrap();
        let input: Vec<i32> = lines
            .iter()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();
        assert_eq!(
            super::total_fuel_required(&input, &super::fuel_required),
            3232358
        );
        assert_eq!(
            super::total_fuel_required(&input, &super::next_level_fuel_required),
            4845669
        );
    }
}
