use std::collections::VecDeque;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut spl = input.split("\n\n");
    (
        spl.next()
            .unwrap()
            .lines()
            .map(|l| {
                let mut line_spl = l.split('-');
                let start = line_spl.next().unwrap().parse::<usize>().unwrap();
                let end = line_spl.next().unwrap().parse::<usize>().unwrap();
                (start, end)
            })
            .collect(),
        spl.next()
            .unwrap()
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
    )
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    input
        .1
        .iter()
        .filter(|i| input.0.iter().any(|range| **i >= range.0 && **i <= range.1))
        .count()
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<(usize, usize)>, Vec<usize>)) -> usize {
    let mut sorted_ranges = input.0.clone();
    sorted_ranges.sort_by_key(|&(start, _)| start);

    let mut merged = VecDeque::new();
    merged.push_back(sorted_ranges[0]);

    for &(start, end) in &sorted_ranges[1..] {
        let &(last_start, last_end) = merged.back().unwrap();

        if start <= last_end + 1 {
            let new_end = end.max(last_end);
            *merged.back_mut().unwrap() = (last_start, new_end);
        } else {
            merged.push_back((start, end));
        }
    }
    merged.iter().map(|&(start, end)| end - start + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(3, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(14, part2(&input_generator(input)));
    }
}
