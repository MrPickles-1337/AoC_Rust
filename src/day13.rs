use std::{cmp::Ordering, str::Chars};

#[derive(Debug)]
enum Node {
    Value(u8),
    List(Vec<Node>),
}

impl Node {
    pub fn compare(&self, other: &Node) -> Ordering {
        match self {
            Node::Value(self_v) => match other {
                Node::Value(other_v) => self_v.cmp(other_v),
                Node::List(_) => Node::List(vec![Node::Value(self_v.to_owned())]).compare(other),
            },
            Node::List(self_l) => match other {
                Node::Value(other_v) => {
                    Node::List(vec![Node::Value(other_v.to_owned())]).compare(self)
                }
                Node::List(other_l) => match self_l.len().cmp(&other_l.len()) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => {
                        for (i, j) in std::iter::zip(self_l, other_l) {
                            return match i.compare(j) {
                                Ordering::Less => Ordering::Less,
                                Ordering::Equal => continue,
                                Ordering::Greater => Ordering::Greater,
                            };
                        }
                        Ordering::Equal
                    }
                },
            },
        }
    }
}

fn parse_list(input: &mut Chars) -> Node {
    let mut result = Vec::new();
    while let Some(c) = input.next() {
        if c == ',' {
            continue;
        }
        if c == '[' {
            let list = parse_list(input);
            result.push(list);
            continue;
        }
        if c == ']' {
            break;
        }
        result.push(Node::Value(c.to_digit(10).unwrap() as u8));
    }
    Node::List(result)
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<(Node, Node)> {
    input
        .split("\n\n")
        .map(|s| {
            let mut spl = s.split('\n');
            let (left, right) = (spl.next().unwrap(), spl.next().unwrap());
            let mut l_chars = left.chars();
            let mut r_chars = right.chars();
            l_chars.next();
            r_chars.next();
            let left = parse_list(&mut l_chars);
            let right = parse_list(&mut r_chars);
            (left, right)
        })
        .collect()
}

#[aoc(day13, part1)]
fn part1(input: &[(Node, Node)]) -> usize {
    for (i, j) in input {
        println!("{:?}", i.compare(j));
    }
    input
        .iter()
        .filter(|n| matches!(n.0.compare(&n.1), Ordering::Less))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
        assert_eq!(13, part1(&input_generator(input)));
    }
}
