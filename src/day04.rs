#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn check(x: usize, y: usize, map: &[Vec<char>]) -> bool {
    let deltas = [
        (-1, -1),
        (-1, 0),
        (0, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
        (1, 0),
        (0, 1),
    ];
    let mut n = 0;
    for delta in deltas {
        if let Some(l) = map.get((y as isize + delta.0) as usize)
            && let Some(c) = l.get((x as isize + delta.1) as usize)
            && *c == '@'
        {
            n += 1;
        }

        if n == 4 {
            return false;
        }
    }
    true
}

#[aoc(day4, part1)]
pub fn part1(input: &[Vec<char>]) -> u32 {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter(move |(j, c)| **c == '@' && check(*j, i, input))
        })
        .count() as u32
}

pub fn get_can_be_removed(input: &[Vec<char>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            l.iter()
                .enumerate()
                .filter(move |(j, c)| **c == '@' && check(*j, i, input))
                .map(move |(j, _)| (i, j))
        })
        .collect::<Vec<_>>()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Vec<char>]) -> u32 {
    let mut input = Vec::from(input);
    let mut count = 0;
    let mut can_be_removed = get_can_be_removed(&input);
    loop {
        count += can_be_removed.len();
        for (i, j) in can_be_removed {
            *input.get_mut(i).unwrap().get_mut(j).unwrap() = '.';
        }
        can_be_removed = get_can_be_removed(&input);
        if can_be_removed.is_empty() {
            break;
        }
    }
    count as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(13, part1(&input_generator(input)));
    }
    #[test]
    fn part2_test() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(43, part2(&input_generator(input)));
    }
}
