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

#[aoc(day5, part2)]
pub fn part_two(initial_state: &[i32]) -> i32 {
    let program = run(initial_state, 5);
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
        let result = run(&program, 0);
        assert_eq!(result.state[4], 99)
    }

    #[test]
    fn sample_one() {
        let program: Vec<i32> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let result = run(&program, 0);
        assert_eq!(result.state[0], 3500)
    }

    #[test]
    fn sample_two() {
        let program: Vec<i32> = vec![1, 0, 0, 0, 99];
        let result = run(&program, 0);
        assert_eq!(result.state[0], 2)
    }

    #[test]
    fn sample_three() {
        let program: Vec<i32> = vec![2, 3, 0, 3, 99];
        let result = run(&program, 0);
        assert_eq!(result.state[0], 2)
    }

    #[test]
    fn sample_four() {
        let program: Vec<i32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let result = run(&program, 0);
        assert_eq!(result.state[0], 30)
    }

    #[test]
    fn sample_five() {
        let program: Vec<i32> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let eight = run(&program, 8);
        assert_eq!(eight.get_diagnostic_code(), 1);
        let not_eight = run(&program, 7);
        assert_eq!(not_eight.get_diagnostic_code(), 0);
    }

    #[test]
    fn sample_six() {
        let program: Vec<i32> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let eight = run(&program, 8);
        assert_eq!(eight.get_diagnostic_code(), 0);
        let less_than_eight = run(&program, 7);
        assert_eq!(less_than_eight.get_diagnostic_code(), 1);
    }

    #[test]
    fn sample_seven() {
        let program: Vec<i32> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let eight = run(&program, 8);
        assert_eq!(eight.get_diagnostic_code(), 1);
        let not_eight = run(&program, 7);
        assert_eq!(not_eight.get_diagnostic_code(), 0);
    }

    #[test]
    fn sample_eight() {
        let program: Vec<i32> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let eight = run(&program, 8);
        assert_eq!(eight.get_diagnostic_code(), 0);
        let less_than_eight = run(&program, 7);
        assert_eq!(less_than_eight.get_diagnostic_code(), 1);
    }

    #[test]
    fn sample_nine() {
        let program: Vec<i32> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let zero = run(&program, 0);
        assert_eq!(zero.get_diagnostic_code(), 0);
        let not_zero = run(&program, 1);
        assert_eq!(not_zero.get_diagnostic_code(), 1);
    }

    #[test]
    fn sample_ten() {
        let program: Vec<i32> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let zero = run(&program, 0);
        assert_eq!(zero.get_diagnostic_code(), 0);
        let not_zero = run(&program, 1);
        assert_eq!(not_zero.get_diagnostic_code(), 1);
    }

    #[test]
    fn beeeg_sample() {
        let program: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let below_eight = run(&program, 7);
        assert_eq!(below_eight.get_diagnostic_code(), 999);
        let eight = run(&program, 8);
        assert_eq!(eight.get_diagnostic_code(), 1000);
        let above_eight = run(&program, 9);
        assert_eq!(above_eight.get_diagnostic_code(), 1001);
    }
}
