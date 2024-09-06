use rayon::prelude::*;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    let input = input.replace("  ", " ");
    input
        .lines()
        .map(|l| {
            let card = l.split(": ").last().unwrap();
            let mut split = card.split(" | ");
            (
                split
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect(),
                split
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[(Vec<u32>, Vec<u32>)]) -> u32 {
    let mut result = 0;
    for game in input {
        let wining = &game.0;
        let numbers = &game.1;
        let mut first = true;
        let mut points = 0;
        for i in numbers {
            if wining.contains(i) {
                if first {
                    first = false;
                    points += 1;
                } else {
                    points *= 2;
                }
            }
        }
        result += points;
    }

    result
}

fn calc_card(winning: &[u32], points: &[u32], cards: &[(Vec<u32>, Vec<u32>)]) -> u32 {
    let mut count = 0;
    for i in points {
        if winning.contains(i) {
            count += 1;
        }
    }
    if count == 0 {
        1
    } else {
        let mut result = 1;
        for (i, card) in cards[1..count + 1].iter().enumerate() {
            result += calc_card(&card.0, &card.1, &cards[i + 1..]);
        }
        result
    }
}

#[aoc(day4, part2)]
pub fn part2(input: &[(Vec<u32>, Vec<u32>)]) -> u32 {
    input
        .par_iter()
        .enumerate()
        .map(|(i, card)| calc_card(&card.0, &card.1, &input[i..]))
        .sum()
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "full_input")]
    use std::fs;

    use super::*;

    #[test]
    fn part1_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, part2(&input_generator(input)));
    }

    #[cfg(feature = "full_input")]
    #[test]
    fn part1_with_input_test() {
        let input = fs::read_to_string("input/2023/day4.txt").unwrap();
        assert_eq!(17782, part1(&input_generator(input.as_str())));
    }

    #[cfg(feature = "full_input")]
    #[test]
    fn part2_with_input_test() {
        let input = fs::read_to_string("input/2023/day4.txt").unwrap();
        assert_eq!(8477787, part2(&input_generator(input.as_str())));
    }
}
