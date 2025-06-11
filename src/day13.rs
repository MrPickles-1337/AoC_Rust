#[derive(Debug)]
pub struct Machine {
    a_button: (u32, u32),
    b_button: (u32, u32),
    prize: (u32, u32),
}

fn parse_xy(line: &str) -> (u32, u32) {
    let mut spl = line.split(',');
    let x = spl
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |mut acc, c| {
            let i = c.to_digit(10).unwrap();
            acc *= 10;
            acc + i
        });
    let y = spl
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .fold(0, |mut acc, c| {
            let i = c.to_digit(10).unwrap();
            acc *= 10;
            acc + i
        });
    (x, y)
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|m| {
            let mut split = m.split("\n");
            let a = parse_xy(split.next().unwrap());
            let b = parse_xy(split.next().unwrap());
            let prize = parse_xy(split.next().unwrap());
            Machine {
                a_button: a,
                b_button: b,
                prize,
            }
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &[Machine]) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        assert_eq!(480, part1(&input_generator(input)));
    }
}
