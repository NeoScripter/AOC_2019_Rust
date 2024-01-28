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

    /// Get the current opcode under execution
    fn get_opcode(&self) -> i32 {
        self.memory[self.pc] % 100
    }

    /// Get the mode for the given parameter
    fn get_param_mode(&self, offset: usize) -> OperationMode {
        match (self.memory[self.pc] as usize / (10_usize.pow(offset as u32 + 1))) % 10 {
            1 => OperationMode::Immediate,
            _ => OperationMode::Position
        }
    }

    /// Get values based on the operation mode
    fn get_param(&self, nth: usize) -> i32 {
        let memory = &self.memory;

        let param_mode = self.get_param_mode(nth);
        let offset = self.pc + nth;

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
        self.memory[self.pc + nth] as usize
    }

    pub fn execute_program(&mut self) {
        loop {
            let pc = self.pc;
            let opcode = self.get_opcode();

            let step = match opcode {
                // sum
                1 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let target_location = self.memory[pc + 3];
                    self.memory[target_location as usize] = v1 + v2;
    
                    4
                },
                // mul
                2 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let target_location = self.memory[pc + 3];
                    self.memory[target_location as usize] = v1 * v2;
    
                    4
                },
                // Store
                3 => {
                    let address = self.get_address(1);
                    self.memory[address] = self.input.remove(0);
    
                    2
                },
                // Read
                4 => {
                    let value = self.get_param(1);
                    self.output.push(value);
    
                    2
                },
                // Jump if true
                5 => {
                    let value = self.get_param(1);
                    if value != 0 {
                        let address = self.get_param(2);
                        self.pc = address as usize;
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
                        self.pc = address as usize;
                        0
                    } else {
                        3
                    }
                },
                // less than
                7 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let address = self.get_address(3);
                    self.memory[address] = (v1 < v2) as i32;

                    4
                },
                8 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let address = self.get_address(3);
                    self.memory[address] = (v1 == v2) as i32;

                    4
                },
                99 => { break; },
                _ => {
                    panic!("[{}:{}] Opcode not recognized", self.pc, opcode)
                },
            };
    
            self.pc += step;
        }
    }
}
fn parse(input: &str) -> Vec<i32> {
    let initial_memory: Vec<i32> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    initial_memory
}
fn part1(input: &str) -> i32 {
    let initial_memory: Vec<i32> = parse(input);
    let initial_setting = vec![0, 1, 2, 3, 4];
    initial_setting.into_iter().permutations(5).map(|perm| {
        let mut result = 0;
        for ps in perm {
            let mut phase_settings = Vec::new();
            phase_settings.push(ps);
            phase_settings.push(result);
            let mut machine = Machine::new(initial_memory.clone(), phase_settings);
            machine.execute_program();
            result = *machine.output.iter().next().unwrap();
        }
        result
    }).max().unwrap()
}
fn part2(input: &str) -> i32 {
    let initial_memory: Vec<i32> = parse(input);
    let initial_setting = vec![5, 6, 7, 8, 9];
    initial_setting.into_iter().permutations(5).map(|mut perm| {
        let mut result = Vec::new();
        perm.insert(1, 0);
        let mut apmlifiers = Amp {
            amps: vec![],
            index: 0,
        };
        for idx in 1..6 {
            let mut comb = vec![];
            if idx == 1 {
                comb = perm[0..idx + 1].to_vec();
            } else {
                comb = perm[idx..=idx].to_vec();
            }
            apmlifiers.amps.push(Machine::new(initial_memory.clone(), comb));
        }
        apmlifiers.execute_program();
        result.extend(&apmlifiers.amps[4].output);
        result.into_iter().last().unwrap()
    }).max().unwrap()
}
fn main() {
    let input = include_str!("input7.txt");
    println!("{}, {}", part1(input), part2(input));
}