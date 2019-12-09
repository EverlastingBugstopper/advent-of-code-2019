mod one;
mod three;
mod two;

pub fn run(day: &str) -> Result<(), failure::Error> {
    match day {
        "1" => one::solution(),
        "2" => two::solution(),
        "3" => three::solution(),
        _ => failure::bail!("solution for day {} does not exist", day),
    }
}
