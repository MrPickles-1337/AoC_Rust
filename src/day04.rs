// i don't know what is this i stole it :(
struct _2DMask {
    relative_pos: Vec<isize>,
    left_offset: isize,
    right_offset: isize,
}

impl _2DMask {
    pub const fn new(relative_pos: Vec<isize>, left_offset: isize, right_offset: isize) -> Self {
        Self {
            relative_pos,
            left_offset,
            right_offset,
        }
    }

    pub fn apply<'a, T>(&self, pos: isize, width: isize, vals: &'a [T]) -> Option<Vec<&'a T>> {
        if pos % width < self.left_offset || (pos % width) + self.right_offset >= width {
            return None;
        }
        self.relative_pos
            .iter()
            .map(|&i| vals.get((i + pos) as usize))
            .collect()
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let width = input.find("\n").unwrap() as isize;
    let input = input.replace("\n", "").chars().collect::<Vec<char>>();
    let input_slice = input.as_slice();
    let masks = [
        _2DMask::new(vec![0, 1, 2, 3], 0, 3),
        _2DMask::new(vec![0, width + 1, 2 * (width + 1), 3 * (width + 1)], 0, 3),
        _2DMask::new(vec![0, width, 2 * width, 3 * width], 0, 0),
        _2DMask::new(vec![0, width - 1, 2 * (width - 1), 3 * (width - 1)], 3, 0),
        _2DMask::new(vec![0, -1, -2, -3], 3, 0),
        _2DMask::new(
            vec![0, -width - 1, 2 * (-width - 1), 3 * (-width - 1)],
            3,
            0,
        ),
        _2DMask::new(vec![0, -width, -2 * width, -3 * width], 0, 0),
        _2DMask::new(vec![0, -width + 1, -2 * width + 2, -3 * width + 3], 0, 3),
    ];
    (0..input.len() as isize)
        .flat_map(|pos| {
            masks
                .iter()
                .filter_map(move |mask| mask.apply(pos, width, input_slice))
        })
        .filter(|vals| {
            vals.len() == 4
                && *vals[0] == 'X'
                && *vals[1] == 'M'
                && *vals[2] == 'A'
                && *vals[3] == 'S'
        })
        .count() as u32
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
    let width = input.find("\n").unwrap() as isize;
    let input = input.replace("\n", "").chars().collect::<Vec<char>>();
    let input_slice = input.as_slice();
    let masks = [
        _2DMask::new(vec![0, 2, width + 1, 2 * width + 2, 2 * width], 0, 2),
        _2DMask::new(vec![2 * width, 2, width + 1, 0, 2 * width + 2], 0, 2),
        _2DMask::new(vec![2, 2 * width, width + 1, 0, 2 * width + 2], 0, 2),
        _2DMask::new(vec![0, 2 * width, width + 1, 2 * width + 2, 2], 0, 2),
    ];
    (0..input.len() as isize)
        .flat_map(|pos| {
            masks
                .iter()
                .filter_map(move |mask| mask.apply(pos, width, input_slice))
        })
        .filter(|vals| {
            vals.len() == 5
                && *vals[0] == 'M'
                && *vals[1] == 'S'
                && *vals[2] == 'A'
                && *vals[3] == 'S'
                && *vals[4] == 'M'
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        assert_eq!(18, part1(input));
    }

    #[test]
    fn part2_test() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";
        assert_eq!(9, part2(input));
    }
}
