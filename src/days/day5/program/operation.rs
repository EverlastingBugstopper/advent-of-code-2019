use std::fmt;

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Abort,
}

impl From<String> for Operation {
    fn from(opcode: String) -> Operation {
        match opcode.as_str() {
            "01" => Operation::Add,
            "02" => Operation::Multiply,
            "03" => Operation::Input,
            "04" => Operation::Output,
            "05" => Operation::JumpIfTrue,
            "06" => Operation::JumpIfFalse,
            "07" => Operation::LessThan,
            "08" => Operation::Equals,
            "99" => Operation::Abort,
            _ => unreachable!("Unexpected opcode {}", opcode),
        }
    }
}

impl Operation {
    pub fn num_params(&self) -> usize {
        match self {
            Operation::Add => 3,
            Operation::Multiply => 3,
            Operation::Input => 1,
            Operation::Output => 1,
            Operation::Abort => 0,
            Operation::JumpIfTrue => 2,
            Operation::JumpIfFalse => 2,
            Operation::LessThan => 3,
            Operation::Equals => 3,
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
            Operation::Add => "1",
            Operation::Multiply => "2",
            Operation::Input => "3",
            Operation::Output => "4",
            Operation::JumpIfTrue => "5",
            Operation::JumpIfFalse => "6",
            Operation::LessThan => "7",
            Operation::Equals => "8",
            Operation::Abort => "99",
        };
        write!(f, "{} ({:?})", x, self)
    }
}
