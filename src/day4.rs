#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let key = String::from(input);
    let mut i = 0;
    loop {
        let v = format!("{}{}", key, i);
        let digest = format!("{:x}", md5::compute(v));
        if &digest[..5] == "00000" {
            return i;
        }
        i += 1;
    }
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let key = String::from(input);
    let mut i = 0;
    loop {
        let v = format!("{}{}", key, i);
        let digest = format!("{:x}", md5::compute(v));
        if &digest[..6] == "000000" {
            return i;
        }
        i += 1;
    }
}
