use std::fmt;

use super::direction::Direction;

pub struct Movement {
    pub direction: Direction,
    pub distance: usize,
}

impl Movement {
    pub fn new(input: &str) -> Movement {
        let input = input.to_string();
        let direction_input = &input[0..1];
        let direction = match direction_input {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!("invalid direction {}", direction_input),
        };
        let distance = input[1..].parse::<usize>().unwrap();
        Movement {
            direction,
            distance,
        }
    }
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.direction, self.distance)
    }
}
