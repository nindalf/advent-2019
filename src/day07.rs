use crate::computer::Computer;
use itertools::Itertools;
use std::cmp;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn max_signal(input: &[i32]) -> i32 {
    let mut max_output = 0;
    for settings in (0..5).permutations(5) {
        let mut prev_output = 0;
        for setting in settings {
            let mut amplifier = Computer::new(&input);
            prev_output = amplifier.compute(&vec![setting, prev_output]);
        }
        max_output = cmp::max(max_output, prev_output);
    }
    max_output
}

#[aoc(day7, part2)]
pub fn feedback_max_signal(input: &[i32]) -> i32 {
    let mut max_output = 0;
    for settings in (5..10).permutations(5) {
        let mut prev_output = 0;
        let mut amplifiers = Vec::with_capacity(5);
        // initialize
        for setting in settings {
            let mut amplifier = Computer::new(&input);
            prev_output = amplifier.compute(&vec![setting, prev_output]);
            amplifiers.push(amplifier);
        }
        for i in 0.. {
            let current_amplifier = &mut amplifiers[i % 5];
            if current_amplifier.is_halted() {
                break;
            }
            prev_output = current_amplifier.compute(&vec![prev_output]);
        }
        max_output = cmp::max(max_output, prev_output);
    }
    max_output
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_max_signal() {
        let input = super::input_generator("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(super::max_signal(&input), 43210);

        let input = super::input_generator(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        assert_eq!(super::max_signal(&input), 54321);

        let input = super::input_generator("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        assert_eq!(super::max_signal(&input), 65210);
    }

    #[test]
    fn test_feedback_max_signal() {
        let input = super::input_generator(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(super::feedback_max_signal(&input), 139629729);

        let input = super::input_generator("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
        assert_eq!(super::feedback_max_signal(&input), 18216);
    }
}
