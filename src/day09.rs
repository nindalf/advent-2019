use crate::computer::Computer;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn boost_code(input: &[i64]) -> i64 {
    compute(&input, &vec![1])[0]
}

#[aoc(day9, part2)]
pub fn distress_signal(input: &[i64]) -> i64 {
    compute(&input, &vec![2])[0]
}

pub fn compute(instructions: &[i64], input: &[i64]) -> Vec<i64> {
    let mut computer = Computer::new(instructions);
    let mut result = Vec::new();
    let mut output = computer.compute(input);
    result.push(output);
    while !computer.is_halted() {
        output = computer.compute(&vec![]);
        // TODO fix this hack
        if !computer.is_halted() {
            result.push(output);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_complete_computer() {
        let instructions =
            super::input_generator("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        assert_eq!(
            super::compute(&instructions, &vec![]),
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );

        let instructions = super::input_generator("1102,34915192,34915192,7,4,7,99,0");
        assert_eq!(super::compute(&instructions, &vec![]), vec![1219070632396864]);

        let instructions = super::input_generator("104,1125899906842624,99");
        assert_eq!(super::compute(&instructions, &vec![]), vec![1125899906842624]);
    }
}
