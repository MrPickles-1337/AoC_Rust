#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[String]) -> u32 {
    let yep = input
        .iter()
        .map(|s| {
            let mut chars = s.chars();
            let mut code = 0;
            let mut text = 0;

            while let Some(c) = chars.next() {
                code += 1;
                if c == '\\' {
                    let next = chars.next().unwrap();
                    if next == 'x' {
                        chars.next().unwrap();
                        chars.next().unwrap();
                        code += 3;
                        text += 1;
                    } else {
                        code += 1;
                        text += 1;
                    }
                } else {
                    text += 1;
                }
            }
            (code, text)
        })
        .fold((0, 0), |(a, b), (next_a, next_b)| {
            (a + next_a, b + (next_b - 2))
        });

    yep.0 - yep.1
}

#[aoc(day8, part2)]
pub fn part2(input: &[String]) -> usize {
    let (a, b) = input
        .iter()
        .map(|s| {
            let mut new_str = s.chars().fold(String::from('"'), |mut new_s, c| {
                if c == '\\' {
                    new_s.push('\\');
                    new_s.push('\\');
                } else if c == '"' {
                    new_s.push('\\');
                    new_s.push('"');
                } else {
                    new_s.push(c);
                }
                new_s
            });
            new_str.push('"');
            new_str
        })
        .zip(input)
        .fold((0, 0), |(a, b), (next_a, next_b)| {
            (a + next_a.len(), b + next_b.len())
        });
    a - b
}
