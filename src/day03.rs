use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Part {
    pub start: (u32, u32),
    pub lenght: u32,
    pub value: u32,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (Vec<Part>, String) {
    let mut parts = vec![];
    input.lines().enumerate().for_each(|(y, l)| {
        let chars = l.chars();
        let mut number = 0;
        let mut save = false;
        for (x, c) in chars.enumerate() {
            if let Some(c) = c.to_digit(10) {
                save = true;
                number = number * 10 + c;
            } else if save {
                let len = number.to_string().len();
                parts.push(Part {
                    start: ((x - len) as u32, y as u32),
                    lenght: len as u32,
                    value: number,
                });
                save = false;
                number = 0;
            }
            if save && x + 1 == l.len() {
                let len = number.to_string().len();
                parts.push(Part {
                    start: ((x - len + 1) as u32, y as u32),
                    lenght: len as u32,
                    value: number,
                });
                save = false;
                number = 0;
            }
        }
    });
    (parts, input.to_string())
}

fn check(c: &char) -> bool {
    matches!(
        c,
        '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    )
}

#[aoc(day3, part1)]
pub fn part1(input: &(Vec<Part>, String)) -> u32 {
    let map: Vec<Vec<char>> = input.1.lines().map(|l| l.chars().collect()).collect();
    let mut result = 0;
    for part in input.0.clone() {
        if part.start.0 > 0 {
            let y = part.start.1;
            let x = part.start.0 - 1;
            let row = map
                .get(y as usize)
                .unwrap_or_else(|| panic!("{}", "expected row at {y}"));
            let c = row
                .get(x as usize)
                .unwrap_or_else(|| panic!("{}", "expected char at {x}"));
            if check(c) {
                result += part.value;
                continue;
            }
        }
        if part.start.0 + part.lenght < map.first().unwrap().len() as u32 {
            let x = part.start.0 + part.lenght;
            let y = part.start.1;
            let row = map
                .get(y as usize)
                .unwrap_or_else(|| panic!("{}", "expected row at {y}"));
            let c = row
                .get(x as usize)
                .unwrap_or_else(|| panic!("{}", "expected char at {x}"));
            if check(c) {
                result += part.value;
                continue;
            }
        }
        let mut is_part = false;
        'a: for x in part.start.0 as i32 - 1..(part.start.0 + part.lenght) as i32 + 1 {
            if x < 0 || x >= map.first().unwrap().len() as i32 {
                continue;
            }
            let y = part.start.1 as i32 - 1;
            if y >= 0 {
                let row = map
                    .get(y as usize)
                    .unwrap_or_else(|| panic!("{}", "expected row at {y}"));
                let c = row
                    .get(x as usize)
                    .unwrap_or_else(|| panic!("{}", "expected char at {x}"));
                if check(c) {
                    is_part = true;
                    break 'a;
                }
            }
            let y = part.start.1 + 1;
            if (y as usize) < map.len() {
                let row = map
                    .get(y as usize)
                    .unwrap_or_else(|| panic!("{}", "expected row at {y}"));
                let c = row
                    .get(x as usize)
                    .unwrap_or_else(|| panic!("{}", "expected char at {x}"));
                if check(c) {
                    is_part = true;
                    break 'a;
                }
            }
        }
        if is_part {
            result += part.value;
        }
    }
    result
}

fn check_2(c: &char) -> bool {
    matches!(c, '*')
}

#[aoc(day3, part2)]
pub fn part2(input: &(Vec<Part>, String)) -> u32 {
    let map: Vec<Vec<char>> = input.1.lines().map(|l| l.chars().collect()).collect();
    let mut geared: HashMap<(u32, u32), u32> = HashMap::new();
    let mut result = 0;
    for part in input.0.clone() {
        if part.start.0 > 0 {
            let y = part.start.1;
            let x = part.start.0 - 1;
            let row = map
                .get(y as usize)
                .unwrap_or_else(|| panic!("{}", "expected row at {y}"));
            let c = row
                .get(x as usize)
                .unwrap_or_else(|| panic!("{}", "expected char at {x}"));
            if check_2(c) {
                if let Some(a) = geared.get(&(x, y)) {
                    result += part.value * a;
                } else {
                    geared.insert((x, y), part.value);
                }
                continue;
            }
        }
        if part.start.0 + part.lenght < map.first().unwrap().len() as u32 {
            let x = part.start.0 + part.lenght;
            let y = part.start.1;
            let row = map
                .get(y as usize)
                .unwrap_or_else(|| panic!("{}", "expected row at {y}"));
            let c = row
                .get(x as usize)
                .unwrap_or_else(|| panic!("{}", "expected char at {x}"));
            if check_2(c) {
                if let Some(a) = geared.get(&(x, y)) {
                    result += part.value * a;
                } else {
                    geared.insert((x, y), part.value);
                }
                continue;
            }
        }
        'a: for x in part.start.0 as i32 - 1..(part.start.0 + part.lenght) as i32 + 1 {
            if x < 0 || x >= map.first().unwrap().len() as i32 {
                continue;
            }
            let y = part.start.1 as i32 - 1;
            if y >= 0 {
                let row = map
                    .get(y as usize)
                    .unwrap_or_else(|| panic!("{}", "expected row at {y}"));
                let c = row
                    .get(x as usize)
                    .unwrap_or_else(|| panic!("{}", "expected char at {x}"));
                if check_2(c) {
                    let x = x as u32;
                    let y = y as u32;
                    if let Some(a) = geared.get(&(x, y)) {
                        result += part.value * a;
                    } else {
                        geared.insert((x, y), part.value);
                    }
                    break 'a;
                }
            }
            let y = part.start.1 + 1;
            if (y as usize) < map.len() {
                let row = map
                    .get(y as usize)
                    .unwrap_or_else(|| panic!("{}", "expected row at {y}"));
                let c = row
                    .get(x as usize)
                    .unwrap_or_else(|| panic!("{}", "expected char at {x}"));
                if check(c) {
                    let x = x as u32;
                    if let Some(a) = geared.get(&(x, y)) {
                        result += part.value * a;
                    } else {
                        geared.insert((x, y), part.value);
                    }
                    break 'a;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "full_input")]
    use std::fs;

    use super::*;

    #[test]
    fn part1_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(4361, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(467835, part2(&input_generator(input)));
    }

    #[cfg(feature = "full_input")]
    #[test]
    fn part1_with_input_test() {
        let input = fs::read_to_string("input/2023/day3.txt").unwrap();
        assert_eq!(521515, part1(&input_generator(input.as_str())));
    }

    #[cfg(feature = "full_input")]
    #[test]
    fn part2_with_input_test() {
        let input = fs::read_to_string("input/2023/day3.txt").unwrap();
        assert_eq!(69527306, part2(&input_generator(input.as_str())));
    }
}
