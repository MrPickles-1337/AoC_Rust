#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn move_next(
    position: &mut (usize, usize),
    direction: &mut Direction,
    pipe: &mut char,
    map: &Vec<Vec<char>>,
) {
    let next = match direction {
        Direction::Left => match pipe {
            '-' => Direction::Right,
            'J' => Direction::Up,
            '7' => Direction::Down,
            _ => unreachable!("IMPOSSIBLE: {}", pipe),
        },
        Direction::Right => match pipe {
            '-' => Direction::Left,
            'L' => Direction::Up,
            'F' => Direction::Down,
            _ => unreachable!("IMPOSSIBLE: {}", pipe),
        },
        Direction::Up => match pipe {
            '|' => Direction::Down,
            'L' => Direction::Right,
            'J' => Direction::Left,
            _ => unreachable!("IMPOSSIBLE: {}", pipe),
        },
        Direction::Down => match pipe {
            '|' => Direction::Up,
            '7' => Direction::Left,
            'F' => Direction::Right,
            _ => unreachable!("IMPOSSIBLE: {}", pipe),
        },
    };
    let (y, x) = match next {
        Direction::Left => (position.0, position.1 - 1),
        Direction::Right => (position.0, position.1 + 1),
        Direction::Up => (position.0 - 1, position.1),
        Direction::Down => (position.0 + 1, position.1),
    };
    if let Some(line) = map.get(y) {
        if let Some(char) = line.get(x) {
            *pipe = *char;
            *position = (y, x);
            *direction = match next {
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
            }
        }
    }
}

fn can_go(direction: Direction, pos: (usize, usize), map: &Vec<Vec<char>>) -> bool {
    if let Some(l) = map.get(pos.0) {
        if let Some(c) = l.get(pos.1) {
            match direction {
                Direction::Left => match c {
                    '-' | 'J' | '7' => true,
                    _ => false,
                },
                Direction::Right => match c {
                    '-' | 'F' | 'L' => true,
                    _ => false,
                },
                Direction::Up => match c {
                    '|' | 'L' | 'J' => true,
                    _ => false,
                },
                Direction::Down => match c {
                    '|' | 'F' | '7' => true,
                    _ => false,
                },
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn get_s_pos(input: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, l) in input.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if c == &'S' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<Vec<char>>) -> u32 {
    let start = get_s_pos(input);

    let mut dir1;
    let mut dir2;
    let mut pipe1;
    let mut pipe2;
    let mut pos1;
    let mut pos2;
    let yep = [
        (start.0, start.1 - 1, Direction::Right),
        (start.0, start.1 + 1, Direction::Left),
        (start.0 - 1, start.1, Direction::Down),
        (start.0 + 1, start.1, Direction::Up),
    ];
    let yep: Vec<_> = yep
        .iter()
        .filter(|a| can_go(a.2, (a.0, a.1), input))
        .collect();
    let first = yep.first().unwrap();
    let last = yep.last().unwrap();
    dir1 = first.2;
    dir2 = last.2;
    pos1 = (first.0, first.1);
    pos2 = (last.0, last.1);
    pipe1 = *input.get(pos1.0).unwrap().get(pos1.1).unwrap();
    pipe2 = *input.get(pos2.0).unwrap().get(pos2.1).unwrap();

    let mut count = 1;
    loop {
        move_next(&mut pos1, &mut dir1, &mut pipe1, input);
        move_next(&mut pos2, &mut dir2, &mut pipe2, input);
        count += 1;
        if pos1 == pos2 {
            return count;
        }
    }
}

#[aoc(day10, part2)]
pub fn part2(input: &Vec<Vec<char>>) -> u32 {
    let start = get_s_pos(input);

    let mut dir1;
    let mut dir2;
    let mut pipe1;
    let mut pipe2;
    let mut pos1;
    let mut pos2;
    let yep = [
        (start.0, start.1 - 1, Direction::Right),
        (start.0, start.1 + 1, Direction::Left),
        (start.0 - 1, start.1, Direction::Down),
        (start.0 + 1, start.1, Direction::Up),
    ];
    let yep: Vec<_> = yep
        .iter()
        .filter(|a| can_go(a.2, (a.0, a.1), input))
        .collect();
    let first = yep.first().unwrap();
    let last = yep.last().unwrap();
    dir1 = first.2;
    dir2 = last.2;
    pos1 = (first.0, first.1);
    pos2 = (last.0, last.1);
    pipe1 = *input.get(pos1.0).unwrap().get(pos1.1).unwrap();
    pipe2 = *input.get(pos2.0).unwrap().get(pos2.1).unwrap();

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4, part1(&input_generator(input)));
    }
}
