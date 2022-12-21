pub enum Instruction {
    Addx(isize),
    Noop,
}

#[aoc_generator(day10)]
pub fn input_generatr(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let spl = line.split(" ").collect::<Vec<&str>>();
            let instr = match spl.first().unwrap() {
                &"addx" => Instruction::Addx(str::parse::<isize>(spl.last().unwrap()).unwrap()),
                &"noop" => Instruction::Noop,
                _ => unreachable!(),
            };
            return instr;
        })
        .collect::<Vec<Instruction>>()
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<Instruction>) -> isize {
    let mut x: isize = 1;
    let mut cycle = 0;
    let mut res = 0;
    for instr in input {
        match instr {
            Instruction::Addx(n) => {
                cycle += 2;
                let cycle = if cycle % 10 == 0 { cycle } else { cycle - 1 };
                match cycle {
                    20 | 60 | 100 | 140 | 180 | 220 => {
                        res += cycle * x;
                    }
                    _ => (),
                }
                x += n;
            }
            Instruction::Noop => {
                cycle += 1;
                match cycle {
                    20 | 60 | 100 | 140 | 180 | 220 => {
                        res += cycle * x;
                    }
                    _ => (),
                }
            }
        }
    }
    return res;
}

pub fn part2(input: &Vec<Instruction>) -> usize {
    let mut screen = vec![false; 6*40];
    let mut position = 0;

    for instr in input {
        match instr {
            Instruction::Addx(n) => {
                todo!();
            },
            Instruction::Noop => todo!(),
        }
    }

    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(13140, part1(&input_generatr(input)));
    }

    #[test]
    fn part2_test() {}
}
