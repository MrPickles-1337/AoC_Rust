use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    col: i64,
    row: i64,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col - rhs.col,
            row: self.row - rhs.row,
        }
    }
}

impl Position {
    fn new(col: usize, row: usize) -> Self {
        Self {
            col: col.try_into().unwrap(),
            row: row.try_into().unwrap(),
        }
    }
    fn check_bounds(&self, width: i64, height: i64) -> bool {
        0 <= self.col && self.col < width && 0 <= self.row && self.row < height
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> ((usize, usize), HashMap<char, Vec<Position>>) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();
    let roof = input.lines().enumerate().fold(
        HashMap::<char, Vec<Position>>::new(),
        |antennas, (row, line)| {
            line.char_indices().filter(|(_, c)| *c != '.').fold(
                antennas,
                |mut antennas, (col, char)| {
                    antennas
                        .entry(char)
                        .or_default()
                        .push(Position::new(col, row));
                    antennas
                },
            )
        },
    );
    ((height, width), roof)
}

#[aoc(day8, part1)]
pub fn part1(input: &((usize, usize), HashMap<char, Vec<Position>>)) -> usize {
    let height = input.0 .0 as i64;
    let width = input.0 .1 as i64;
    let mut antinodes: HashSet<Position> = HashSet::new();

    for positions in input.1.values() {
        for pair in positions.iter().combinations(2) {
            let a1 = *pair[0];
            let a2 = *pair[1];

            let diff = a2 - a1;

            let anti1 = a2 + diff;
            let anti2 = a1 - diff;

            if anti1.check_bounds(width, height) {
                antinodes.insert(anti1);
            }

            if anti2.check_bounds(width, height) {
                antinodes.insert(anti2);
            }
        }
    }
    antinodes.len()
}

#[aoc(day8, part2)]
pub fn part2(input: &((usize, usize), HashMap<char, Vec<Position>>)) -> usize {
    let height = input.0 .0 as i64;
    let width = input.0 .1 as i64;
    let mut antinodes: HashSet<Position> = HashSet::new();

    for positions in input.1.values() {
        for pair in positions.iter().combinations(2) {
            let a1 = *pair[0];
            let a2 = *pair[1];

            let diff = a2 - a1;

            let mut antinode = a1;

            while antinode.check_bounds(width, height) {
                antinodes.insert(antinode);
                antinode = antinode - diff;
            }

            let mut antinode = a2;

            while antinode.check_bounds(width, height) {
                antinodes.insert(antinode);
                antinode = antinode + diff;
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(14, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(34, part2(&input_generator(input)));
    }
}
