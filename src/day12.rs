use std::{cell::RefCell, rc::Rc};

type TileType = Option<Rc<RefCell<Box<Tile>>>>;

#[derive(Debug, PartialEq)]
struct Tile {
    pub top: TileType,
    pub right: TileType,
    pub bottom: TileType,
    pub left: TileType,
    pub height: u16,
    pub closed: bool,
}

impl Tile {
    fn new(height: u16) -> TileType {
        Some(Rc::new(RefCell::new(Box::new(Self {
            top: None,
            right: None,
            bottom: None,
            left: None,
            height,
            closed: false,
        }))))
    }

    fn can_go_right(&self) -> bool {
        if let Some(right) = self.right.as_ref() {
            return self.height >= right.borrow().height - 1 && !self.closed;
        } else {
            println!("we are on edge");
            false
        }
    }

    fn can_go_bottom(&self) -> bool {
        if let Some(bottom) = self.bottom.as_ref() {
            return self.height >= bottom.borrow().height - 1 && !self.closed;
        } else {
            println!("we are on edge");
            false
        }
    }

    fn can_go_left(&self) -> bool {
        if let Some(left) = self.left.as_ref() {
            return self.height >= left.borrow().height - 1 && !self.closed;
        } else {
            println!("we are on edge");
            false
        }
    }

    fn can_go_top(&self) -> bool {
        if let Some(top) = self.top.as_ref() {
            return self.height >= top.borrow().height - 1 && !self.closed;
        } else {
            println!("we are on edge");
            false
        }
    }

    pub fn shortest_path(&mut self) -> u32 {
        self.closed = true;
        if self.height == 'E' as u16 {
            println!("found E");
            return 1;
        }
        let top = if self.can_go_top() {
            println!("going top");
            self.top.as_ref().unwrap().borrow_mut().shortest_path()
        } else {
            0
        };
        let right = if self.can_go_right() {
            println!("going right");
            self.right.as_ref().unwrap().borrow_mut().shortest_path()
        } else {
            0
        };
        let bottom = if self.can_go_bottom() {
            println!("going bottom");
            self.bottom.as_ref().unwrap().borrow_mut().shortest_path()
        } else {
            0
        };
        let left = if self.can_go_left() {
            println!("going left");
            self.left.as_ref().unwrap().borrow_mut().shortest_path()
        } else {
            0
        };

        let mut yep = Vec::new();
        if top > 0 {
            yep.push(top);
        }
        if right > 0 {
            yep.push(right);
        }
        if bottom > 0 {
            yep.push(bottom);
        }
        if left > 0 {
            yep.push(left);
        }
        yep.sort();
        dbg!(yep);
        1
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
                    if row.is_empty() {
                        let tile = Tile::new(c as u16).unwrap();
                        if c == 'S' {
                            start_tile = Some(tile.clone());
                        }
                        row.push(Some(tile));
                        return;
                    }
                    let last_tile = row.last_mut().unwrap().as_mut().unwrap();
                    let tile = Tile::new(c as u16).unwrap();
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
    start_tile.as_ref().unwrap().borrow_mut().closed = true;
    start_tile
}

#[aoc(day12, part1)]
fn part1(input: &TileType) -> u32 {
    let yep = input.as_ref().unwrap().borrow_mut();
    println!(
        "{},{},{},{}",
        yep.top.is_some(),
        yep.right.is_some(),
        yep.bottom.is_some(),
        yep.left.is_some()
    );
    // input.as_ref().unwrap().borrow_mut().shortest_path()
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(part1(&input_generator(input)), 31);
    }
}
