mod one;

pub fn run(day: &str) {
    match day {
        "1" => one::solution(),
        _ => eprintln!("solution for day {} does not exist", day),
    }
}
