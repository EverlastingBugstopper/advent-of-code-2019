mod instruction;
mod operation;
mod parameter;

use instruction::Instruction;
use operation::Operation;

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
        match instruction.operation {
            Operation::Add => self.add(&instruction),
            Operation::Multiply => self.multiply(&instruction),
            Operation::Input => self.input(&instruction),
            Operation::Output => self.output(&instruction),
            Operation::JumpIfTrue => self.jump_if_true(&instruction),
            Operation::JumpIfFalse => self.jump_if_false(&instruction),
            Operation::LessThan => self.less_than(&instruction),
            Operation::Equals => self.equals(&instruction),
            Operation::Abort => self.abort(),
        }
    }

    fn add(&mut self, instruction: &Instruction) -> bool {
        let output_position = instruction.parameters[2].value as usize;
        self.state[output_position] = instruction.parameters[0].value(&self.state)
            + instruction.parameters[1].value(&self.state);
        self.instruction_pointer += instruction.operation.num_params() + 1;
        false
    }

    fn multiply(&mut self, instruction: &Instruction) -> bool {
        let output_position = instruction.parameters[2].value as usize;
        self.state[output_position] = instruction.parameters[0].value(&self.state)
            * instruction.parameters[1].value(&self.state);
        self.instruction_pointer += instruction.operation.num_params() + 1;
        false
    }

    fn input(&mut self, instruction: &Instruction) -> bool {
        let output_position = instruction.parameters[0].value as usize;
        self.state[output_position] = self.input;
        self.instruction_pointer += instruction.operation.num_params() + 1;
        false
    }

    fn output(&mut self, instruction: &Instruction) -> bool {
        let value = instruction.parameters[0].value(&self.state);
        self.output.push(value);
        self.instruction_pointer += instruction.operation.num_params() + 1;
        false
    }

    fn jump_if_false(&mut self, instruction: &Instruction) -> bool {
        let val = instruction.parameters[0].value(&self.state);
        match val {
            0 => self.instruction_pointer = instruction.parameters[1].value(&self.state) as usize,
            _ => self.instruction_pointer += instruction.operation.num_params() + 1,
        }
        false
    }

    fn jump_if_true(&mut self, instruction: &Instruction) -> bool {
        match instruction.parameters[0].value(&self.state) {
            0 => self.instruction_pointer += instruction.operation.num_params() + 1,
            _ => self.instruction_pointer = instruction.parameters[1].value(&self.state) as usize,
        }
        false
    }

    fn less_than(&mut self, instruction: &Instruction) -> bool {
        let output_position = instruction.parameters[2].value as usize;
        if instruction.parameters[0].value(&self.state)
            < instruction.parameters[1].value(&self.state)
        {
            self.state[output_position] = 1;
        } else {
            self.state[output_position] = 0;
        }
        self.instruction_pointer += instruction.operation.num_params() + 1;
        false
    }

    fn equals(&mut self, instruction: &Instruction) -> bool {
        let output_position = instruction.parameters[2].value as usize;
        if instruction.parameters[0].value(&self.state)
            == instruction.parameters[1].value(&self.state)
        {
            self.state[output_position] = 1;
        } else {
            self.state[output_position] = 0;
        }
        self.instruction_pointer += instruction.operation.num_params() + 1;
        false
    }

    fn abort(&mut self) -> bool {
        true
    }
}
