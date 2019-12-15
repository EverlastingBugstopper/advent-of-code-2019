use std::collections::HashMap;

const RADIX: u32 = 10;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<u32> {
    let input: Vec<&str> = input.split("-").collect();
    let mut num_vec = Vec::new();
    for num in input[0].parse::<u32>().unwrap()..input[1].parse::<u32>().unwrap() {
        num_vec.push(num)
    }

    num_vec
}

#[aoc(day4, part1)]
pub fn part_one(numbers: &[u32]) -> u32 {
    let mut num_possibilities: u32 = 0;
    for num in numbers {
        if fits_criteria(&num.to_string(), false) {
            num_possibilities += 1;
        }
    }
    num_possibilities
}

#[aoc(day4, part2)]
pub fn part_two(numbers: &[u32]) -> u32 {
    let mut num_possibilities: u32 = 0;
    for num in numbers {
        let num = num.to_string();
        if fits_criteria(&num.to_string(), true) {
            num_possibilities += 1;
        }
    }
    num_possibilities
}

pub fn fits_criteria(num: &str, is_part_two: bool) -> bool {
    let mut prev_digit = None;
    let mut sibling_ranges: HashMap<u32, (usize, usize)> = HashMap::new();
    for (idx, digit) in num.char_indices() {
        if idx == 6 {
            return false;
        }
        let digit = digit.to_digit(RADIX).unwrap();
        if let Some(prev_digit) = prev_digit {
            if digit < prev_digit {
                return false;
            } else if digit == prev_digit {
                match sibling_ranges.get(&digit) {
                    Some((start, _)) => sibling_ranges.insert(digit, (start.to_owned(), idx)),
                    None => sibling_ranges.insert(digit, (idx - 1, idx)),
                };
            }
        }
        prev_digit = Some(digit);
    }
    if !is_part_two {
        !sibling_ranges.is_empty()
    } else {
        for (_, (start, end)) in sibling_ranges {
            if end - start == 1 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::fits_criteria;

    #[test]
    fn sample_1() {
        assert!(fits_criteria("111111", false))
    }

    #[test]
    fn sample_2() {
        assert_eq!(fits_criteria("223450", false), false)
    }

    #[test]
    fn sample_3() {
        assert_eq!(fits_criteria("123789", false), false)
    }

    #[test]
    fn sample_4() {
        assert!(fits_criteria("112233", true))
    }

    #[test]
    fn sample_5() {
        assert_eq!(fits_criteria("123444", true), false)
    }

    #[test]
    fn sample_6() {
        assert_eq!(fits_criteria("111122", true), true)
    }
}
