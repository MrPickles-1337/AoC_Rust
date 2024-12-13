use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Point {
    obstacle: bool,
    visited: bool,
}

impl Point {
    pub fn new(obstacle: bool) -> Self {
        Self {
            obstacle,
            visited: false,
        }
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
    pub fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    map: Vec<Vec<Point>>,
    initial_position: (usize, usize),
    position: (usize, usize),
    initial_direction: Direction,
    direction: Direction,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "pos: {}, {}", self.position.0, self.position.1)?;
        writeln!(f, "dir: {:?}", self.direction)?;
        for i in self.map.iter() {
            for j in i {
                let point = if j.obstacle {
                    '#'
                } else if j.visited {
                    'X'
                } else {
                    '.'
                };
                write!(f, "{point}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub enum StepResult {
    Ok,
    Loop,
    Outside,
}

impl Map {
    pub fn new(map: Vec<Vec<Point>>, position: (usize, usize), direction: Direction) -> Self {
        Self {
            map,
            initial_position: position,
            position,
            initial_direction: direction,
            direction,
        }
    }

    pub fn is_loop(&mut self) -> bool {
        loop {
            match self.step() {
                StepResult::Ok => {}
                StepResult::Loop => return true,
                StepResult::Outside => return false,
            }
        }
    }

    fn get_all_possible_positions(&mut self) -> Vec<(usize, usize)> {
        while let StepResult::Ok = self.step() {}
        self.map
            .iter()
            .enumerate()
            .flat_map(|(i, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(_, p)| p.visited)
                    .map(move |(j, _)| (i, j))
            })
            .collect()
    }

    pub fn step(&mut self) -> StepResult {
        // println!("{}", self);
        let next: (isize, isize) = match self.direction {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };
        let next = (
            self.position.0 as isize + next.0,
            self.position.1 as isize + next.1,
        );
        if next.0 < 0
            || next.1 < 0
            || next.0 >= self.map.len() as isize
            || next.1 >= self.map[0].len() as isize
        {
            println!("outside");
            return StepResult::Outside;
        }
        if self
            .map
            .get(next.0 as usize)
            .unwrap()
            .get(next.1 as usize)
            .unwrap()
            .obstacle
        {
            self.direction = self.direction.turn();
        } else {
            self.position = (next.0 as usize, next.1 as usize);
            self.map
                .get_mut(self.position.0)
                .unwrap()
                .get_mut(self.position.1)
                .unwrap()
                .visit();
        }

        if self.position == self.initial_position && self.direction == self.initial_direction {
            println!("loop");
            return StepResult::Loop;
        }
        println!("ok");
        StepResult::Ok
    }
}

#[aoc_generator(day6)]
pub fn input_genereator(input: &str) -> Map {
    let mut position = (0, 0);
    let mut direction = Direction::Up;
    let map = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '.' => Point::new(false),
                    '#' => Point::new(true),
                    '^' | '>' | 'v' | '<' => {
                        position = (i, j);
                        direction = Direction::from_char(c);
                        Point::new(false)
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    Map::new(map, position, direction)
}

#[aoc(day6, part1)]
pub fn part1(input: &Map) -> usize {
    let mut map = input.clone();
    map.get_all_possible_positions().len()
}

#[aoc(day6, part2)]
pub fn part2(input: &Map) -> u32 {
    let mut input_cloned = input.clone();
    let positions = input_cloned.get_all_possible_positions();
    positions
        .iter()
        .filter(|pos| {
            if pos == &&input.position {
                return false;
            }
            let mut input = input.clone();
            input
                .map
                .get_mut(pos.0)
                .unwrap()
                .get_mut(pos.1)
                .unwrap()
                .obstacle = true;

            let is_loop = input.is_loop();
            println!("{is_loop}");
            is_loop
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!(41, part1(&input_genereator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
        assert_eq!(6, part2(&input_genereator(input)));
    }
}
