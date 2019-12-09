mod input;
use input::INPUT;

pub fn solution() -> Result<(), failure::Error> {
    let mut program: Vec<usize> = INPUT
        .split(",")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    program[1] = 12;
    program[2] = 2;
    let mut position = 0;
    while program[position] != 99 {
        let opcode = program[position];
        let position_one = program[position + 1];
        let position_two = program[position + 2];
        let result_position = program[position + 3];
        match opcode {
            1 => {
                program[result_position] = program[position_one] + program[position_two];
            }
            2 => {
                program[result_position] = program[position_one] * program[position_two];
            }
            _ => failure::bail!("unexpected opcode {}", opcode),
        }
        position += 4;
    }
    println!("{}", program[0]);
    Ok(())
}
