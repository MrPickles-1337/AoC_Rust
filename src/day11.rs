#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(' ').map(|i| i.parse().unwrap()).collect()
}

fn process(a: &mut Vec<usize>, b: &mut Vec<usize>) {
    while let Some(i) = a.pop() {
        if i == 0 {
            b.push(1);
        } else {
            let digits = i.ilog10() + 1;
            if digits % 2 == 0 {
                let half_digits = digits / 2;
                let yep = 10usize.pow(half_digits);
                b.push(i / yep);
                b.push(i % yep);
            } else {
                b.push(i * 2024);
            }
        }
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut a = Vec::from(input);
    let mut b = Vec::new();
    for _ in 0..25 {
        process(&mut a, &mut b);
        std::mem::swap(&mut a, &mut b);
    }
    a.len() + b.len()
}

#[aoc(day11, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut a = Vec::from(input);
    let mut b = Vec::new();
    for _ in 0..75 {
        process(&mut a, &mut b);
        std::mem::swap(&mut a, &mut b);
    }
    a.len() + b.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "125 17";

        assert_eq!(55312, part1(&input_generator(input)));
    }
}
