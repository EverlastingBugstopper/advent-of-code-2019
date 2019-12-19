#[derive(Debug)]
pub struct Parameter {
    mode: Mode,
    pub value: i32,
}

impl Parameter {
    pub fn new(mode: i32, value: i32) -> Parameter {
        Parameter {
            mode: Mode::from(mode),
            value,
        }
    }

    pub fn value(&self, program_state: &[i32]) -> i32 {
        match self.mode {
            Mode::Position => program_state[self.value as usize],
            Mode::Immediate => self.value,
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    Position,
    Immediate,
}

impl From<i32> for Mode {
    fn from(mode: i32) -> Mode {
        match mode {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => unreachable!("Unexpected parameter mode {}", mode),
        }
    }
}
