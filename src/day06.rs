use regex::Regex;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(u32, u32)> {
    let rx = Regex::new(r"\d+").unwrap();
    let i: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            rx.find_iter(l)
                .map(|x| x.as_str().parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    i[0].iter()
        .zip(i[1].iter())
        .map(|(a, b)| (*a, *b))
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<(u32, u32)>) -> u32 {
    input
        .iter()
        .map(|&(time, distance)| {
            let yep = 2;
            2
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, part1(&input_generator(input)));
    }
}
