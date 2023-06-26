use std::{cell::RefCell, collections::VecDeque};

enum Op {
    Add(usize),
    Mul(usize),
    Square,
}

struct Monkey {
    pub items: VecDeque<usize>,
    pub operation: Op,
    pub test: usize,
    pub monkey_index_to_pass_if_true: usize,
    pub monkey_index_to_pass_if_false: usize,
    pub inspected: usize,
}

fn input_generator(input: &str) -> Vec<RefCell<Monkey>> {
    input
        .split("\n\n")
        .map(|monkey| {
            let mut lines = monkey.lines();
            lines.next().unwrap();

            let itemes_str = lines
                .next()
                .unwrap()
                .replace("  Starting items: ", "")
                .replace(' ', "");
            let items = itemes_str
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<VecDeque<usize>>();

            let operation_str = lines.next().unwrap().replace("  Operation: new = old ", "");
            let op = operation_str.chars().next().unwrap();
            let value = operation_str
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .ok();
            let operation = match value {
                Some(i) => match op {
                    '+' => Op::Add(i),
                    '*' => Op::Mul(i),
                    _ => unreachable!(),
                },
                None => Op::Square,
            };

            let test_str = lines.next().unwrap().replace("  Test: divisible by ", "");
            let test = test_str.parse::<usize>().unwrap();

            let if_true_str = lines
                .next()
                .unwrap()
                .replace("    If true: throw to monkey ", "");
            let if_true = if_true_str.parse::<usize>().unwrap();

            let if_false_str = lines
                .next()
                .unwrap()
                .replace("    If false: throw to monkey ", "");
            let if_false = if_false_str.parse::<usize>().unwrap();

            RefCell::new(Monkey {
                items,
                operation,
                test,
                monkey_index_to_pass_if_true: if_true,
                monkey_index_to_pass_if_false: if_false,
                inspected: 0,
            })
        })
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let mut input = input_generator(input);
    for _ in 0..20 {
        for monkey in input.iter() {
            let mut monkey = monkey.borrow_mut();
            while let Some(item) = monkey.items.pop_front() {
                monkey.inspected += 1;
                let new = match monkey.operation {
                    Op::Add(i) => item + i,
                    Op::Mul(i) => item * i,
                    Op::Square => item * item,
                } / 3;
                if new % monkey.test == 0 {
                    input[monkey.monkey_index_to_pass_if_true]
                        .borrow_mut()
                        .items
                        .push_back(new);
                } else {
                    input[monkey.monkey_index_to_pass_if_false]
                        .borrow_mut()
                        .items
                        .push_back(new);
                }
            }
        }
    }

    input.sort_by_key(|m| m.borrow().inspected);
    return input.pop().unwrap().borrow().inspected * input.pop().unwrap().borrow().inspected;
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let mut input = input_generator(input);
    let modulus = input.iter().map(|m| m.borrow().test).product::<usize>();
    for _ in 0..10000 {
        for monkey in input.iter() {
            let mut monkey = monkey.borrow_mut();
            while let Some(item) = monkey.items.pop_front() {
                monkey.inspected += 1;
                let new = match monkey.operation {
                    Op::Add(i) => item + i,
                    Op::Mul(i) => item * i,
                    Op::Square => item * item,
                } % modulus;
                if new % monkey.test == 0 {
                    input[monkey.monkey_index_to_pass_if_true]
                        .borrow_mut()
                        .items
                        .push_back(new);
                } else {
                    input[monkey.monkey_index_to_pass_if_false]
                        .borrow_mut()
                        .items
                        .push_back(new);
                }
            }
        }
    }

    input.sort_by_key(|m| m.borrow().inspected);
    return input.pop().unwrap().borrow().inspected * input.pop().unwrap().borrow().inspected;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(10605, part1(input));
    }

    #[test]
    fn part2_test() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(2713310158, part2(input));
    }
}
