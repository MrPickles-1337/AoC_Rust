#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<u8>]) -> u32 {
    input
        .iter()
        .map(|bank| {
            let mut a = *bank.first().unwrap();
            let mut n = a * 10 + bank.get(1).unwrap();
            for i in 0..bank.len() {
                let new_a = bank[i];
                if new_a < a {
                    continue;
                } else {
                    a = new_a;
                }
                for j in bank.iter().skip(i + 1) {
                    let new_n = a * 10 + j;
                    if new_n > n {
                        n = new_n;
                    }
                }
            }
            n as u32
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<u8>]) -> usize {
    // input.iter().map(|bank| {
    //     let mut a = *bank.first().unwrap();
    //     // let mut n =
    // })
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(357, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(3121910778619, part2(&input_generator(input)));
    }
}
