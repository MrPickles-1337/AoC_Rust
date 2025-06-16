use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Input {
    Wire(String),
    Value(u16),
}

impl Input {
    pub fn evaluate(&self, gates: &[Gate], wires: &mut HashMap<String, u16>) -> u16 {
        match self {
            Input::Value(value) => *value,
            Input::Wire(wire) => evaluate_wire(wire, gates, wires),
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

impl Gate {
    #[allow(unused_variables)]
    pub fn output(&self) -> &String {
        match self {
            Gate::PASS { input, output } => output,
            Gate::OR {
                input1,
                input2,
                output,
            } => output,
            Gate::AND {
                input1,
                input2,
                output,
            } => output,
            Gate::LSHIFT {
                input1,
                input2,
                output,
            } => output,
            Gate::RSHIFT {
                input1,
                input2,
                output,
            } => output,
            Gate::NOT { input, output } => output,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Board {
    wires: HashMap<String, u16>,
    gates: Vec<Gate>,
}

fn evaluate_wire(wire: &String, gates: &[Gate], wires: &mut HashMap<String, u16>) -> u16 {
    if let Some(value) = wires.get(wire) {
        return *value;
    }

    let gate = gates.iter().find(|g| g.output() == wire).unwrap();
    #[allow(unused_variables)]
    let value = match gate {
        Gate::PASS { input, output } => input.evaluate(gates, wires),
        Gate::OR {
            input1,
            input2,
            output,
        } => input1.evaluate(gates, wires) | input2.evaluate(gates, wires),
        Gate::AND {
            input1,
            input2,
            output,
        } => input1.evaluate(gates, wires) & input2.evaluate(gates, wires),
        Gate::LSHIFT {
            input1,
            input2,
            output,
        } => input1.evaluate(gates, wires) << input2.evaluate(gates, wires),
        Gate::RSHIFT {
            input1,
            input2,
            output,
        } => input1.evaluate(gates, wires) >> input2.evaluate(gates, wires),
        Gate::NOT { input, output } => !input.evaluate(gates, wires),
    };
    wires.insert(gate.output().clone(), value);
    value
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
    evaluate_wire(&String::from("a"), &board.gates, &mut board.wires)
}

#[aoc(day7, part2)]
pub fn part2(input: &Board) -> u16 {
    let mut board = input.clone();
    let mut board_clone = input.clone();
    let a = evaluate_wire(&String::from("a"), &board.gates, &mut board.wires);
    board_clone.add_wire(String::from("b"), a);
    evaluate_wire(
        &String::from("a"),
        &board_clone.gates,
        &mut board_clone.wires,
    )
}
