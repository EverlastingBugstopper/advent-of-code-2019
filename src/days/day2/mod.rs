#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn part_one(program: &[usize]) -> usize {
    run(&mut program.to_vec(), 12, 2)
}

#[aoc(day2, part2)]
pub fn part_two(program: &[usize]) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            let result = run(&mut program.to_vec(), noun, verb);
            if result == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!("Did not find noun/verb that satisfies the requirements")
}

fn run(program: &mut Vec<usize>, noun: usize, verb: usize) -> usize {
    program[1] = noun;
    program[2] = verb;
    let mut instruction_pointer = 0;
    while program[instruction_pointer] != 99 {
        let instruction = &program.clone()[instruction_pointer..instruction_pointer + 4];
        process(program, instruction);
        instruction_pointer += 4;
    }
    program[0]
}

fn process(program: &mut Vec<usize>, instruction: &[usize]) -> () {
    let opcode = instruction[0];
    let parameters = &instruction[1..];
    match opcode {
        1 => program[parameters[2]] = program[parameters[0]] + program[parameters[1]],
        2 => program[parameters[2]] = program[parameters[0]] * program[parameters[1]],
        _ => unreachable!("unexpected opcode {}", opcode),
    }
}
