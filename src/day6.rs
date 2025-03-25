#[derive(Debug)]
pub enum Command {
    TurnOn((u32, u32), (u32, u32)),
    TurnOff((u32, u32), (u32, u32)),
    Toggle((u32, u32), (u32, u32)),
}

fn get_coordinates(input: &str) -> (u32, u32) {
    let vec: Vec<u32> = input
        .split(",")
        .map(|e| e.parse::<u32>().unwrap())
        .collect();
    (vec[0], vec[1])
}
#[aoc_generator(day6)]
pub fn data_generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut splitted = line.split(" ");
            let command = splitted.next().unwrap();
            if command == "turn" {
                let on = splitted.next().unwrap();
                let start = get_coordinates(splitted.next().unwrap());
                let end = get_coordinates(splitted.last().unwrap());

                if on == "on" {
                    Command::TurnOn(start, end)
                } else {
                    Command::TurnOff(start, end)
                }
            } else {
                let start = get_coordinates(splitted.next().unwrap());
                let end = get_coordinates(splitted.last().unwrap());

                Command::Toggle(start, end)
            }
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<Command>) -> u32 {
    let mut grid = vec![vec![false; 1000]; 1000];
    for command in input {
        match command {
            Command::TurnOn(start, end) => {
                for i in start.0..end.0 + 1 {
                    for j in start.1..end.1 + 1 {
                        if let Some(row) = grid.get_mut(i as usize) {
                            if let Some(v) = row.get_mut(j as usize) {
                                *v = true;
                            }
                        }
                    }
                }
            }
            Command::TurnOff(start, end) => {
                for i in start.0..end.0 + 1 {
                    for j in start.1..end.1 + 1 {
                        if let Some(row) = grid.get_mut(i as usize) {
                            if let Some(v) = row.get_mut(j as usize) {
                                *v = false;
                            }
                        }
                    }
                }
            }
            Command::Toggle(start, end) => {
                for i in start.0..end.0 + 1 {
                    for j in start.1..end.1 + 1 {
                        if let Some(row) = grid.get_mut(i as usize) {
                            if let Some(v) = row.get_mut(j as usize) {
                                *v ^= true;
                            }
                        }
                    }
                }
            }
        };
    }
    let mut count = 0;
    for i in &grid {
        for j in i {
            if *j {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<Command>) -> u32 {
    let mut grid = vec![vec![0; 1000]; 1000];
    for command in input {
        match command {
            Command::TurnOn(start, end) => {
                for i in start.0..end.0 + 1 {
                    for j in start.1..end.1 + 1 {
                        if let Some(row) = grid.get_mut(i as usize) {
                            if let Some(v) = row.get_mut(j as usize) {
                                *v += 1;
                            }
                        }
                    }
                }
            }
            Command::TurnOff(start, end) => {
                for i in start.0..end.0 + 1 {
                    for j in start.1..end.1 + 1 {
                        if let Some(row) = grid.get_mut(i as usize) {
                            if let Some(v) = row.get_mut(j as usize) {
                                if *v > 0 {
                                    *v -= 1;
                                }
                            }
                        }
                    }
                }
            }
            Command::Toggle(start, end) => {
                for i in start.0..end.0 + 1 {
                    for j in start.1..end.1 + 1 {
                        if let Some(row) = grid.get_mut(i as usize) {
                            if let Some(v) = row.get_mut(j as usize) {
                                *v += 2;
                            }
                        }
                    }
                }
            }
        };
    }
    let mut sum = 0;
    for i in &grid {
        for j in i {
            sum += *j;
        }
    }
    sum
}
