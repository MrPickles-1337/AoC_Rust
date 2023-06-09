pub struct Round {
    yours: Shape,
    others: Shape,
}

#[derive(PartialEq, Debug)]
enum Shape {
    Rock(usize),
    Paper(usize),
    Scissors(usize),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|line| {
            let mut s = line.split(' ');
            return Round {
                others: match s.next().unwrap().chars().next().unwrap() {
                    'A' => Shape::Rock(1),
                    'B' => Shape::Paper(2),
                    'C' => Shape::Scissors(3),
                    _ => unreachable!(),
                },
                yours: match s.next().unwrap().chars().next().unwrap() {
                    'X' => Shape::Rock(1),
                    'Y' => Shape::Paper(2),
                    'Z' => Shape::Scissors(3),
                    _ => unreachable!(),
                },
            };
        })
        .collect()
}

fn calculate_round_part1(round: &Round) -> usize {
    // println!("yours: {:?}, others: {:?}", round.yours, round.others);
    if round.yours == round.others {
        match round.yours {
            Shape::Paper(n) | Shape::Rock(n) | Shape::Scissors(n) => return 3 + n,
        }
    }
    match round.yours {
        Shape::Rock(n) => match round.others {
            Shape::Paper(_) => n,
            Shape::Scissors(_) => 6 + n,
            Shape::Rock(_) => unreachable!(),
        },
        Shape::Paper(n) => match round.others {
            Shape::Rock(_) => 6 + n,
            Shape::Scissors(_) => n,
            Shape::Paper(_) => unreachable!(),
        },
        Shape::Scissors(n) => match round.others {
            Shape::Rock(_) => n,
            Shape::Paper(_) => 6 + n,
            Shape::Scissors(_) => unreachable!(),
        },
    }
}

fn calculate_round_part2(round: &Round) -> usize {
    match round.others {
        Shape::Rock(_) => match round.yours {
            Shape::Rock(_) => 3,
            Shape::Paper(_) => 3 + 1,
            Shape::Scissors(_) => 6 + 2,
        },
        Shape::Paper(_) => match round.yours {
            Shape::Rock(_) => 1,
            Shape::Paper(_) => 3 + 2,
            Shape::Scissors(_) => 6 + 3,
        },
        Shape::Scissors(_) => match round.yours {
            Shape::Rock(_) => 2,
            Shape::Paper(_) => 3 + 3,
            Shape::Scissors(_) => 6 + 1,
        },
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &[Round]) -> usize {
    input.iter().map(calculate_round_part1).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Round]) -> usize {
    input.iter().map(calculate_round_part2).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(15, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "A Y\nB X\nC Z";
        assert_eq!(12, part2(&input_generator(input)));
    }
}
