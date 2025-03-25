#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input
        .chars()
        .map(|c| if c == '(' { 1 as i8 } else { -1 as i8 })
        .sum::<i8>() as u32
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut floor = 1;
    for i in 0..input.len() {
        if input.as_bytes()[i] as char == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 {
            return i as u32;
        }
    }
    0
}
