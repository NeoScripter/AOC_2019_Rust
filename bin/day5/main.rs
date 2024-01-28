// The solution to part 2 is not mine, I found it on a forum as I got stuck with it

#[derive(Debug)]
enum OperationMode {
    Position,
    Immediate,
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
use nom::{
    bytes::complete::tag,
    character::complete::i32,
    multi::separated_list0,
    IResult,
};

const ID: i32 = 1;

fn parse_vector(s: &str) -> IResult<&str, Vec<i32>> {
    separated_list0(tag(","), i32)(s)
}
fn parse(s: &str) -> Vec<i32> {
    let mut vector = vec![];
    if let Ok((_rest, vec)) = parse_vector(s) {
        vector = vec;
    }
    vector
}
fn break_into_digits(num: i32) -> (i32, i32, i32, i32) {
    let right_two_digits = num % 100;
    let remaining_number = num / 100;
    let right_digit = remaining_number % 10;
    let middle_digit = (remaining_number / 10) % 10;
    let left_digit = remaining_number / 100;
    (left_digit, middle_digit, right_digit, right_two_digits)
}
fn part1(input: &str) -> Vec<i32> {
    let mut program = parse(input);
    let mut pos: usize = 0;
    let mut output = vec![];
    loop {
        let (par3_mode, par2_mode, par1_mode, two_digit_op) = break_into_digits(program[pos]);
        let mut par1 = 0;
        let mut par2 = 0;
        if par1_mode == 0 {
            par1 = program[pos + 1] as usize;
        } else {
            par1 = (pos + 1) as usize;
        }
        if par2_mode == 0 {
            par2 = program[pos + 2] as usize;
        } else {
            par2 = (pos + 2) as usize;
        }
        let par3 = program[pos + 3] as usize;
        match two_digit_op {
            1 => { program[par3] = program[par1] + program[par2]; pos += 4;}
            2 => { program[par3] = program[par1] * program[par2]; pos += 4;}
            3 => { program[par1] = ID; pos += 2; }
            4 => { output.push(program[par1]); pos += 2; }
            99 => { break; }
            _ => { unreachable!(); }
        }
    }
    output
}

fn main() {
    let input = include_str!("input5.txt");
    
    let initial_memory: Vec<i32> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    
    println!("Part 1: {:?}", part1(input));

    let mut machine = Machine::new(initial_memory.clone(), vec![5]);
    machine.execute_program();
    println!("Part 2: {:?}", machine.output);
}