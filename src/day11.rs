use std::vec;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Tile {
    Space,
    Galaxy,
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<Vec<Tile>> {
    let mut image: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Space,
                    '#' => Tile::Galaxy,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let width = image.first().unwrap().len();

    let mut insert_row_at = Vec::new();
    let mut insert_column_at = Vec::new();

    for (i, t) in image.iter().enumerate() {
        if t.contains(&Tile::Galaxy) {
            continue;
        }
        insert_row_at.push(i);
    }

    'a: for i in 0..width {
        for j in image.iter() {
            if j[i] == Tile::Galaxy {
                continue 'a;
            }
        }
        insert_column_at.push(i);
    }

    for i in insert_row_at {
        image.insert(i, vec![Tile::Space; width]);
    }
    for i in insert_column_at {
        for row in image.iter_mut() {
            row.insert(i, Tile::Space);
        }
    }

    image
}

#[aoc(day11, part1)]
fn part1(input: &Vec<Vec<Tile>>) -> u32 {
    let mut sum = 0;
    let mut galaxies = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for j in 0..row.len() {
            if let Tile::Space = input[i][j] {
                continue;
            }
            println!("got galaxy");
            if galaxies.len() == 0 {
                galaxies.push((i, j));
                continue;
            }

            for g in galaxies.iter() {
                let yep = (i as isize - g.0 as isize).abs() + (j as isize - g.1 as isize).abs() - 1;
                println!("{} - {} + {} - {} = {}", i, g.0, j, g.1, yep);
                sum += yep;
            }
            galaxies.push((i, j));
        }
    }
    sum as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(374, part1(&input_generator(input)));
    }
}
