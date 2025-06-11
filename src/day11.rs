use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(' ').map(|i| i.parse().unwrap()).collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut cache = HashMap::new();
    input
        .iter()
        .map(|i| blink_stone(*i, 0, 25, &mut cache))
        .sum()
}

pub fn blink_stone(
    i: usize,
    blinks: u32,
    stop_at: u32,
    cache: &mut HashMap<(usize, u32), usize>,
) -> usize {
    if blinks >= stop_at {
        return 1;
    }
    if let Some(e) = cache.get(&(i, blinks)) {
        return *e;
    }

    let result = if i == 0 {
        blink_stone(1, blinks + 1, stop_at, cache)
    } else {
        let digits = i.ilog10() + 1;
        if digits % 2 == 0 {
            let half_digits = digits / 2;
            let yep = 10usize.pow(half_digits);
            blink_stone(i / yep, blinks + 1, stop_at, cache)
                + blink_stone(i % yep, blinks + 1, stop_at, cache)
        } else {
            blink_stone(i * 2024, blinks + 1, stop_at, cache)
        }
    };

    cache.insert((i, blinks), result);
    result
}

#[aoc(day11, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut cache = HashMap::new();
    input
        .iter()
        .map(|i| blink_stone(*i, 0, 75, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "125 17";

        assert_eq!(55312, part1(&input_generator(input)));
    }
}
