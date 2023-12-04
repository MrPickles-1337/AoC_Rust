#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let nums = l
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>();
            nums.first().unwrap() * 10 + nums.last().unwrap()
        })
        .sum()
}

const NUMS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_digit(s: &str) -> Option<u32> {
    let c = s.chars().next()?;
    if let r @ Some(_) = c.to_digit(10) {
        return r;
    }

    for (str, n) in NUMS {
        if s.starts_with(str) {
            return Some(n.to_owned());
        }
    }
    None
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut digits = (0..l.len()).filter_map(|i| get_digit(&l[i..]));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn part1_test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, part1(input));
    }

    #[test]
    fn part2_test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, part2(input));
    }

    #[test]
    fn part1_with_input_test() {
        let input = fs::read_to_string("input/2023/day1.txt").unwrap();
        assert_eq!(54644, part1(input.as_str()));
    }

    #[test]
    fn part2_with_input_test() {
        let input = fs::read_to_string("input/2023/day1.txt").unwrap();
        assert_eq!(53348, part2(input.as_str()));
    }
}
