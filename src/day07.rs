use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Input {
    Wire(String),
    Value(u16),
}

impl Input {
    pub fn to_value(&self, board: &Board) -> u16 {
        println!("{self:?}");
        println!("{:?}", board.wires);
        match self {
            Self::Value(v) => *v,
            Self::Wire(wire) => *board.wires.get(wire).unwrap(),
        }
    }
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

    pub fn get_wire(&mut self, wire: String) -> u16 {
        if let Some(value) = self.wires.get(&wire) {
            return *value;
        }

        self.gates.iter().find(|w| w.)

        todo!()
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Board {
    let mut board = Board::default();
    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<_>>();
        let len = split.len();
        if len == 3 {
            if let Ok(value) = split.first().unwrap().parse() {
                let wire = split.last().unwrap().to_string();
                board.add_wire(wire, value);
            } else {
                board.add_gate(Gate::PASS {
                    input: Input::Wire(split.first().unwrap().to_string()),
                    output: split.last().unwrap().to_string(),
                });
            };
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
    board
}

#[aoc(day7, part1)]
pub fn part1(input: &Board) -> u16 {
    let mut board = input.clone();
    for gate in input.gates.iter() {
        match gate {
            Gate::PASS { input, output } => match input {
                Input::Wire(wire) => {
                    let value = board.wires.get(wire).unwrap();
                    board.wires.insert(output.clone(), *value);
                }
                Input::Value(value) => {
                    board.wires.insert(output.clone(), *value);
                }
            },
            Gate::OR {
                input1,
                input2,
                output,
            } => {
                let input1 = input1.to_value(&board);
                let input2 = input2.to_value(&board);
                board.wires.insert(output.to_owned(), input1 | input2);
            }
            Gate::AND {
                input1,
                input2,
                output,
            } => {
                let input1 = input1.to_value(&board);
                let input2 = input2.to_value(&board);
                board.wires.insert(output.to_owned(), input1 & input2);
            }

            Gate::LSHIFT {
                input1,
                input2,
                output,
            } => {
                let input1 = input1.to_value(&board);
                let input2 = input2.to_value(&board);
                board.wires.insert(output.to_owned(), input1 << input2);
            }
            Gate::RSHIFT {
                input1,
                input2,
                output,
            } => {
                let input1 = input1.to_value(&board);
                let input2 = input2.to_value(&board);
                board.wires.insert(output.to_owned(), input1 >> input2);
            }
            Gate::NOT { input, output } => {
                let input = input.to_value(&board);
                board.wires.insert(output.to_owned(), !input);
            }
        };
    }

    *input.wires.get(&String::from("a")).unwrap()
}
