use std::{cell::RefCell, rc::Rc};

type TileType = Option<Rc<RefCell<Box<Tile>>>>;

#[derive(Debug, PartialEq)]
struct Tile {
    pub top: TileType,
    pub right: TileType,
    pub bottom: TileType,
    pub left: TileType,
    pub _height: u8,
}

impl Tile {
    fn new(height: u8) -> TileType {
        Some(Rc::new(RefCell::new(Box::new(Self {
            top: None,
            right: None,
            bottom: None,
            left: None,
            _height: height,
        }))))
    }
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> TileType {
    let mut start_tile: TileType = None;
    let mut last_row: Option<Vec<TileType>> = None;
    input
        .lines()
        .map(|line| {
            let mut row: Vec<TileType> = Vec::new();
            line.chars()
                .enumerate()
                .map(|(i, c)| {
                    println!("{c}");
                    if row.is_empty() {
                        let tile = Tile::new(c as u8).unwrap();
                        if c == 'S' {
                            start_tile = Some(tile.clone());
                        }
                        row.push(Some(tile));
                        return;
                    }
                    let last_tile = row.last_mut().unwrap().as_mut().unwrap();
                    let tile = Tile::new(c as u8).unwrap();
                    tile.borrow_mut().left = Some(last_tile.clone());
                    last_tile.borrow_mut().right = Some(tile.clone());
                    if let Some(lr) = last_row.as_ref() {
                        let tile_from_last_row = lr.get(i).unwrap().as_ref().unwrap();
                        tile_from_last_row.borrow_mut().bottom = Some(tile.clone());
                        tile.borrow_mut().top = Some(tile_from_last_row.clone());
                    }
                    if c == 'S' {
                        start_tile = Some(tile.clone());
                    }
                    row.push(Some(tile));
                })
                .for_each(drop);
            last_row = Some(row);
        })
        .for_each(drop);
    start_tile
}

#[aoc(day12, part1)]
fn part1(input: &TileType) -> u8 {
    dbg!(input);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "";
        assert_eq!(part1(&input_generator(input)), 1);
    }
}
