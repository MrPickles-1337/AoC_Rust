#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| l.split("   "))
        .map(|mut s| {
            (
                s.next().unwrap().parse::<u32>().unwrap(),
                s.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .fold((Vec::new(), Vec::new()), |mut yep, split| {
            yep.0.push(split.0);
            yep.1.push(split.1);
            yep
        })
}

#[aoc(day1, part1)]
pub fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut a = input.0.clone();
    let mut b = input.1.clone();
    a.sort();
    b.sort();
    a.iter().zip(b.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    input
        .0
        .iter()
        .map(|i| input.1.iter().filter(|j| *j == i).count() as u32 * i)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(11, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(31, part2(&input_generator(input)));
    }
}
