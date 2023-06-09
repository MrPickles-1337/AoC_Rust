use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    for i in 0..chars.len() {
        let mut set = HashSet::new();
        for j in chars.iter().skip(i).take(4) {
            set.insert(j);
        }
        if set.len() == 4 {
            return i + 4;
        }
    }
    unreachable!();
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    for i in 0..chars.len() {
        let mut set = HashSet::new();
        for j in chars.iter().skip(i).take(14) {
            set.insert(j);
        }
        if set.len() == 14 {
            return i + 14;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, part1(input));
    }
    #[test]
    fn part1_test2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, part1(input));
    }
    #[test]
    fn part1_test3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, part1(input));
    }
    #[test]
    fn part1_test4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(10, part1(input));
    }
    #[test]
    fn part1_test5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(11, part1(input));
    }

    #[test]
    fn part2_test1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, part2(input));
    }
    #[test]
    fn part2_test2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(23, part2(input));
    }
    #[test]
    fn part2_test3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(23, part2(input));
    }
    #[test]
    fn part2_test4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(29, part2(input));
    }
    #[test]
    fn part2_test5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(26, part2(input));
    }
}
