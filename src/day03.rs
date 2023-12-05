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
            } else {
                if save {
                    let len = number.to_string().len();
                    parts.push(Part {
                        start: ((x - len) as u32, y as u32),
                        lenght: len as u32,
                        value: number,
                    });
                    save = false;
                    number = 0;
                }
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
    match c {
        '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => false,
        _ => true,
    }
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
                .expect(format!("expected row at {y}").as_str());
            let c = row
                .get(x as usize)
                .expect(format!("expected char at {x}").as_str());
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
                .expect(format!("expected row at {y}").as_str());
            let c = row
                .get(x as usize)
                .expect(format!("expected char at {x}").as_str());
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
                    .expect(format!("expected row at {y}").as_str());
                let c = row
                    .get(x as usize)
                    .expect(format!("expected char at {x}").as_str());
                if check(c) {
                    is_part = true;
                    break 'a;
                }
            }
            let y = part.start.1 + 1;
            if (y as usize) < map.len() {
                let row = map
                    .get(y as usize)
                    .expect(format!("expected row at {y}").as_str());
                let c = row
                    .get(x as usize)
                    .expect(format!("expected char at {x}").as_str());
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

#[cfg(test)]
mod tests {
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
}
