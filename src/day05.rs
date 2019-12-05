use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
#[derive(PartialEq, Copy, Clone)]
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

#[derive(FromPrimitive, PartialEq, Copy, Clone)]
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
            Some(instruction) => {
                match instruction {
                    Instruction::Addition(param_1, param_2, _) => {
                        self.op_1 = self.compute_operand(param_1);
                        self.op_2 = self.compute_operand(param_2);
                        self.compute_destination()
                    }
                    Instruction::Multiplication(param_1, param_2, _) => {
                        self.op_1 = self.compute_operand(param_1);
                        self.op_2 = self.compute_operand(param_2);
                        self.compute_destination()
                    }
                    Instruction::Input(_) => self.compute_destination(),
                    Instruction::Output(_) => self.compute_destination(),
                    Instruction::JumpIfTrue(param_1, param_2) => {
                        self.op_1 = self.compute_operand(param_1);
                        self.op_2 = self.compute_operand(param_2);
                    },
                    Instruction::JumpIfFalse(param_1, param_2) => {
                        self.op_1 = self.compute_operand(param_1);
                        self.op_2 = self.compute_operand(param_2);
                    },
                    Instruction::LessThan(param_1, param_2, _) => {
                        self.op_1 = self.compute_operand(param_1);
                        self.op_2 = self.compute_operand(param_2);
                        self.compute_destination()
                    },
                    Instruction::Equals(param_1, param_2, _) => {
                        self.op_1 = self.compute_operand(param_1);
                        self.op_2 = self.compute_operand(param_2);
                        self.compute_destination()
                    }
                    Instruction::Stop => {},
                }
            }
            None => {}
        };
        instruction
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
                },
                Instruction::Multiplication(_, _, _) => {
                    self.input[self.destination] = self.op_1 * self.op_2;
                },
                Instruction::Input(_) => {
                    self.input[self.destination] = input;
                },
                Instruction::Output(_) => {
                    self.output.push(self.input[self.destination]);
                },
                Instruction::JumpIfTrue(_, _) => {
                    if self.op_1 != 0 {
                        self.counter = self.op_2 as usize;
                    }
                },
                Instruction::JumpIfFalse(_, _) => {
                    if self.op_1 == 0 {
                        self.counter = self.op_2 as usize;
                    }
                },
                Instruction::LessThan(_, _, _) => {
                    if self.op_1 < self.op_2 {
                        self.input[self.destination] = 1;
                    } else {
                        self.input[self.destination] = 0;
                    }
                },
                Instruction::Equals(_, _, _) => {
                    if self.op_1 == self.op_2 {
                        self.input[self.destination] = 1;
                    } else {
                        self.input[self.destination] = 0;
                    }
                },
                Instruction::Stop => return,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_execute() {
        let input = DAY_5_INPUT.lock().unwrap();
        let mut computer = super::Computer::new(&input);
        computer.compute(1);
        assert_eq!(computer.output[computer.output.len() - 1], 16489636);
        
        let mut computer = super::Computer::new(&input);
        computer.compute(5);
        assert_eq!(computer.output[computer.output.len() - 1], 9386583);
    }

    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    #[allow(dead_code)]
    static DAY_5_INPUT: Lazy<Mutex<Vec<i32>>> = Lazy::new(|| {
        Mutex::new(vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 91, 92, 225, 1102, 85, 13, 225,
            1, 47, 17, 224, 101, -176, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 7, 224, 1,
            223, 224, 223, 1102, 79, 43, 225, 1102, 91, 79, 225, 1101, 94, 61, 225, 1002, 99, 42,
            224, 1001, 224, -1890, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 6, 224, 1, 224, 223,
            223, 102, 77, 52, 224, 1001, 224, -4697, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 7,
            224, 1, 224, 223, 223, 1101, 45, 47, 225, 1001, 43, 93, 224, 1001, 224, -172, 224, 4,
            224, 102, 8, 223, 223, 1001, 224, 1, 224, 1, 224, 223, 223, 1102, 53, 88, 225, 1101,
            64, 75, 225, 2, 14, 129, 224, 101, -5888, 224, 224, 4, 224, 102, 8, 223, 223, 101, 6,
            224, 224, 1, 223, 224, 223, 101, 60, 126, 224, 101, -148, 224, 224, 4, 224, 1002, 223,
            8, 223, 1001, 224, 2, 224, 1, 224, 223, 223, 1102, 82, 56, 224, 1001, 224, -4592, 224,
            4, 224, 1002, 223, 8, 223, 101, 4, 224, 224, 1, 224, 223, 223, 1101, 22, 82, 224, 1001,
            224, -104, 224, 4, 224, 1002, 223, 8, 223, 101, 4, 224, 224, 1, 223, 224, 223, 4, 223,
            99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247,
            1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106,
            0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0,
            300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 8,
            226, 677, 224, 102, 2, 223, 223, 1005, 224, 329, 1001, 223, 1, 223, 1007, 226, 226,
            224, 1002, 223, 2, 223, 1006, 224, 344, 101, 1, 223, 223, 108, 226, 226, 224, 1002,
            223, 2, 223, 1006, 224, 359, 1001, 223, 1, 223, 107, 226, 677, 224, 102, 2, 223, 223,
            1006, 224, 374, 101, 1, 223, 223, 8, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 389,
            1001, 223, 1, 223, 1008, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 404, 101, 1, 223,
            223, 7, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 419, 101, 1, 223, 223, 1108, 226,
            677, 224, 1002, 223, 2, 223, 1005, 224, 434, 101, 1, 223, 223, 1108, 226, 226, 224,
            102, 2, 223, 223, 1005, 224, 449, 1001, 223, 1, 223, 107, 226, 226, 224, 102, 2, 223,
            223, 1005, 224, 464, 101, 1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1006,
            224, 479, 101, 1, 223, 223, 1007, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 494,
            1001, 223, 1, 223, 1008, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 509, 1001, 223,
            1, 223, 1108, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 524, 1001, 223, 1, 223, 108,
            677, 677, 224, 1002, 223, 2, 223, 1005, 224, 539, 101, 1, 223, 223, 108, 226, 677, 224,
            1002, 223, 2, 223, 1005, 224, 554, 101, 1, 223, 223, 1008, 677, 677, 224, 1002, 223, 2,
            223, 1006, 224, 569, 1001, 223, 1, 223, 1107, 677, 677, 224, 102, 2, 223, 223, 1005,
            224, 584, 1001, 223, 1, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 599, 1001,
            223, 1, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 614, 1001, 223, 1, 223, 7,
            226, 677, 224, 1002, 223, 2, 223, 1006, 224, 629, 101, 1, 223, 223, 1107, 677, 226,
            224, 1002, 223, 2, 223, 1005, 224, 644, 1001, 223, 1, 223, 1107, 226, 677, 224, 102, 2,
            223, 223, 1006, 224, 659, 1001, 223, 1, 223, 107, 677, 677, 224, 1002, 223, 2, 223,
            1005, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226,
        ])
    });
}
