#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|l| l.split(" ").map(|i| i.parse().unwrap()).collect())
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<i8>]) -> u32 {
    input.iter().filter(check).count() as u32
}

fn check(i: &&Vec<i8>) -> bool {
    let mut valid = true;
    let increasing = i[0] - i[1] > 0;
    for j in 0..i.len() - 1 {
        let diff = i[j] - i[j + 1];
        if increasing && diff < 0 {
            valid = false;
            break;
        }
        if !increasing && diff > 0 {
            valid = false;
            break;
        }
        if diff == 0 {
            valid = false;
            break;
        }
        if diff.abs() > 3 {
            valid = false;
            break;
        }
    }
    valid
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<i8>]) -> u32 {
    input
        .iter()
        .filter(|l| {
            if check(l) {
                return true;
            }
            for i in 0..l.len() {
                if check(&&[&l[0..i], &l[i + 1..]].concat()) {
                    return true;
                }
            }
            false
        })
        .count() as u32
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(2, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(4, part2(&input_generator(input)));
    }
}
