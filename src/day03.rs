use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    Regex::new(r"mul(\(\d+,\d+\))")
        .unwrap()
        .captures_iter(input)
        .map(|c| c.extract::<1>().1)
        .map(|mul| {
            mul[0][1..mul[0].len() - 1]
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .product::<u32>()
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let input = input.lines().fold(String::new(), |mut s, l| {
        s.push_str(l);
        s
    });
    let mut enabled = String::new();
    let mut split = input.split("don't()");
    enabled.push_str(split.next().unwrap());
    for i in split {
        for j in i.split("do()").skip(1) {
            enabled.push_str(j);
        }
    }
    Regex::new(r"mul(\(\d+,\d+\))")
        .unwrap()
        .captures_iter(&enabled)
        .map(|c| c.extract::<1>().1)
        .map(|mul| {
            mul[0][1..mul[0].len() - 1]
                .split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, part1(input));
    }

    #[test]
    fn part2_test() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, part2(input));
    }
}
