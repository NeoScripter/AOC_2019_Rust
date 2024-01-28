use std::collections::HashMap;

#[derive(Debug)]
enum OperationMode {
    Position,
    Immediate,
    Relative,
}

struct Machine {
    pc: usize,
    memory: HashMap<usize, i128>,
    input: Vec<i128>,
    output: Vec<i128>,
    relative_base: usize,
}

impl Machine {
    pub fn new(program: Vec<i128>, input: Vec<i128>) -> Self {
        let memory = program.into_iter().enumerate().map(|(i, v)| (i, v)).collect();
        Self {
            pc: 0,
            memory,
            input,
            output: Vec::new(),
            relative_base: 0,
        }
    }

    fn read_memory(&self, address: usize) -> i128 {
        *self.memory.get(&address).unwrap_or(&0)
    }

    fn write_memory(&mut self, address: usize, value: i128) {
        self.memory.insert(address, value);
    }

    fn get_opcode(&self) -> i128 {
        self.read_memory(self.pc) % 100
    }

    fn get_param_mode(&self, offset: usize) -> OperationMode {
        match (self.read_memory(self.pc) as usize / (10_usize.pow(offset as u32 + 1))) % 10 {
            1 => OperationMode::Immediate,
            2 => OperationMode::Relative,
            _ => OperationMode::Position,
        }
    }

    fn get_param(&self, nth: usize) -> i128 {
        let param_mode = self.get_param_mode(nth);
        let offset = self.pc + nth;
        let relative_base = self.relative_base;

        match param_mode {
            OperationMode::Position => {
                let address = self.read_memory(offset) as usize;
                self.read_memory(address)
            },
            OperationMode::Immediate => {
                self.read_memory(offset)
            },
            OperationMode::Relative => {
                let address = (self.read_memory(offset) + relative_base as i128) as usize;
                self.read_memory(address)
            }
        }
    }

    fn get_address(&self, nth: usize) -> usize {
        let param_mode = self.get_param_mode(nth);

        match param_mode {
            OperationMode::Position => self.read_memory(self.pc + nth) as usize,
            OperationMode::Relative => (self.read_memory(self.pc + nth) + self.relative_base as i128) as usize,
            _ => panic!("Invalid mode for getting address"),
        }
    }

    pub fn execute_program(&mut self) {
        loop {
            let opcode = self.get_opcode();

            match opcode {
                1 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let target_address = self.get_address(3);
                    self.write_memory(target_address, v1 + v2);
                    self.pc += 4;
                },
                2 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let target_address = self.get_address(3);
                    self.write_memory(target_address, v1 * v2);
                    self.pc += 4;
                },
                3 => {
                    let input_value = self.input.remove(0);
                    let address = self.get_address(1);
                    self.write_memory(address, input_value);
                    self.pc += 2;
                },
                4 => {
                    let value = self.get_param(1);
                    self.output.push(value);
                    self.pc += 2;
                },
                5 => {
                    let value = self.get_param(1);
                    if value != 0 {
                        self.pc = self.get_param(2) as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                6 => {
                    let value = self.get_param(1);
                    if value == 0 {
                        self.pc = self.get_param(2) as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                7 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let target_address = self.get_address(3);
                    self.write_memory(target_address, (v1 < v2) as i128);
                    self.pc += 4;
                },
                8 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let target_address = self.get_address(3);
                    self.write_memory(target_address, (v1 == v2) as i128);
                    self.pc += 4;
                },
                9 => {
                    let value = self.get_param(1);
                    self.relative_base = (self.relative_base as i128 + value) as usize;
                    self.pc += 2;
                },
                99 => { break; },
                _ => {
                    panic!("Invalid opcode: {}", opcode)
                },
            };
        }
    }
}

fn main() {
    let input = include_str!("input9.txt");
    let initial_memory: Vec<i128> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut machine = Machine::new(initial_memory, vec![2]);
    machine.execute_program();

    println!("{:?}", machine.output);
}
