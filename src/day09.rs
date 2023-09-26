use std::{cell::RefCell, collections::HashSet};

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|l| {
            let mut s = l.split(' ');
            let c = s.next().unwrap().chars().next().unwrap();
            let i = s.next().unwrap().parse::<i32>().unwrap();
            (c, i)
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
    visited.len()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Knot {
    x: i32,
    y: i32,
    last: bool,
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<(char, i32)>) -> usize {
    let mut knots: Vec<_> = vec![
        RefCell::new(Knot {
            x: 0,
            y: 0,
            last: false
        });
        9
    ];
    knots.push(RefCell::new(Knot {
        x: 0,
        y: 0,
        last: true,
    }));
    let mut visited: HashSet<Knot> = HashSet::new();
    visited.insert(*knots.first().unwrap().borrow());
    for i in input {
        for _ in 0..i.1 {
            match i.0 {
                'U' => {
                    knots.first_mut().unwrap().borrow_mut().y += 1;
                }
                'D' => {
                    knots.first_mut().unwrap().borrow_mut().y -= 1;
                }
                'L' => {
                    knots.first_mut().unwrap().borrow_mut().x -= 1;
                }
                'R' => {
                    knots.first_mut().unwrap().borrow_mut().x += 1;
                }
                _ => unreachable!(),
            }

            knots.windows(2).for_each(|chunk| {
                let head = chunk.get(0).unwrap().borrow();
                let mut tail = chunk.get(1).unwrap().borrow_mut();
                let (mut dx, mut dy) = (head.x - tail.x, head.y - tail.y);
                while dx.abs() > 1 || dy.abs() > 1 {
                    tail.x += dx.signum();
                    tail.y += dy.signum();
                    (dx, dy) = (head.x - tail.x, head.y - tail.y);
                    if tail.last && !visited.contains(&tail) {
                        visited.insert(*tail);
                    }
                }
            });
        }
    }
    visited.len()
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

    #[test]
    fn part1_test1() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(1, part2(&input_generator(input)));
    }
    #[test]
    fn part1_test2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        assert_eq!(36, part2(&input_generator(input)));
    }
}
