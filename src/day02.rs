pub struct Gift {
    l: u32,
    w: u32,
    h: u32,
}

#[aoc_generator(day2)]
pub fn data_generator(input: &str) -> Vec<Gift> {
    input
        .lines()
        .map(|l| {
            let mut gift = l.trim().split('x').map(|d| d.parse().unwrap());
            Gift {
                l: gift.next().unwrap(),
                w: gift.next().unwrap(),
                h: gift.next().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Gift]) -> u32 {
    input
        .iter()
        .map(|gift| {
            let (s1, s2) = smallest_side(gift);
            2 * gift.l * gift.w + 2 * gift.w * gift.h + 2 * gift.h * gift.l + s1 * s2
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Gift]) -> u32 {
    input
        .iter()
        .map(|gift| {
            let (s1, s2) = smallest_side(gift);
            s1 + s1 + s2 + s2 + gift.l * gift.h * gift.w
        })
        .sum()
}

fn smallest_side(gift: &Gift) -> (u32, u32) {
    let mut vec = [gift.l, gift.w, gift.h];
    vec.sort();

    (vec[0], vec[1])
}
