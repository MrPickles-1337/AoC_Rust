use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|l| {
            let mut s = l.chars();
            let c = s.next().unwrap();
            s.next().unwrap();
            let i = s.next().unwrap().to_digit(10).unwrap() as i32;
            return (c, i);
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<(char, i32)>) -> usize {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tail);
    for i in input {
        match i.0 {
            'U' => {
                head.1 += i.1;
            }
            'D' => {
                head.1 -= i.1;
            }
            'L' => {
                head.0 -= i.1;
            }
            'R' => {
                head.0 += i.1;
            }
            _ => unreachable!(),
        }
        let (mut dx, mut dy) = (head.0 - tail.0, head.1 - tail.1);
        while dx.abs() > 1 || dy.abs() > 1 {
            tail.0 += dx.signum();
            tail.1 += dy.signum();
            (dx, dy) = (head.0 - tail.0, head.1 - tail.1);
            if !visited.contains(&tail) {
                visited.insert(tail);
            }
        }
    }
    return visited.len();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_test() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, part1(&input_generator(input)));
    }
}
