use itertools::Itertools;
#[derive(Debug)]
enum OperationMode {
    Position,
    Immediate,
}
struct Amp {
    amps: Vec<Machine>,
    index: usize,
}
struct Machine {
    pc: usize,
    memory: Vec<i32>,
    input: Vec<i32>,
    output: Vec<i32>,
}
impl Machine {
    pub fn new(memory: Vec<i32>, input: Vec<i32>) -> Self {
        Self {
            pc: 0,
            memory,
            input,
            output: Vec::new(),
        }
    }
}
impl Amp {
    /// Get the current opcode under execution
    fn get_opcode(&self) -> i32 {
        self.amps[self.index].memory[self.amps[self.index].pc] % 100
    }

    /// Get the mode for the given parameter
    fn get_param_mode(&self, offset: usize) -> OperationMode {
        match (self.amps[self.index].memory[self.amps[self.index].pc] as usize / (10_usize.pow(offset as u32 + 1))) % 10 {
            1 => OperationMode::Immediate,
            _ => OperationMode::Position
        }
    }

    /// Get values based on the operation mode
    fn get_param(&self, nth: usize) -> i32 {
        let memory = &self.amps[self.index].memory;

        let param_mode = self.get_param_mode(nth);
        let offset = self.amps[self.index].pc + nth;

        match param_mode {
            OperationMode::Position => {
                let address: usize = memory[offset] as usize;
                memory[address]
            },
            OperationMode::Immediate => {
                memory[offset]
            }
        }
    }

    /// Get address based on PC offset
    fn get_address(&self, nth: usize) -> usize {
        self.amps[self.index].memory[self.amps[self.index].pc + nth] as usize
    }

    pub fn execute_program(&mut self) {
        loop {
            let mut switch = false;
            let pc = self.amps[self.index].pc;
            let opcode = self.get_opcode();

            let step = match opcode {
                // sum
                1 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let target_location = self.amps[self.index].memory[pc + 3];
                    self.amps[self.index].memory[target_location as usize] = v1 + v2;
    
                    4
                },
                // mul
                2 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let target_location = self.amps[self.index].memory[pc + 3];
                    self.amps[self.index].memory[target_location as usize] = v1 * v2;
    
                    4
                },
                // Store
                3 => {
                    let address = self.get_address(1);
                        self.amps[self.index].memory[address] = self.amps[self.index].input.remove(0);
    
                    2
                },
                // Read
                4 => {
                    let value = self.get_param(1);
                    self.amps[self.index].output.push(value);
                    self.amps[(self.index + 1) % 5].input.push(value);
                    switch = true;
    
                    2
                },
                // Jump if true
                5 => {
                    let value = self.get_param(1);
                    if value != 0 {
                        let address = self.get_param(2);
                        self.amps[self.index].pc = address as usize;
                        0
                    } else {
                        3
                    }
                },
                // Jump if false
                6 => {
                    let value = self.get_param(1);
                    if value == 0 {
                        let address = self.get_param(2);
                        self.amps[self.index].pc = address as usize;
                        0
                    } else {
                        3
                    }
                },
                // less than
                7 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let address = self.get_address(3);
                    self.amps[self.index].memory[address] = (v1 < v2) as i32;

                    4
                },
                8 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let address = self.get_address(3);
                    self.amps[self.index].memory[address] = (v1 == v2) as i32;

                    4
                },
                99 => { if self.index != 4 {
                    self.index = (self.index + 1) % 5;
                }; break
                 },
                _ => {
                    panic!("[{}:{}] Opcode not recognized", self.amps[self.index].pc, opcode)
                },
            };
    
            self.amps[self.index].pc += step;
            if switch == true {
                self.index = (self.index + 1) % 5;
            }
        }
    }
}
fn part1(input: &str, mut set: Vec<i32>) -> i32 {
    let initial_memory: Vec<i32> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut result = Vec::new();
    set.insert(1, 0);
    let mut apmlifiers = Amp {
        amps: vec![Machine::new(initial_memory.clone(), set[0..2].to_vec()), Machine::new(initial_memory.clone(), set[2..3].to_vec()), Machine::new(initial_memory.clone(), set[3..4].to_vec()), Machine::new(initial_memory.clone(), set[4..5].to_vec()), Machine::new(initial_memory.clone(), set[5..=5].to_vec())],
        index: 0,
    };
    apmlifiers.execute_program();
    result.extend(&apmlifiers.amps[4].output);
    result.into_iter().last().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(139629729, part1("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5", vec![9,8,7,6,5]));
    }

    #[test]
    fn test_2() {
        assert_eq!(18216, part1("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", vec![9,7,8,5,6]));
    }
}