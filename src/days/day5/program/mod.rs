mod instruction;
mod operation;
mod parameter;

use instruction::Instruction;
use operation::Operation;
use parameter::Parameter;

pub struct Program {
    instruction_pointer: usize,
    pub output: Vec<i32>,
    pub state: Vec<i32>,
    input: i32,
}

impl Program {
    pub fn new(state: &[i32], input: i32) -> Program {
        Program {
            instruction_pointer: 0,
            output: Vec::new(),
            state: state.to_vec(),
            input,
        }
    }

    pub fn output_as_str(&self) -> String {
        let mut result = String::new();
        for (idx, output) in self.output.iter().enumerate() {
            if idx < self.output.len() - 1 {
                result = format!("{}, ", output);
            } else {
                result = format!("{}", output);
            }
        }
        result
    }

    pub fn get_diagnostic_code(&self) -> i32 {
        let last_index = self.output.len() - 1;
        for (idx, output) in self.output.iter().enumerate() {
            if output.to_owned() != 0 && idx != last_index {
                panic!("diagnostic program outputted non-zero error code");
            }
        }
        self.output[last_index]
    }

    pub fn run(&mut self) -> () {
        let mut abort = false;
        while !abort {
            abort = self.cycle();
        }
    }

    fn cycle(&mut self) -> bool {
        let instruction = Instruction::new(self.instruction_pointer, &mut self.state);
        self.instruction_pointer += instruction.operation.num_params() + 1;
        match instruction.operation {
            Operation::Add => self.add(&instruction.parameters),
            Operation::Multiply => self.multiply(&instruction.parameters),
            Operation::Input => self.input(&instruction.parameters),
            Operation::Output => self.output(&instruction.parameters),
            Operation::Abort => self.abort(&instruction.parameters),
        }
    }

    fn add(&mut self, parameters: &[Parameter]) -> bool {
        let output_position = parameters[2].value as usize;
        self.state[output_position] =
            parameters[0].value(&self.state) + parameters[1].value(&self.state);
        false
    }

    fn multiply(&mut self, parameters: &[Parameter]) -> bool {
        let output_position = parameters[2].value as usize;
        self.state[output_position] =
            parameters[0].value(&self.state) * parameters[1].value(&self.state);
        false
    }

    fn input(&mut self, parameters: &[Parameter]) -> bool {
        let output_position = parameters[0].value as usize;
        self.state[output_position] = self.input;
        false
    }

    fn output(&mut self, parameters: &[Parameter]) -> bool {
        let value = parameters[0].value(&self.state);
        self.output.push(value);
        false
    }

    fn abort(&mut self, _: &[Parameter]) -> bool {
        true
    }
}
