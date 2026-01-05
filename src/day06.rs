#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let ops = lines.last().unwrap();
    lines
        .iter()
        .take(lines.len() - 1)
        .fold(vec![0; ops.len()], |mut result, line| {
            for (i, n) in line.iter().enumerate() {
                let operation = ops.get(i).unwrap();
                match *operation {
                    "*" => {
                        let a = result.get_mut(i).unwrap();
                        if *a == 0 {
                            *a = 1;
                        }
                        *a *= n.parse::<usize>().unwrap();
                    }
                    "+" => *result.get_mut(i).unwrap() += n.parse::<usize>().unwrap(),
                    _ => unreachable!(),
                }
            }
            result
        })
        .iter()
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let ops = lines.last().unwrap().split_whitespace();
    // lines.last().unwrap().chars().rev().fold(init, f)
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";
        assert_eq!(4277556, part1(input));
    }

    #[test]
    fn part2_test() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";
        assert_eq!(4277556, part2(input));
    }
}
