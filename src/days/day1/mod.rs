#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part_one(input: &[i32]) -> i32 {
    input.iter().map(|mass| (mass / 3) - 2).sum()
}

#[aoc(day1, part2)]
pub fn solution(input: &[i32]) -> i32 {
    input.iter().map(|mass| get_fuel(mass.to_owned())).sum()
}

fn get_fuel(mut mass: i32) -> i32 {
    let mut sum: i32 = 0;
    while mass >= 0 {
        mass = (mass / 3) - 2;
        if mass > 0 {
            sum += mass;
        }
    }
    sum
}
