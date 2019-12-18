pub mod direction;
mod movement;
mod path;
use path::Path;

pub struct Wire {
    pub path: Path,
}

impl Wire {
    pub fn new(input: &str) -> Wire {
        Wire {
            path: Path::new(input),
        }
    }
}
