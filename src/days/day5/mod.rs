mod program;

use program::Program;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn part_one(initial_state: &[i32]) -> i32 {
    let program = run(initial_state, 1);
    program.get_diagnostic_code()
}

pub fn run(state: &[i32], input: i32) -> Program {
    let mut program = Program::new(state, input);
    program.run();
    program
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn can_process_new_opcode_format() {
        let program: Vec<i32> = vec![1002, 4, 3, 4, 33];
        let result = run(&program, None);
        assert_eq!(result.state[4], 99)
    }

    #[test]
    fn sample_one() {
        let program: Vec<i32> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let result = run(&program, None);
        assert_eq!(result.state[0], 3500)
    }

    #[test]
    fn sample_two() {
        let program: Vec<i32> = vec![1, 0, 0, 0, 99];
        let result = run(&program, None);
        assert_eq!(result.state[0], 2)
    }

    #[test]
    fn sample_three() {
        let program: Vec<i32> = vec![2, 3, 0, 3, 99];
        let result = run(&program, None);
        assert_eq!(result.state[0], 2)
    }

    #[test]
    fn sample_four() {
        let program: Vec<i32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result = run(&program, None);
        assert_eq!(result.state[0], 30)
    }
}
