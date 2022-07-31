use std::collections::HashMap;
// Using stacks ?
// Using graphs ? -> Trees ?
// Simpler:
// Find direct assignments first
// Then find components such that the lhs involves both of the
// in the map. Repeat until no instructions left.

/// Circuit emulator given the input list in AOC2015V standard.

// bummer, can't store
// static NO_SIG: u16 = 2<<15 + 1;
pub struct Emulator {
    wires: HashMap<String, u16>,
    raw_signals: Vec<String>,
}

impl Emulator {
    pub fn new(path: &str) -> Emulator {
        let signals: Vec<String> = std::fs::read_to_string(path)
            .unwrap()
            .split("\n")
            .map(|e| e.to_string())
            .collect();

        Emulator {
            wires: HashMap::new(),
            raw_signals: signals,
        }
    }

    pub fn execute_circuit(&mut self) {
        // Find direct assignments first
        //split a signal with delimiter: '\n'
        //3 -> direct assignment
        //5 -> binary op
        //4 -> unary op: !
        // Then find components such that the lhs involves both of the
        // in the map. Repeat until no instructions left.

        // first loop looking ok
        for (i, signal) in self.raw_signals.to_owned().iter().enumerate() {
            let parts: Vec<&str> = signal.split(" ").collect();
            if parts.len() == 3 {
                let v = parts[0].to_string().parse::<u16>();
                match v {
                    Err(_) => (),
                    Ok(v)=> {
                        self.wires.insert(
                            parts[2].to_string(),
                            v,
                        );
                        self.raw_signals.swap_remove(i);
                    }
                }
            }
        }
        // eol

        // problem:
        // given the operands, one of them be either a wire or a literal. How to match while 
        // preserving readable structure ? 
        loop {
            if self.raw_signals.len() == 0 {
                break;
            }
            'inner:for (i, signal) in self.raw_signals.to_owned().iter().enumerate() {
                let parts: Vec<&str> = signal.split(" ").collect();
                match parts.len() {
                    4 => {
                        //unary op : NOT
                        let result_to = parts[3].to_string();
                        let operand = parts[1].to_string();
                        match self.wires.to_owned().get(&operand) {
                            Some(v ) => {
                                match self.wires.get_mut(&result_to) {
                                    Some(a) => *a = !*v,
                                    None => {self.wires.insert(result_to, !*v);},
                                }
                            }
                            None => {
                                match self.wires.get_mut(&result_to){
                                    Some(a) => *a = !operand.parse::<u16>().unwrap(),
                                    None => {self.wires.insert(result_to,!operand.parse::<u16>().unwrap());}
                                }
                            },
                        }
                        self.raw_signals.swap_remove(i);
                    },
                    // Problem starts here
                    5 => {
                        let key_1 = parts[0].to_string();
                        let operator = parts[1].to_string();
                        let key_2 = parts[2].to_string();
                        let result_to = parts[4].to_string();
                        let lhs = match self.wires.contains_key(&key_1) {
                            true => {
                                *self.wires.get(&key_1).unwrap()
                            },
                            false =>{
                                match parts[0].to_string().parse::<u16>(){
                                    Ok(v) => v,
                                    Err(_) => break 'inner, //indicates signal not found yet, since we have 16 bit signals
                                }
                            },
                        };
                        let rhs = match self.wires.contains_key(&key_2) {
                            true => {
                                *self.wires.get(&key_2).unwrap()
                            },
                            false =>{
                                match parts[2].to_string().parse::<u16>(){
                                    Ok(v) => v,
                                    Err(_) => break 'inner, //indicates signal not found yet, since we have 16 bit signals
                                }
                            },
                        };
                            let result = match operator.as_str() {
                                "AND" => lhs ^ rhs,
                                "OR" => lhs | rhs,
                                "RSHIFT" => lhs >> rhs,
                                "LSHIFT" => lhs << rhs,
                                _ => 0,
                            };
                            self.raw_signals.swap_remove(i);
                            if self.wires.contains_key(&result_to) {
                                *self.wires.get_mut(&result_to).unwrap() = result;
                            } else {
                                self.wires.insert(result_to, result);
                            }
                            break 'inner;
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn get_wire(&self, wire: &str) -> u16 {
        *self.wires.get(&wire.to_string()).unwrap()
    }
}
