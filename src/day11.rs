use std::collections::HashMap;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(' ').map(|i| i.parse().unwrap()).collect()
}

fn process(a: &mut Vec<usize>, b: &mut Vec<usize>) {
    while let Some(i) = a.pop() {
        if i == 0 {
            b.push(1);
        } else {
            let digits = i.ilog10() + 1;
            if digits % 2 == 0 {
                let half_digits = digits / 2;
                let yep = 10usize.pow(half_digits);
                b.push(i / yep);
                b.push(i % yep);
            } else {
                b.push(i * 2024);
            }
        }
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut a = Vec::from(input);
    for _ in 0..25 {
        let mut b = Vec::new();
        process(&mut a, &mut b);
        a = b;
    }
    a.len()
}

pub fn blink_stone(i: usize, blinks: u32) -> Vec<usize> {
    let mut a = vec![i];
    for i in 0..blinks {
        println!("blink {i}");
        let mut b = Vec::new();
        process(&mut a, &mut b);
        a = b;
    }
    a
}

pub fn wink_stone(i: usize, blinks: u32, cache: &mut HashMap<(usize, u32), usize>) -> usize {
    eprintln!("cache size: {}", cache.len());
    println!("wink: {blinks}");
    if blinks == 0 {
        return 1;
    }
    if let Some(e) = cache.get(&(i, blinks)) {
        return *e;
    }
    let mut total = 0;
    let stones = blink_stone(i, blinks);
    for s in stones {
        total += wink_stone(s, blinks - 1, cache);
    }

    cache.insert((i, blinks), total);
    total
}

#[aoc(day11, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut cache = HashMap::new();
    input.iter().map(|i| wink_stone(*i, 75, &mut cache)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "125 17";

        assert_eq!(55312, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "125 17";

        assert_eq!(55312, part2(&input_generator(input)));
    }
}
