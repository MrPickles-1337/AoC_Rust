use regex::Regex;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<(u64, u64)> {
    let rx = Regex::new(r"\d+").unwrap();
    let i: Vec<Vec<u64>> = input
        .lines()
        .map(|l| {
            rx.find_iter(l)
                .map(|x| x.as_str().parse::<u64>().unwrap())
                .collect()
        })
        .collect();
    i[0].iter()
        .zip(i[1].iter())
        .map(|(a, b)| (*a, *b))
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .map(|&(time, distance)| {
            (0..time)
                .map(|hold_time /* speed */| {
                    let travel_time = time - hold_time;
                    let traveled_distance = travel_time * hold_time;
                    traveled_distance > distance
                })
                .filter(|&a| a)
                .count() as u64
        })
        .product()
}

#[aoc(day6, part2)]
pub fn part2(input: &[(u64, u64)]) -> u64 {
    let mut time = 0;
    let mut distance = 0;
    for (t, d) in input.iter() {
        time = time * 10u64.pow(t.to_string().len() as u32) + t;
        distance = distance * 10u64.pow(d.to_string().len() as u32) + d;
    }

    (0..time)
        .map(|hold_time /* speed */| {
            let travel_time = time - hold_time;
            let traveled_distance = travel_time * hold_time;
            traveled_distance > distance
        })
        .filter(|&a| a)
        .count() as u64
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

    #[test]
    fn part2_test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(71503, part2(&input_generator(input)));
    }
}
