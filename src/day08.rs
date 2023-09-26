#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn visible_from_top(field: &[Vec<u32>], x: usize, y: usize, height: &u32) -> bool {
    let mut result = true;

    for i in field.iter().take(y) {
        if i[x] >= *height {
            result = false;
        }
    }

    result
}

fn visible_from_right(field: &[Vec<u32>], x: usize, y: usize, height: &u32) -> bool {
    let mut result = true;

    for i in x + 1..field[0].len() {
        if field[y][i] >= *height {
            result = false;
        }
    }

    result
}

fn visible_from_bottom(field: &[Vec<u32>], x: usize, y: usize, height: &u32) -> bool {
    let mut result = true;

    for i in field.iter().skip(y + 1) {
        if i[x] >= *height {
            result = false;
        }
    }

    result
}

fn visible_from_left(field: &[Vec<u32>], x: usize, y: usize, height: &u32) -> bool {
    let mut result = true;

    for i in 0..x {
        if field[y][i] >= *height {
            result = false;
        }
    }

    result
}

#[aoc(day8, part1)]
pub fn part1(input: &Vec<Vec<u32>>) -> usize {
    let mut visible = input.len() * 2 + (input[0].len() - 2) * 2;
    for i in 1..input.len() - 1 {
        for (j, tree) in input[i][1..input[i].len() - 1].iter().enumerate() {
            let j = j + 1;
            if visible_from_top(input, j, i, tree)
                || visible_from_right(input, j, i, tree)
                || visible_from_bottom(input, j, i, tree)
                || visible_from_left(input, j, i, tree)
            {
                visible += 1;
            }
        }
    }

    visible
}

fn calculate_scenic_score(field: &[Vec<u32>], x: usize, y: usize) -> u32 {
    let height = field[y][x];
    let mut top = 0;
    for i in (0..y).rev() {
        top += 1;
        if field[i][x] >= height {
            break;
        }
    }

    let mut right = 0;
    for i in x + 1..field[0].len() {
        right += 1;
        if field[y][i] >= height {
            break;
        }
    }

    let mut bottom = 0;
    for i in field.iter().skip(y + 1) {
        bottom += 1;
        if i[x] >= height {
            break;
        }
    }

    let mut left = 0;
    for i in (0..x).rev() {
        left += 1;
        if field[y][i] >= height {
            break;
        }
    }

    left * right * top * bottom
}

#[aoc(day8, part2)]
pub fn part2(input: &Vec<Vec<u32>>) -> usize {
    let mut result = 0;
    for i in 1..input.len() - 1 {
        for j in 1..input[0].len() - 1 {
            let score = calculate_scenic_score(input, j, i);
            if score > result {
                result = score;
            }
        }
    }

    result as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn part2_test() {
        let input = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(part2(&input), 8);
    }
}
