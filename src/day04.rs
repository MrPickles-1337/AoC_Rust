#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();
            let a1 = str::parse::<usize>(a1).unwrap();
            let a2 = str::parse::<usize>(a2).unwrap();
            let b1 = str::parse::<usize>(b1).unwrap();
            let b2 = str::parse::<usize>(b2).unwrap();

            (a1 >= b1 && a2 <= b2) || (b1 >= a1 && b2 <= a2)
        })
        .fold(0, |a, v| if v { a + 1 } else { a })
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let (a1, a2) = a.split_once('-').unwrap();
            let (b1, b2) = b.split_once('-').unwrap();
            let a1 = str::parse::<usize>(a1).unwrap();
            let a2 = str::parse::<usize>(a2).unwrap();
            let b1 = str::parse::<usize>(b1).unwrap();
            let b2 = str::parse::<usize>(b2).unwrap();

            (a1 >= b1 && a1 <= b2)
                || (b1 >= a1 && b1 <= a2)
                || (a2 >= b1 && a2 <= b2)
                || (b2 >= a1 && b2 <= a2)
        })
        .fold(0, |a, v| if v { a + 1 } else { a })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(2, part1(input));
    }

    #[test]
    fn part2_test() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(4, part2(input));
    }
}
