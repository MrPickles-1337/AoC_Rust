use std::cmp::Ordering;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let mut split = input.split("\n\n");
    let ordering = split
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut split = l.split('|');
            (
                split.next().unwrap().parse::<u8>().unwrap(),
                split.next().unwrap().parse::<u8>().unwrap(),
            )
        })
        .collect();
    let updates = split
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(|n| n.parse::<u8>().unwrap()).collect())
        .collect();
    (ordering, updates)
}

#[aoc(day5, part1)]
pub fn part1((ordering, updates): &(Vec<(u8, u8)>, Vec<Vec<u8>>)) -> u32 {
    updates
        .iter()
        .filter(|update| {
            let mut valid = true;
            for i in ordering {
                if let Some(a) = update.iter().position(|&n| n == i.0) {
                    if let Some(b) = update.iter().position(|&n| n == i.1) {
                        if a > b {
                            valid = false;
                            break;
                        }
                    }
                }
            }
            valid
        })
        .map(|update| update[update.len() / 2] as u32)
        .sum()
}

#[aoc(day5, part2)]
pub fn part2((ordering, updates): &(Vec<(u8, u8)>, Vec<Vec<u8>>)) -> u32 {
    let updates = updates.clone();
    updates
        .into_iter()
        .filter(|update| !update.is_sorted_by(|a, b| !ordering.contains(&(*b, *a))))
        .map(|mut update| {
            update.sort_by(|a, b| {
                if ordering.contains(&(*a, *b)) {
                    Ordering::Greater
                } else if ordering.contains(&(*b, *a)) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            });
            update
        })
        .map(|update| update[update.len() / 2] as u32)
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(143, part1(&input_generator(input)));
    }
    #[test]
    fn part2_test() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(123, part2(&input_generator(input)));
    }
}
