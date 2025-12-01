#[derive(Debug)]
pub enum Turn {
    L(i32),
    R(i32),
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Turn> {
    input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let direction = chars.next().unwrap();
            let distance = chars.collect::<String>().parse::<i32>().unwrap();
            match direction {
                'L' => Turn::L(distance),
                'R' => Turn::R(distance),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn turn_p1(dial: &mut i32, turn: &Turn) {
    match turn {
        Turn::L(distance) => {
            *dial -= distance;
        }
        Turn::R(distance) => {
            *dial += distance;
        }
    }
    *dial = (*dial % 100 + 100) % 100;
}

fn turn_p2(dial: &mut i32, turn: &Turn) -> i32 {
    let mut result = 0;
    match turn {
        Turn::L(value) => {
            let t = if *dial == 0 { 100 } else { *dial };
            if *value >= t {
                result += 1 + (value - t) / 100;
            }
            *dial = ((*dial - value) % 100 + 100) % 100;
        }
        Turn::R(value) => {
            let diff = *dial + value;
            result += diff / 100;
            *dial = (diff % 100 + 100) % 100;
        }
    }
    result
}

#[aoc(day1, part1)]
pub fn part1(input: &[Turn]) -> u32 {
    input
        .iter()
        .fold((50, 0), |(mut pos, mut result), turn| {
            turn_p1(&mut pos, turn);
            if pos == 0 {
                result += 1;
            }
            (pos, result)
        })
        .1
}

#[aoc(day1, part2)]
pub fn part2(input: &[Turn]) -> u32 {
    input
        .iter()
        .fold((50, 0), |(mut pos, mut result), turn| {
            result += turn_p2(&mut pos, turn);
            (pos, result)
        })
        .1 as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(3, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(6, part2(&input_generator(input)));
    }
}
