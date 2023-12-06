#[derive(Debug)]
pub enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Cube {
    pub(crate) fn add(self, value: u32) -> Self {
        match self {
            Cube::Red(i) => Cube::Red(i + value),
            Cube::Green(i) => Cube::Green(i + value),
            Cube::Blue(i) => Cube::Blue(i + value),
        }
    }

    pub fn value(&self) -> &u32 {
        match self {
            Cube::Red(i) => i,
            Cube::Green(i) => i,
            Cube::Blue(i) => i,
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<(Cube, Cube, Cube)>> {
    input
        .lines()
        .map(|l| {
            let pulls = l.split(": ").nth(1).unwrap();
            pulls
                .split("; ")
                .map(|p| {
                    let mut red = Cube::Red(0);
                    let mut green = Cube::Green(0);
                    let mut blue = Cube::Blue(0);
                    let split = p.split(", ");
                    for cube in split {
                        let mut split = cube.split(' ');
                        let (c, color) = (split.next().unwrap(), split.next().unwrap());
                        match color {
                            "red" => red = red.add(c.parse().unwrap()),
                            "green" => green = green.add(c.parse().unwrap()),
                            "blue" => blue = blue.add(c.parse().unwrap()),
                            _ => unreachable!(),
                        }
                    }
                    (red, green, blue)
                })
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<(Cube, Cube, Cube)>]) -> usize {
    let mut result = 0;

    for (i, game) in input.iter().enumerate() {
        let mut possible = true;
        for pull in game {
            if pull.0.value() > &12 {
                possible = false;
                break;
            }
            if pull.1.value() > &13 {
                possible = false;
                break;
            }
            if pull.2.value() > &14 {
                possible = false;
                break;
            }
        }
        if possible {
            result += i + 1;
        }
    }

    result
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<(Cube, Cube, Cube)>]) -> u32 {
    let mut result = 0;
    for game in input {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for pull in game {
            let pulled_red = *pull.0.value();
            let pulled_green = *pull.1.value();
            let pulled_blue = *pull.2.value();
            if pulled_red > red {
                red = pulled_red;
            }
            if pulled_green > green {
                green = pulled_green;
            }
            if pulled_blue > blue {
                blue = pulled_blue;
            }
        }
        result += red * green * blue;
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, part2(&input_generator(input)));
    }

    #[cfg(feature = "full_input")]
    #[test]
    fn part1_with_input_test() {
        let input = fs::read_to_string("input/2023/day2.txt").unwrap();
        assert_eq!(2101, part1(&input_generator(input.as_str())));
    }

    #[cfg(feature = "full_input")]
    #[test]
    fn part2_with_input_test() {
        let input = fs::read_to_string("input/2023/day2.txt").unwrap();
        assert_eq!(58269, part2(&input_generator(input.as_str())));
    }
}
