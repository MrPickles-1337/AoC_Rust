use std::collections::HashSet;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(usize, usize)> {
    input
        .split(',')
        .map(|range| {
            let mut split = range.split('-');
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        .map(|range| {
            (range.0..=range.1)
                .filter(|i| {
                    let mut s = i.to_string();
                    let half = s.split_off(s.len() / 2);
                    s == half
                })
                .sum::<usize>()
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        .map(|range| {
            (range.0..=range.1)
                .filter(|num| {
                    let s = num.to_string();
                    let n = s.len();
                    for len in 1..=(n / 2) {
                        if n % len == 0 {
                            let sub = &s[0..len];
                            let mut is_repeated = true;
                            for i in (len..n).step_by(len) {
                                if &s[i..(i + len)] != sub {
                                    is_repeated = false;
                                    break;
                                }
                            }
                            if is_repeated {
                                return true;
                            }
                        }
                    }
                    false
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(1227775554, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(4174379265, part2(&input_generator(input)));
    }
}
