use std::{
    collections::HashMap,
    fmt::{self, Debug, Display},
    str::FromStr,
};

use anyhow::{bail, Result};
use num_integer::Integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instructions(Vec<Direction>);

impl Instructions {
    fn cycle(&self) -> impl Iterator<Item = Direction> + '_ {
        self.0.iter().copied().cycle()
    }
}

impl FromStr for Instructions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let directions = s
            .chars()
            .map(|c| match c {
                'L' => Ok(Direction::Left),
                'R' => Ok(Direction::Right),
                _ => Err(anyhow::anyhow!("Invalid direction: {:?}", c)),
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self(directions))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Node(u8, u8, u8);

impl Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.0 as char, self.1 as char, self.2 as char)
    }
}

impl Node {
    const START: Node = Node(b'A', b'A', b'A');
    const END: Node = Node(b'Z', b'Z', b'Z');

    const fn is_ghost_start(self) -> bool {
        self.2 == b'A'
    }

    const fn is_ghost_end(self) -> bool {
        self.2 == b'Z'
    }
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [first, second, third] = s.as_bytes() else {
            bail!("Expected 3 character long string: {:?}", s);
        };

        Ok(Self(*first, *second, *third))
    }
}

#[derive(Debug)]
pub struct DesertMap {
    instructions: Instructions,
    map: HashMap<Node, (Node, Node)>,
}

impl DesertMap {
    fn steps_impl<F>(&self, start: Node, is_end: F) -> anyhow::Result<u64>
    where
        F: Fn(Node) -> bool,
    {
        let mut steps = 0;
        let mut current = start;
        let mut instructions = self.instructions.cycle();

        while !is_end(current) {
            let Some(direction) = instructions.next() else {
                bail!("Unexpected end of instructions");
            };
            let Some((left, right)) = self.map.get(&current) else {
                bail!("No entry found for {} in map", current);
            };
            current = match direction {
                Direction::Left => *left,
                Direction::Right => *right,
            };
            steps += 1;
        }

        Ok(steps)
    }

    fn steps(&self) -> Result<u64> {
        self.steps_impl(Node::START, |node| node == Node::END)
    }

    fn ghost_steps(&self) -> Result<u64> {
        self.map
            .keys()
            .copied()
            .filter(|node| node.is_ghost_start())
            .map(|start| self.steps_impl(start, Node::is_ghost_end))
            .try_fold(1, |acc, steps| Ok(acc.lcm(&steps?)))
    }
}

impl FromStr for DesertMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let instructions = lines
            .next()
            .ok_or_else(|| anyhow::anyhow!("Expected instruction line"))?
            .parse::<Instructions>()?;

        let lines = lines.skip(1); // Skip the empty line

        let mut map = HashMap::new();
        for (idx, line) in lines.enumerate() {
            if line.len() != 16 {
                bail!("Instruction {} is not 15 characters long: {:?}", idx, line);
            }

            map.insert(
                line[0..3].parse()?,
                (line[7..10].parse()?, line[12..15].parse()?),
            );
        }

        Ok(Self { instructions, map })
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> anyhow::Result<DesertMap> {
    input.parse()
}

#[aoc(day8, part1)]
pub fn part1(input: &DesertMap) -> u64 {
    input.steps().unwrap()
}

#[aoc(day8, part2)]
pub fn part2(input: &DesertMap) -> u64 {
    input.ghost_steps().unwrap()
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
        assert_eq!(2, part1(&input_generator(input1).unwrap()));
        assert_eq!(6, part1(&input_generator(input2).unwrap()));
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
        assert_eq!(6, part2(&input_generator(input).unwrap()));
    }
}
