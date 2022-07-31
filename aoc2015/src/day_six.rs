pub fn parse_pair(s: &str) -> (i32, i32) {
    let s: Vec<i32> = s
        .split(",")
        .map(|e| e.to_string().parse::<i32>().unwrap())
        .collect();

    (*s.get(0).unwrap(), *s.get(1).unwrap())
}

enum Command {
    Toggle,
    TurnOn,
    TurnOff,
}
pub struct Instruction {
    r1: (i32, i32),
    r2: (i32, i32),
    command: Command,
}

impl Instruction {
    pub fn new(s: &str) -> Instruction {
        let tokens: Vec<&str> = s.split(" ").collect();
        match tokens.len() {
            4 => {
                //toggle command
                return Instruction {
                    r1: parse_pair(tokens[1]),
                    r2: parse_pair(tokens[3]),
                    command: Command::Toggle,
                };
            }
            5 => {
                //turn off/on
                match tokens[1] {
                    "on" => {
                        return Instruction {
                            r1: parse_pair(tokens[2]),
                            r2: parse_pair(tokens[4]),
                            command: Command::TurnOn,
                        }
                    }
                    "off" => {
                        return Instruction {
                            r1: parse_pair(tokens[2]),
                            r2: parse_pair(tokens[4]),
                            command: Command::TurnOff,
                        }
                    }
                    _ => {
                        return Instruction {
                            r1: (0, 0),
                            r2: (0, 0),
                            command: Command::Toggle,
                        }
                    }
                }
            }
            _ => Instruction {
                r1: (0, 0),
                r2: (0, 0),
                command: Command::TurnOff,
            },
        }
    }
}

pub struct LightMap {
    board: [[i32; 1000]; 1000],
    pub count: i32,
}

impl LightMap {
    pub fn new() -> LightMap {
        LightMap {
            board: [[0; 1000]; 1000],
            count: 0,
        }
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        self.turn_on(x, y);
        self.turn_on(x, y);
    }

    pub fn turn_on(&mut self, x: usize, y: usize) {
        self.board[x][y] += 1;
        self.count += 1;
    }

    pub fn turn_off(&mut self, x: usize, y: usize) {
        if self.board[x][y] > 0 {
            self.board[x][y] -= 1;
            self.count -= 1;
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        for i in instruction.r1.0..instruction.r2.0 + 1 {
            for j in instruction.r1.1..instruction.r2.1 + 1 {
                match instruction.command {
                    Command::Toggle => self.toggle(i as usize, j as usize),
                    Command::TurnOn => self.turn_on(i as usize, j as usize),
                    Command::TurnOff => self.turn_off(i as usize, j as usize),
                }
            }
        }
    }
}

pub fn build_decoration(path: &str) -> i32 {
    let mut m = LightMap::new();

    let commands: Vec<&str> = std::fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|e| {
            let i = Instruction::new(e);
            m.execute_instruction(i);
            e
        })
        .collect();

    m.count
}
