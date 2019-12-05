use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
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

fn compute_instructions(input: &[u32]) -> Vec<u32> {
    let mut counter: usize = 0;
    let mut results = input.to_vec();
    while counter < input.len() {
        let instruction = FromPrimitive::from_u32(results[counter]).unwrap();
        if instruction == Instruction::Stop {
            return results;
        }
        let op_1 = results[input[counter + 1] as usize];
        let op_2 = results[input[counter + 2] as usize];
        let result = compute(instruction, op_1, op_2);
        let destination = results[counter + 3] as usize;
        results[destination] = result;
        counter += 4;
    }
    results
}

#[allow(dead_code)]
fn exhaustive_search(input: &[u32], desired_result: u32) -> Option<u32> {
    let mut mut_input = input.to_vec();
    for noun in 0..100 {
        for verb in 0..100 {
            mut_input[1] = noun;
            mut_input[2] = verb;
            let computed_result = compute_instructions(&mut_input)[0];
            if computed_result == desired_result {
                return Some(100 * noun + verb);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_computer() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(super::compute_instructions(&input), expected);

        let input = vec![1, 0, 0, 0, 99];
        let expected =  vec![2, 0, 0, 0, 99];
        assert_eq!(super::compute_instructions(&input), expected);

        let input = vec![2, 3, 0, 3, 99];
        let expected = vec![2, 3, 0, 6, 99];
        assert_eq!(super::compute_instructions(&input), expected);

        let input = vec![2, 4, 4, 5, 99, 0];
        let expected = vec![2, 4, 4, 5, 99, 9801];
        assert_eq!(super::compute_instructions(&input), expected);

        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(super::compute_instructions(&input), expected);
    }

    #[test]
    fn test_computer_real() {
        let mut input = read_day_2_input();
        input[1] = 12;
        input[2] = 2;
        let result = super::compute_instructions(&input);
        assert_eq!(result[0], 4138687);
    }
    #[test]
    fn test_exhaustive_search() {
        let input = read_day_2_input();
        let result = super::exhaustive_search(&input, 19690720);
        assert_eq!(result, Some(6635));
    }

    fn read_day_2_input() -> Vec<u32> {
        let input = crate::utils::read_entire_file("data/day02.txt").unwrap();
        input
            .trim()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect()
    }
}
