#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

enum Direction {
    Initial,
    Left,
    Right,
    Up,
    Down,
}

fn move_next(
    position: &mut (usize, usize),
    direction: &mut Direction,
    pipe: &mut char,
    map: &Vec<Vec<char>>,
) {
}

#[aoc(day10, part1)]
pub fn part1(input: &Vec<Vec<char>>) -> u32 {
    let start;
    'a: for (i, l) in input.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            match c {
                'S' => {
                    start = (i, j);
                    break 'a;
                }
                _ => {}
            }
        }
    }
    let mut dir1 = Direction::Initial;
    let mut dir2 = Direction::Initial;
    let mut pipe1 = &'S';
    let mut pipe2 = &'S';
    let mut pos1 = start.clone();
    let mut pos2 = start;

    loop {}
}
