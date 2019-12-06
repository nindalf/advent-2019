use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn compute_instructions(input: &[u32]) -> u32 {
    let mut counter: usize = 0;
    let mut results = input.to_vec();
    while counter < input.len() {
        let instruction = FromPrimitive::from_u32(results[counter]).unwrap();
        if instruction == Instruction::Stop {
            return results[0];
        }
        let op_1 = results[input[counter + 1] as usize];
        let op_2 = results[input[counter + 2] as usize];
        let result = compute(instruction, op_1, op_2);
        let destination = results[counter + 3] as usize;
        results[destination] = result;
        counter += 4;
    }
    results[0]
}

#[aoc(day2, part2)]
pub fn exhaustive_search(input: &[u32]) -> Option<u32> {
    let mut mut_input = input.to_vec();
    let desired_result = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            mut_input[1] = noun;
            mut_input[2] = verb;
            let computed_result = compute_instructions(&mut_input);
            if computed_result == desired_result {
                return Some(100 * noun + verb);
            }
        }
    }
    None
}

#[derive(FromPrimitive, PartialEq)]
enum Instruction {
    Addition = 1,
    Multiplication = 2,
    Stop = 99,
}

fn compute(instruction: Instruction, op_1: u32, op_2: u32) -> u32 {
    match instruction {
        Instruction::Addition => op_1 + op_2,
        Instruction::Multiplication => op_1 * op_2,
        _ => panic!("unimplemented instruction"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_computer() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(super::compute_instructions(&input), 3500);

        let input = vec![1, 0, 0, 0, 99];
        assert_eq!(super::compute_instructions(&input), 2);

        let input = vec![2, 3, 0, 3, 99];
        assert_eq!(super::compute_instructions(&input), 2);

        let input = vec![2, 4, 4, 5, 99, 0];
        assert_eq!(super::compute_instructions(&input), 2);

        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(super::compute_instructions(&input), 30);
    }
}
