use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
#[derive(FromPrimitive, PartialEq)]
enum Instruction {
    Addition = 1,
    Multiplication = 2,
    Stop = 99,
}

#[allow(dead_code)]
fn compute(instruction: Instruction, op_1: u32, op_2: u32) -> u32 {
    match instruction {
        Instruction::Addition => op_1 + op_2,
        Instruction::Multiplication => op_1 * op_2,
        _ => panic!("unimplemented instruction"),
    }
}

#[allow(dead_code)]
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
fn exhaustive_search(input: &[u32], desired_result: u32) -> u32 {
    let mut mut_input = input.to_vec();
    for noun in 0..100 {
        for verb in 0..100 {
            mut_input[1] = noun;
            mut_input[2] = verb;
            let computed_result = compute_instructions(&mut_input)[0];
            if computed_result == desired_result {
                return 100 * noun + verb;
            }
        }
    }
    0
}

mod tests {
    #[test]
    fn test_computer() {
        let input_1 = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(
            super::compute_instructions(&input_1),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        let input_2 = vec![1, 0, 0, 0, 99];
        assert_eq!(super::compute_instructions(&input_2), vec![2, 0, 0, 0, 99]);
        let input_3 = vec![2, 3, 0, 3, 99];
        assert_eq!(super::compute_instructions(&input_3), vec![2, 3, 0, 6, 99]);
        let input_4 = vec![2, 4, 4, 5, 99, 0];
        assert_eq!(
            super::compute_instructions(&input_4),
            vec![2, 4, 4, 5, 99, 9801]
        );
        let input_5 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(
            super::compute_instructions(&input_5),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn test_computer_real() {
        let mut input = DAY_2_INPUT.lock().unwrap();
        input[1] = 12;
        input[2] = 2;
        let result = super::compute_instructions(&input);
        assert_eq!(result[0], 4138687);
    }
    #[test]
    fn test_exhaustive_search() {
        let input = DAY_2_INPUT.lock().unwrap();
        let result = super::exhaustive_search(&input, 19690720);
        assert_eq!(result, 6635);
    }

    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    #[allow(dead_code)]
    static DAY_2_INPUT: Lazy<Mutex<Vec<u32>>> = Lazy::new(|| {
        Mutex::new(vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 6, 1, 19, 1, 19, 10, 23, 2, 13, 23,
            27, 1, 5, 27, 31, 2, 6, 31, 35, 1, 6, 35, 39, 2, 39, 9, 43, 1, 5, 43, 47, 1, 13, 47,
            51, 1, 10, 51, 55, 2, 55, 10, 59, 2, 10, 59, 63, 1, 9, 63, 67, 2, 67, 13, 71, 1, 71, 6,
            75, 2, 6, 75, 79, 1, 5, 79, 83, 2, 83, 9, 87, 1, 6, 87, 91, 2, 91, 6, 95, 1, 95, 6, 99,
            2, 99, 13, 103, 1, 6, 103, 107, 1, 2, 107, 111, 1, 111, 9, 0, 99, 2, 14, 0, 0,
        ])
    });
}
