use std::fmt;

#[derive(Debug, Default)]
pub struct Cell {
    pub wire_one: bool,
    pub wire_two: bool,
    pub is_center: bool,
}

impl Cell {
    pub fn cross(&mut self, wire: usize) -> bool {
        match wire {
            0 => self.wire_one = true,
            1 => self.wire_two = true,
            _ => println!("wrong wire num"),
        }
        self.wire_one && self.wire_two
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp = if self.is_center {
            "O"
        } else if self.wire_one && !self.wire_two {
            "1"
        } else if !self.wire_one && self.wire_two {
            "2"
        } else if self.wire_one && self.wire_two {
            "X"
        } else {
            "."
        };
        write!(f, "{}", disp)
    }
}
