#[derive(Debug, Clone, Copy)]
pub struct Container(char);

impl Container {
    fn from(input: &str) -> Container {
        return Container(input.chars().nth(1).unwrap());
    }
}

#[derive(Debug)]
pub struct Operation {
    count: usize,
    from: usize,
    to: usize,
}

impl Operation {
    fn from(input: &str) -> Operation {
        let spl: Vec<&str> = input.split(" ").collect();
        let count = spl[1].parse().unwrap();
        let from = spl[3].parse().unwrap();
        let to = spl[5].parse().unwrap();

        return Operation { count, from, to };
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<Vec<Container>>, Vec<Operation>) {
    let mut field_result: Vec<Vec<Container>> = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();
    let (field, operations_data) = input.split_once("\n\n").unwrap();
    let mut field_lines = field.lines().collect::<Vec<&str>>();
    let last_str = field_lines.last().unwrap().trim();
    field_lines.reverse();
    for column in last_str.split_whitespace() {
        let n: usize = str::parse::<usize>(column).unwrap() - 1;
        field_result.push(Vec::new());
        for line in field_lines.iter().skip(1) {
            let stack = field_result.get_mut(n as usize).unwrap();

            unsafe {
                let block = line.get_unchecked(n * 4..n * 4 + 3);
                if !block.contains(' ') {
                    stack.push(Container::from(block));
                }
            }
        }
    }

    for line in operations_data.lines() {
        operations.push(Operation::from(line));
    }

    return (field_result, operations);
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<Vec<Container>>, Vec<Operation>)) -> String {
    let field = &mut input.0.clone();
    let operations = &input.1;

    for op in operations {
        for _ in 0..op.count {
            let con = &field.get_mut(op.from - 1).unwrap().pop().unwrap();
            field.get_mut(op.to - 1).unwrap().push(*con);
        }
    }
    let mut result = String::new();
    for column in field {
        result.push(column.last().unwrap().0);
    }
    return result;
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<Vec<Container>>, Vec<Operation>)) -> String {
    let field = &mut input.0.clone();
    let operations = &input.1;

    for op in operations {
        let i = field.get(op.from -1).unwrap().len() - op.count;
        let mut drain = field
            .get_mut(op.from - 1)
            .unwrap()
            .drain(i..)
            .collect::<Vec<Container>>();
        field.get_mut(op.to - 1).unwrap().append(&mut drain);
    }
    let mut result = String::new();
    for column in field {
        result.push(column.last().unwrap().0);
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 \n\nmove 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!("CMZ", part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 \n\nmove 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!("MCD", part2(&input_generator(input)));
    }
}
