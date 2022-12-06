use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

const START_LOWER: u8 = b'a' - 1;
const START_UPPER: u8 = b'A' - 1;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| {
            let half = line.len() / 2;
            let (a, b) = line.split_at(half);
            let a = a.chars().collect::<HashSet<_>>();
            return b
                .chars()
                .filter(move |c| a.contains(c))
                .collect::<HashSet<char>>()
                .into_iter()
                .map(|c| {
                    let value = if c.is_ascii_lowercase() {
                        c as u8 - START_LOWER
                    } else {
                        c as u8 - START_UPPER + 26
                    };
                    return value;
                })
                .map(|c| c as usize);
        })
        .sum::<usize>()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .array_chunks::<3>()
        .flat_map(|line| {
            return line
                .iter()
                .flat_map(|line| line.chars().collect::<HashSet<_>>().into_iter())
                .fold(HashMap::new(), |mut map: HashMap<char, u32>, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                })
                .into_iter()
                .filter(|(_, v)| *v == 3);
        })
        .map(|c| c.0)
        .map(|c| {
            let value = if c.is_ascii_lowercase() {
                c as u8 - START_LOWER
            } else {
                c as u8 - START_UPPER + 26
            } as usize;
            return value;
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(157, part1(input));
    }

    #[test]
    fn part2_test() {}
}
