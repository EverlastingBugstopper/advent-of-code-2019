use super::movement::Movement;

pub struct Path {
    pub movements: Vec<Movement>,
}

impl Path {
    pub fn new(input: &str) -> Result<Path, failure::Error> {
        let input: Vec<&str> = input.split(",").collect();
        let mut movements: Vec<Movement> = Vec::new();
        for movement in input {
            movements.push(Movement::new(movement)?)
        }
        Ok(Path { movements })
    }
}
