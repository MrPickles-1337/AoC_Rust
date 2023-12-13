use std::collections::HashMap;

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
}

#[aoc_generator(day8)]
pub fn input_generator(
    input: &str,
) -> (
    String,
    String,
    Vec<Direction>,
    HashMap<String, (String, String)>,
) {
    let mut spl = input.split("\n\n");
    let instructions = spl
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => unreachable!(),
        })
        .collect();
    let mut connections = HashMap::new();
    let connections_str = spl.next().unwrap();
    connections_str.lines().for_each(|l| {
        let yep = l
            .replace("= ", "")
            .replace('(', "")
            .replace(')', "")
            .replace(',', "");
        let mut split = yep.split(' ');
        connections.insert(
            split.next().unwrap().to_string(),
            (
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string(),
            ),
        );
    });
    let first = connections_str
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .next()
        .unwrap()
        .to_string();
    let last = connections_str
        .lines()
        .next_back()
        .unwrap()
        .split(' ')
        .next()
        .unwrap()
        .to_string();
    (first, last, instructions, connections)
}

fn step<'a>(
    pos: &'a String,
    destination: &String,
    directions: &[Direction],
    connections: &'a HashMap<String, (String, String)>,
) -> (bool, u32, &'a String) {
    if directions.is_empty() {
        return (false, 0, pos);
    }
    let direction = directions.first().unwrap();
    match direction {
        Direction::Right => {
            let next = &connections.get(pos).unwrap().1;
            if next == destination {
                return (true, 1, pos);
            } else {
                let (a, b, pos) = step(next, destination, &directions[1..], connections);
                return (a, b + 1, pos);
            }
        }
        Direction::Left => {
            let next = &connections.get(pos).unwrap().0;
            if next == destination {
                return (true, 1, pos);
            } else {
                let (a, b, pos) = step(next, destination, &directions[1..], connections);
                return (a, b + 1, pos);
            }
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(
    input: &(
        String,
        String,
        Vec<Direction>,
        HashMap<String, (String, String)>,
    ),
) -> u32 {
    let mut result = 0;
    let mut pos = &input.0;
    loop {
        let (found, steps, last_pos) = step(pos, &input.1, &input.2, &input.3);
        if found {
            return result + steps;
        } else {
            result += steps;
            pos = last_pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input1 = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, part1(&input_generator(input1)));
        assert_eq!(6, part1(&input_generator(input2)));
    }
}
