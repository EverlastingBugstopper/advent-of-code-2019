use std::fmt;

mod cell;
use cell::Cell;

pub struct Map {
    grid: Vec<Vec<Cell>>,
    pub center: usize,
}

impl Map {
    pub fn new(size: usize) -> Map {
        let center = size / 2;
        let mut grid: Vec<Vec<Cell>> = Vec::new();
        for row in 0..size {
            grid.push(Vec::new());
            for column in 0..size {
                let mut cell = Cell::default();
                if row == center && column == center {
                    cell.is_center = true;
                }
                grid[row].push(cell);
            }
        }
        Map { grid, center }
    }

    pub fn cross(&mut self, row: usize, column: usize, wire: usize) -> bool {
        self.grid[column][row].cross(wire)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
