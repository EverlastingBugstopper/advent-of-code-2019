use std::fmt;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
            Direction::Left => "L",
            Direction::Right => "R",
            Direction::Up => "U",
            Direction::Down => "D",
        };
        write!(f, "{}", x)
    }
}
