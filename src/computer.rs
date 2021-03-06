use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;

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
    AdjustRelativeBase(Parameter),
    Stop,
}

impl Instruction {
    fn new(input: i64) -> Option<Instruction> {
        let param_1 = FromPrimitive::from_i64(input / 100 % 10)?;
        let param_2 = FromPrimitive::from_i64(input / 1000 % 10)?;
        let param_3 = FromPrimitive::from_i64(input / 10000 % 10)?;
        match input % 100 {
            1 => Some(Instruction::Addition(param_1, param_2, param_3)),
            2 => Some(Instruction::Multiplication(param_1, param_2, param_3)),
            3 => Some(Instruction::Input(param_1)),
            4 => Some(Instruction::Output(param_1)),
            5 => Some(Instruction::JumpIfTrue(param_1, param_2)),
            6 => Some(Instruction::JumpIfFalse(param_1, param_2)),
            7 => Some(Instruction::LessThan(param_1, param_2, param_3)),
            8 => Some(Instruction::Equals(param_1, param_2, param_3)),
            9 => Some(Instruction::AdjustRelativeBase(param_1)),
            99 => Some(Instruction::Stop),
            _ => None,
        }
    }
}

#[derive(FromPrimitive, Copy, Clone, Debug)]
enum Parameter {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

pub struct Computer {
    instructions: Vec<i64>,
    counter: i64,
    relative_base: i64,
    extended_memory: HashMap<i64, i64>,
}

impl Computer {
    pub fn new(instructions: &[i64]) -> Computer {
        Computer {
            instructions: instructions.to_vec(),
            counter: 0,
            relative_base: 0,
            extended_memory: HashMap::new(),
        }
    }

    fn next_instruction(&mut self) -> Option<Instruction> {
        let instruction = Instruction::new(self.read_memory(self.counter));
        self.counter += 1;
        instruction
    }

    fn compute_three_operands(
        &mut self,
        param_1: Parameter,
        param_2: Parameter,
        param_3: Parameter,
    ) -> (i64, i64, i64) {
        let op_1 = self.compute_operand(param_1);
        let op_2 = self.compute_operand(param_2);
        let op_3 = self.compute_operand(param_3);
        (op_1, op_2, op_3)
    }

    fn compute_two_operands(&mut self, param_1: Parameter, param_2: Parameter) -> (i64, i64) {
        let op_1 = self.compute_operand(param_1);
        let op_2 = self.compute_operand(param_2);
        (op_1, op_2)
    }

    fn compute_operand(&mut self, parameter: Parameter) -> i64 {
        let op = match parameter {
            Parameter::Position => self.read_memory(self.counter),
            Parameter::Immediate => self.counter,
            Parameter::Relative => self.relative_base + self.read_memory(self.counter),
        };
        self.counter += 1;
        op
    }

    fn read_memory(&self, location: i64) -> i64 {
        if location < 0 {
            panic!("read from invalid memory - {}", location);
        }
        if location < self.instructions.len() as i64 {
            return self.instructions[location as usize];
        }
        *self.extended_memory.get(&location).unwrap_or(&0)
    }

    fn write_memory(&mut self, location: i64, val: i64) {
        if location < 0 {
            panic!("write to invalid memory - {}", location);
        }
        if location < self.instructions.len() as i64 {
            self.instructions[location as usize] = val;
            return;
        }
        self.extended_memory.insert(location, val);
    }

    pub fn compute(&mut self, input: &[i64]) -> Option<i64> {
        let mut input_counter = 0;
        while let Some(instruction) = self.next_instruction() {
            match instruction {
                Instruction::Addition(param_1, param_2, param_3) => {
                    let (op_1, op_2, op_3) = self.compute_three_operands(param_1, param_2, param_3);
                    self.write_memory(
                        op_3,
                        self.read_memory(op_1) + self.read_memory(op_2),
                    );
                }
                Instruction::Multiplication(param_1, param_2, param_3) => {
                    let (op_1, op_2, op_3) = self.compute_three_operands(param_1, param_2, param_3);
                    self.write_memory(
                        op_3,
                        self.read_memory(op_1) * self.read_memory(op_2),
                    );
                }
                Instruction::Input(param) => {
                    let op_1 = self.compute_operand(param);
                    self.write_memory(op_1, input[input_counter]);
                    input_counter += 1;
                }
                Instruction::Output(param) => {
                    let op_1 = self.compute_operand(param);
                    return Some(self.read_memory(op_1));
                }
                Instruction::JumpIfTrue(param_1, param_2) => {
                    let (op_1, op_2) = self.compute_two_operands(param_1, param_2);
                    if self.read_memory(op_1) != 0 {
                        self.counter = self.read_memory(op_2);
                    }
                }
                Instruction::JumpIfFalse(param_1, param_2) => {
                    let (op_1, op_2) = self.compute_two_operands(param_1, param_2);
                    if self.read_memory(op_1) == 0 {
                        self.counter = self.read_memory(op_2);
                    }
                }
                Instruction::LessThan(param_1, param_2, param_3) => {
                    let (op_1, op_2, op_3) = self.compute_three_operands(param_1, param_2, param_3);
                    if self.read_memory(op_1) < self.read_memory(op_2) {
                        self.write_memory(op_3, 1);
                    } else {
                        self.write_memory(op_3, 0);
                    }
                }
                Instruction::Equals(param_1, param_2, param_3) => {
                    let (op_1, op_2, op_3) = self.compute_three_operands(param_1, param_2, param_3);
                    if self.read_memory(op_1) == self.read_memory(op_2) {
                        self.write_memory(op_3, 1);
                    } else {
                        self.write_memory(op_3, 0);
                    }
                }
                Instruction::AdjustRelativeBase(param) => {
                    let op_1 = self.compute_operand(param);
                    self.relative_base += self.read_memory(op_1);
                }
                Instruction::Stop => {
                    return None;
                }
            };
        }
        None
    }
}
