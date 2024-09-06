#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|yep| yep.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|history| {
            let mut yep: Vec<Vec<i64>> = Vec::new();
            yep.push(history.clone());
            while yep.last().unwrap().iter().sum::<i64>() != 0 {
                let mut new: Vec<i64> = Vec::new();
                for i in 0..yep.last().unwrap().len() - 1 {
                    let last = yep.last().unwrap();
                    new.push(last[i + 1] - last[i]);
                }
                yep.push(new);
            }
            let mut next = 0;
            let mut itr = yep.iter();
            while let Some(a) = itr.next_back() {
                next += a.last().unwrap();
            }
            next
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|history| {
            let mut yep: Vec<Vec<i64>> = Vec::new();
            yep.push(history.clone());
            while yep.last().unwrap().iter().sum::<i64>() != 0 {
                let mut new: Vec<i64> = Vec::new();
                for i in 0..yep.last().unwrap().len() - 1 {
                    let last = yep.last().unwrap();
                    new.push(last[i + 1] - last[i]);
                }
                yep.push(new);
            }
            let mut new = 0;
            let mut itr = yep.iter();
            while let Some(a) = itr.next_back() {
                new = a.first().unwrap() - new;
            }
            new
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114, part1(&input_generator(input)));
    }

    #[test]
    fn part2_test() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, part2(&input_generator(input)));
    }
}
