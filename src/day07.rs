use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Input {
    Wire(String),
    Value(u16),
}

#[derive(Debug, Clone)]
pub enum Gate {
    PASS {
        input: Input,
        output: String,
    },
    OR {
        input1: Input,
        input2: Input,
        output: String,
    },
    AND {
        input1: Input,
        input2: Input,
        output: String,
    },
    LSHIFT {
        input1: Input,
        input2: Input,
        output: String,
    },
    RSHIFT {
        input1: Input,
        input2: Input,
        output: String,
    },
    NOT {
        input: Input,
        output: String,
    },
}

#[derive(Debug, Clone, Default)]
pub struct Board {
    wires: HashMap<String, u16>,
    gates: Vec<Gate>,
}

impl Board {
    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }
    pub fn add_wire(&mut self, wire: String, value: u16) {
        self.wires.insert(wire, value);
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Board {
    let mut board = Board::default();
    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<_>>();
        let len = split.len();
        if len == 3 {
            let value: u16 = if let Ok(value) = split.first().unwrap().parse() {
                value
            } else {
                todo!()
            };
            let wire = split.last().unwrap().to_string();
            board.add_wire(wire, value);
        } else if len == 4 {
            let input = split.get(1).unwrap().to_string();
            let output = split.last().unwrap().to_string();
            board.add_gate(Gate::NOT {
                input: Input::Wire(input),
                output,
            });
        } else if len == 5 {
            let input1 = split.first().unwrap();
            let input1 = if let Ok(value) = input1.parse::<u16>() {
                Input::Value(value)
            } else {
                Input::Wire(input1.to_string())
            };

            let input2 = split.get(2).unwrap();
            let input2 = if let Ok(value) = input2.parse::<u16>() {
                Input::Value(value)
            } else {
                Input::Wire(input2.to_string())
            };

            let output = split.last().unwrap().to_string();

            let gate = split.get(1).unwrap();
            let gate = match *gate {
                "AND" => Gate::AND {
                    input1,
                    input2,
                    output,
                },
                "OR" => Gate::OR {
                    input1,
                    input2,
                    output,
                },
                "RSHIFT" => Gate::RSHIFT {
                    input1,
                    input2,
                    output,
                },
                "LSHIFT" => Gate::LSHIFT {
                    input1,
                    input2,
                    output,
                },
                _ => unreachable!(),
            };
            board.add_gate(gate);
        }
    }
    todo!()
}

#[aoc(day7, part1)]
pub fn part1(input: &Board) -> u16 {
    println!("{input:?}");
    todo!()
}
