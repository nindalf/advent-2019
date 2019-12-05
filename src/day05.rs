use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
#[derive(Copy, Clone)]
enum Instruction {
    Addition(Parameter, Parameter, Parameter),
    Multiplication(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Stop,
}

#[derive(FromPrimitive, Copy, Clone)]
enum Parameter {
    Position = 0,
    Immediate = 1,
}

impl Instruction {
    fn new(input: i32) -> Option<Instruction> {
        let param_1 = FromPrimitive::from_i32(input / 100 % 10)?;
        let param_2 = FromPrimitive::from_i32(input / 1000 % 10)?;
        let param_3 = FromPrimitive::from_i32(input / 10000 % 10)?;
        match input % 100 {
            1 => Some(Instruction::Addition(param_1, param_2, param_3)),
            2 => Some(Instruction::Multiplication(param_1, param_2, param_3)),
            3 => Some(Instruction::Input(param_1)),
            4 => Some(Instruction::Output(param_1)),
            5 => Some(Instruction::JumpIfTrue(param_1, param_2)),
            6 => Some(Instruction::JumpIfFalse(param_1, param_2)),
            7 => Some(Instruction::LessThan(param_1, param_2, param_3)),
            8 => Some(Instruction::Equals(param_1, param_2, param_3)),
            99 => Some(Instruction::Stop),
            _ => None,
        }
    }
}

struct Computer {
    input: Vec<i32>,
    output: Vec<i32>,
    counter: usize,
    op_1: i32,
    op_2: i32,
    destination: usize,
}

impl Computer {
    #[allow(dead_code)]
    fn new(input: &[i32]) -> Computer {
        Computer {
            input: input.to_vec(),
            output: Vec::new(),
            counter: 0,
            op_1: 0,
            op_2: 0,
            destination: 0,
        }
    }

    fn next_instruction(&mut self) -> Option<Instruction> {
        let instruction = Instruction::new(self.input[self.counter]);
        self.counter += 1;
        match instruction {
            Some(instruction) => match instruction {
                Instruction::Addition(param_1, param_2, _) => {
                    self.compute_two_operands_and_destination(param_1, param_2)
                }
                Instruction::Multiplication(param_1, param_2, _) => {
                    self.compute_two_operands_and_destination(param_1, param_2)
                }
                Instruction::Input(_) => self.compute_destination(),
                Instruction::Output(_) => self.compute_destination(),
                Instruction::JumpIfTrue(param_1, param_2) => {
                    self.compute_two_operands(param_1, param_2)
                }
                Instruction::JumpIfFalse(param_1, param_2) => {
                    self.compute_two_operands(param_1, param_2)
                }
                Instruction::LessThan(param_1, param_2, _) => {
                    self.compute_two_operands_and_destination(param_1, param_2)
                }
                Instruction::Equals(param_1, param_2, _) => {
                    self.compute_two_operands_and_destination(param_1, param_2)
                }
                Instruction::Stop => {}
            },
            None => {}
        };
        instruction
    }

    fn compute_two_operands_and_destination(&mut self, param_1: Parameter, param_2: Parameter) {
        self.op_1 = self.compute_operand(param_1);
        self.op_2 = self.compute_operand(param_2);
        self.compute_destination()
    }

    fn compute_two_operands(&mut self, param_1: Parameter, param_2: Parameter) {
        self.op_1 = self.compute_operand(param_1);
        self.op_2 = self.compute_operand(param_2);
    }

    fn compute_operand(&mut self, parameter: Parameter) -> i32 {
        let op = match parameter {
            Parameter::Position => self.input[self.input[self.counter] as usize],
            Parameter::Immediate => self.input[self.counter],
        };
        self.counter += 1;
        op
    }

    fn compute_destination(&mut self) {
        self.destination = self.input[self.counter] as usize;
        self.counter += 1;
    }

    #[allow(dead_code)]
    fn compute(&mut self, input: i32) {
        while let Some(instruction) = self.next_instruction() {
            match instruction {
                Instruction::Addition(_, _, _) => {
                    self.input[self.destination] = self.op_1 + self.op_2;
                }
                Instruction::Multiplication(_, _, _) => {
                    self.input[self.destination] = self.op_1 * self.op_2;
                }
                Instruction::Input(_) => {
                    self.input[self.destination] = input;
                }
                Instruction::Output(_) => {
                    self.output.push(self.input[self.destination]);
                }
                Instruction::JumpIfTrue(_, _) => {
                    if self.op_1 != 0 {
                        self.counter = self.op_2 as usize;
                    }
                }
                Instruction::JumpIfFalse(_, _) => {
                    if self.op_1 == 0 {
                        self.counter = self.op_2 as usize;
                    }
                }
                Instruction::LessThan(_, _, _) => {
                    if self.op_1 < self.op_2 {
                        self.input[self.destination] = 1;
                    } else {
                        self.input[self.destination] = 0;
                    }
                }
                Instruction::Equals(_, _, _) => {
                    if self.op_1 == self.op_2 {
                        self.input[self.destination] = 1;
                    } else {
                        self.input[self.destination] = 0;
                    }
                }
                Instruction::Stop => return,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_execute() {
        let input = crate::utils::read_entire_file("data/day05.txt").unwrap();
        let input: Vec<i32> = input
            .trim()
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
            
        let mut computer = super::Computer::new(&input);
        computer.compute(1);
        assert_eq!(computer.output[computer.output.len() - 1], 16489636);

        let mut computer = super::Computer::new(&input);
        computer.compute(5);
        assert_eq!(computer.output[computer.output.len() - 1], 9386583);
    }
}
