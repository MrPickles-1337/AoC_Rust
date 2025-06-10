use std::collections::HashSet;

pub struct Grid(Vec<Vec<u8>>);

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Grid {
    Grid(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect(),
    )
}

fn within_bounds(height: usize, width: usize, row: isize, col: isize) -> bool {
    row >= 0 && (row as usize) < width && col >= 0 && (col as usize) < height
}

fn get_trailheads(
    h: u8,
    pos: (usize, usize),
    grid: &[Vec<u8>],
    height: usize,
    width: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    if h == 9 {
        visited.insert(pos);
    }
    let moves = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for m in moves {
        let new_pos = (pos.0 as isize + m.0, pos.1 as isize + m.1);
        if !within_bounds(height, width, new_pos.0, new_pos.1) {
            continue;
        }
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        if grid[new_pos.0][new_pos.1] == h + 1 {
            get_trailheads(h + 1, new_pos, grid, height, width, visited);
        }
    }
    visited.len()
}

#[aoc(day10, part1)]
pub fn part1(input: &Grid) -> usize {
    let height = input.0.len();
    let width = input.0[0].len();
    input
        .0
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, i)| **i == 0)
                .map(move |(j, col)| {
                    get_trailheads(*col, (i, j), &input.0, height, width, &mut HashSet::new())
                })
        })
        .sum()
}

fn get_trailheads_p2(
    h: u8,
    pos: (usize, usize),
    grid: &[Vec<u8>],
    height: usize,
    width: usize,
) -> usize {
    if h == 9 {
        return 1;
    }
    let moves = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut paths = 0;
    for m in moves {
        let new_pos = (pos.0 as isize + m.0, pos.1 as isize + m.1);
        if !within_bounds(height, width, new_pos.0, new_pos.1) {
            continue;
        }
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        if grid[new_pos.0][new_pos.1] == h + 1 {
            paths += get_trailheads_p2(h + 1, new_pos, grid, height, width);
        }
    }
    paths
}

#[aoc(day10, part2)]
pub fn part2(input: &Grid) -> usize {
    let height = input.0.len();
    let width = input.0[0].len();
    input
        .0
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, i)| **i == 0)
                .map(move |(j, col)| get_trailheads_p2(*col, (i, j), &input.0, height, width))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_i1_test() {
        let input = "1110111
1111111
1112111
6543456
7111117
8111118
9111119";
        assert_eq!(2, part1(&input_generator(input)));
    }

    #[test]
    fn part1_i2_test() {
        let input = "1088988
2888888
3888788
4567654
8888883
8889882
8888801";

        assert_eq!(3, part1(&input_generator(input)));
    }

    #[test]
    fn part1_i3_test() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        assert_eq!(36, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        assert_eq!(81, part2(&input_generator(input)));
    }
}
