use std::collections::HashMap;

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> (Vec<Direction>, HashMap<String, (String, String)>) {
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
            .replace(['(', ')', ','], "");
        let mut split = yep.split(' ');
        connections.insert(
            split.next().unwrap().to_string(),
            (
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string(),
            ),
        );
    });
    (instructions, connections)
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
                (true, 1, pos)
            } else {
                let (a, b, pos) = step(next, destination, &directions[1..], connections);
                (a, b + 1, pos)
            }
        }
        Direction::Left => {
            let next = &connections.get(pos).unwrap().0;
            if next == destination {
                (true, 1, pos)
            } else {
                let (a, b, pos) = step(next, destination, &directions[1..], connections);
                (a, b + 1, pos)
            }
        }
    }
}

fn step_part2<'a>(
    pos: &'a String,
    directions: &[Direction],
    connections: &'a HashMap<String, (String, String)>,
) -> (bool, u32, &'a String) {
    if directions.is_empty() {
        return (false, 0, pos);
    }
    let direction = directions.first().unwrap();
    let next = match direction {
        Direction::Right => &connections.get(pos).unwrap().1,
        Direction::Left => &connections.get(pos).unwrap().0,
    };

    if next.ends_with('Z') {
        (true, 1, pos)
    } else {
        let (a, b, pos) = step_part2(next, &directions[1..], connections);
        (a, b + 1, pos)
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &(Vec<Direction>, HashMap<String, (String, String)>)) -> u32 {
    let mut result = 0;
    let mut pos = &String::from("AAA");
    loop {
        let (found, steps, last_pos) = step(pos, &"ZZZ".to_string(), &input.0, &input.1);
        if found {
            return result + steps;
        } else {
            result += steps;
            pos = last_pos;
        }
    }
}

#[aoc(day8, part2)]
pub fn part2(input: &(Vec<Direction>, HashMap<String, (String, String)>)) -> u32 {
    let mut yep: Vec<u32> = input
        .1
        .iter()
        .filter(|node| node.0.ends_with('A'))
        .map(|a| a.0)
        .map(|node| {
            let mut result = 0;
            let mut pos = node;
            loop {
                let (found, steps, last_pos) = step_part2(pos, &input.0, &input.1);
                if found {
                    return result + steps;
                } else {
                    result += steps;
                    pos = last_pos;
                }
            }
        })
        .collect();
    yep.sort();
    lcm(&yep)
}

fn lcm(nums: &[u32]) -> u32 {
    let mut result = 1;
    for &num in nums {
        result = num * result / gcd(num, result);
    }
    result
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
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

    #[test]
    fn part2_test() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, part2(&input_generator(input)));
    }
}
