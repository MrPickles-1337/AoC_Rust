use std::collections::VecDeque;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(':');
            let value = split.next().unwrap();
            let mut split = split.next().unwrap().split(' ');
            split.next().unwrap();
            (
                value.parse().unwrap(),
                split.map(|i| i.parse::<usize>().unwrap()).collect(),
            )
        })
        .collect()
}

fn calc_next(value: usize, expected: usize, mut values: VecDeque<usize>) -> bool {
    if let Some(next) = values.pop_front() {
        let with_plus = calc_next(value + next, expected, values.clone());
        let with_multi = calc_next(value * next, expected, values);
        with_plus || with_multi
    } else {
        value == expected
    }
}

// fn calc_next(mut value: usize, expected: usize, mut values: VecDeque<usize>) -> bool {
//     if let Some(next) = values.pop_front() {
//         let with_plus = calc_next(value + next, expected, values.clone());
//         if !with_plus {
//             if value == 0 {
//                 value = 1;
//             }
//             calc_next(value * next, expected, values)
//         } else {
//             true
//         }
//     } else {
//         value == expected
//     }
// }

#[aoc(day7, part1)]
pub fn part1(input: &[(usize, Vec<usize>)]) -> usize {
    input
        .iter()
        .map(|i| {
            let test = i.0;
            let mut values = VecDeque::from(i.1.clone());

            if calc_next(values.pop_front().unwrap(), test, values) {
                test
            } else {
                0
            }
        })
        .sum()
}

fn concat(a: usize, b: usize) -> usize {
    a * 10_usize.pow(b.ilog10() + 1) + b
}

fn calc_next_p2(value: usize, expected: usize, mut values: VecDeque<usize>) -> bool {
    if let Some(next) = values.pop_front() {
        let with_plus = calc_next_p2(value + next, expected, values.clone());
        let with_multi = calc_next_p2(value * next, expected, values.clone());
        let with_concat = calc_next_p2(concat(value, next), expected, values);
        with_plus || with_multi || with_concat
    } else {
        value == expected
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &[(usize, Vec<usize>)]) -> usize {
    input
        .iter()
        .map(|i| {
            let test = i.0;
            let mut values = VecDeque::from(i.1.clone());

            let res = calc_next_p2(values.pop_front().unwrap(), test, values);
            if res {
                test
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(3749, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(11387, part2(&input_generator(input)));
    }
}
