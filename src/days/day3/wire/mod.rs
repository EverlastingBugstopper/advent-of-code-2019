pub mod direction;
mod movement;
mod path;
use path::Path;

pub struct Wire {
    pub path: Path,
}

impl Wire {
    pub fn new(input: &'static str) -> Result<Wire, failure::Error> {
        Ok(Wire {
            path: Path::new(input)?,
        })
    }
}
