mod _1;
mod _2;
mod _3;

pub fn run(day: &str) -> Result<(), failure::Error> {
    match day {
        "1" => _1::solution(),
        "2" => _2::solution(),
        "3" => _3::solution(),
        _ => failure::bail!("solution for day {} does not exist", day),
    }
}
