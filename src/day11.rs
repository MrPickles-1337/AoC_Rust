use std::{borrow::BorrowMut, cell::RefCell, ops::Deref};

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
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let operation_str = lines.next().unwrap().replace("  Operation: new = old", "");
            let op = operation_str.chars().next().unwrap();
            let value = operation_str.split(' ').last().unwrap().parse::<u32>().ok();
            let operation = move |item: u32| -> u32 {
                match op {
                    '+' => item + value.unwrap_or(item),
                    '*' => item * value.unwrap_or(item),
                    _ => unreachable!(),
                }
            };

            let test_str = lines.next().unwrap().replace("  Test: divisible by ", "");
            let test_value = test_str.parse::<u32>().unwrap();
            let test = move |item: u32| -> bool { item % test_value == 0 };

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
                operation: Box::new(operation),
                test: Box::new(test),
                monkey_index_to_pass_if_true: if_true,
                monkey_index_to_pass_if_false: if_false,
            })
        })
        .collect()
}

struct Monkey {
    pub items: Vec<u32>,
    pub operation: Box<dyn Fn(u32) -> u32>,
    pub test: Box<dyn Fn(u32) -> bool>,
    pub monkey_index_to_pass_if_true: usize,
    pub monkey_index_to_pass_if_false: usize,
}

#[aoc(day11, part1)]
fn part1(input: &str) -> u32 {
    let mut input = input_generator(input);
    for _ in 0..20 {
        for monkey in input.iter_mut() {
            let item = monkey.get_mut().items.remove(0);
            let new = monkey.borrow().operation.deref()(item);
            let test = monkey.borrow().test.deref()(item);
            if test {
                input
                    .get(monkey.borrow().monkey_index_to_pass_if_true)
                    .unwrap()
                    .get_mut()
                    .items
                    .push(new);
            }
        }
    }
    todo!()
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
}
