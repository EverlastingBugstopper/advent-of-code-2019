mod input;
use input::INPUT;

pub fn solution() -> Result<(), failure::Error> {
    let input: Vec<&str> = INPUT.split("\n").collect();
    let mut sum: i32 = 0;
    for mass_str in input {
        let mass: i32 = mass_str.trim().parse()?;
        sum += get_fuel(mass);
    }
    println!("{}", sum);
    Ok(())
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
