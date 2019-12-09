mod input;
use input::INPUT;

pub fn solution() -> Result<(), failure::Error> {
    for noun in 0..99 {
        for verb in 0..99 {
            let result = run(noun, verb)?;
            if result == 19690720 {
                println!("{}", 100 * noun + verb);
            }
        }
    }
    Ok(())
}

fn run(noun: usize, verb: usize) -> Result<usize, failure::Error> {
    let mut program: Vec<usize> = INPUT
        .split(",")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    program[1] = noun;
    program[2] = verb;
    let mut instruction_pointer = 0;
    while program[instruction_pointer] != 99 {
        let instruction = &program.clone()[instruction_pointer..instruction_pointer + 4];
        process(&mut program, instruction)?;
        instruction_pointer += 4;
    }
    Ok(program[0])
}

fn process(program: &mut Vec<usize>, instruction: &[usize]) -> Result<(), failure::Error> {
    let opcode = instruction[0];
    let parameters = &instruction[1..];
    match opcode {
        1 => {
            program[parameters[2]] = program[parameters[0]] + program[parameters[1]];
        }
        2 => {
            program[parameters[2]] = program[parameters[0]] * program[parameters[1]];
        }
        _ => failure::bail!("unexpected opcode {}", opcode),
    };
    Ok(())
}
