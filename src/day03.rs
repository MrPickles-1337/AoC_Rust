use std::collections::HashSet;

type Position = (i32, i32);

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let mut houses: HashSet<Position> = HashSet::new();
    let mut current_pos = (0, 0);

    houses.insert(current_pos);

    for &c in input.as_bytes() {
        match c {
            b'>' => current_pos.0 += 1,
            b'<' => current_pos.0 -= 1,
            b'^' => current_pos.1 += 1,
            b'v' => current_pos.1 -= 1,
            _ => unreachable!(),
        }

        houses.insert(current_pos);
    }

    houses.len()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut houses: HashSet<Position> = HashSet::new();
    let mut santa = (0, 0);
    let mut robo_santa = (0, 0);
    houses.insert(santa);
    for (i, &c) in input.as_bytes().iter().enumerate() {
        let current_pos = if i % 2 == 0 {
            &mut santa
        } else {
            &mut robo_santa
        };

        match c {
            b'>' => current_pos.0 += 1,
            b'<' => current_pos.0 -= 1,
            b'^' => current_pos.1 += 1,
            b'v' => current_pos.1 -= 1,
            _ => unreachable!(),
        }
        houses.insert(*current_pos);
    }
    houses.len()
}
