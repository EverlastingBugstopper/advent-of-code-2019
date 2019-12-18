#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn part_one(program: &[usize]) -> usize {
    run(&mut program.to_vec(), Some(12), Some(2))
}

#[aoc(day2, part2)]
pub fn part_two(program: &[usize]) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            let result = run(&mut program.to_vec(), Some(noun), Some(verb));
            if result == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!("Did not find noun/verb that satisfies the requirements")
}

pub fn run(program: &mut Vec<usize>, noun: Option<usize>, verb: Option<usize>) -> usize {
    if let Some(noun) = noun {
        program[1] = noun;
    }
    if let Some(verb) = verb {
        program[2] = verb;
    }
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

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn sample_one() {
        let mut program: Vec<usize> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let result = run(&mut program, None, None);
        assert_eq!(result, 3500)
    }
}
