#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| return elf.lines().flat_map(|num| str::parse::<usize>(num)).sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut max = input
        .split("\n\n")
        .map(|elf| return elf.lines().flat_map(|num| str::parse::<usize>(num)).sum())
        .collect::<Vec<usize>>();

    max.sort_by(|a, b| b.cmp(a));
    max.iter().take(3).sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(24000, part1(input));
    }

    #[test]
    fn part2_test() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(45000, part2(input));
    }
}
