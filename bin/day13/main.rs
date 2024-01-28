use std::collections::HashMap;
use itertools::Itertools;
use std::fmt;

#[derive(Debug)]
enum OperationMode {
    Position,
    Immediate,
    Relative,
}

struct Machine {
    pc: usize,
    memory: HashMap<usize, i64>,
    input: Vec<i64>,
    output: Vec<i64>,
    relative_base: usize,
    score: i64,
    ball_x: i64,
    paddle_x: i64,
    display: Vec<Vec<char>>,
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.display {
            for &c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Machine {
    fn new(program: Vec<i64>, input: Vec<i64>) -> Self {
        let memory = program.into_iter().enumerate().map(|(i, v)| (i, v)).collect();
        Self {
            pc: 0,
            memory,
            input,
            output: Vec::new(),
            relative_base: 0,
            score: 0,
            ball_x: 0,
            paddle_x: 0,
            display: vec![vec!['.'; 40]; 30],
        }
    }

    fn calculate_blocks(&self) -> usize {
        self.output.iter().tuples::<(_, _, _)>().filter(|&x| x.2 == &2).count()
    }

    fn read_memory(&self, address: usize) -> i64 {
        *self.memory.get(&address).unwrap_or(&0)
    }

    fn write_memory(&mut self, address: usize, value: i64) {
        self.memory.insert(address, value);
    }

    fn get_opcode(&self) -> i64 {
        self.read_memory(self.pc) % 100
    }

    fn get_param_mode(&self, offset: usize) -> OperationMode {
        match (self.read_memory(self.pc) as usize / (10_usize.pow(offset as u32 + 1))) % 10 {
            1 => OperationMode::Immediate,
            2 => OperationMode::Relative,
            _ => OperationMode::Position,
        }
    }

    fn get_param(&self, nth: usize) -> i64 {
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
                let address = (self.read_memory(offset) + relative_base as i64) as usize;
                self.read_memory(address)
            }
        }
    }

    fn get_address(&self, nth: usize) -> usize {
        let param_mode = self.get_param_mode(nth);

        match param_mode {
            OperationMode::Position => self.read_memory(self.pc + nth) as usize,
            OperationMode::Relative => (self.read_memory(self.pc + nth) + self.relative_base as i64) as usize,
            _ => panic!("Invalid mode for getting address"),
        }
    }

    fn execute_program(&mut self) {
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
                    let joystick_move = if self.ball_x < self.paddle_x {
                        -1
                    } else if self.ball_x > self.paddle_x {
                        1
                    } else {
                        0
                    };
                
                    let address = self.get_address(1);
                    self.write_memory(address, joystick_move);
                    self.pc += 2;
                },
                4 => {
                    let value = self.get_param(1);
                    self.output.push(value);
                
                    if self.output.len() == 3 {
                        let x = self.output[0];
                        let y = self.output[1];
                        let tile_id = self.output[2];
                
                        match (x, y, tile_id) {
                            (-1, 0, new_score) => {
                                self.score = new_score;
                                println!("{}", self);
                                println!("{}", new_score);
                                (0..self.display.len()).for_each(|dy| {
                                    (0..self.display[0].len()).for_each(|dx| {
                                        if self.display[dy][dx] == 'O' || self.display[dy][dx] == '_' {
                                            self.display[dy][dx] = '.'
                                        }
                                    })
                                });
                            },
                            (ball_x, y, 4) => {
                                self.ball_x = ball_x;
                                self.display[y as usize][ball_x as usize] = 'O'
                            },
                            (paddle_x, y, 3) => {
                                self.paddle_x = paddle_x;
                                self.display[y as usize][paddle_x as usize] = '_'
                            },
                            (x, y, 2) => {
                                self.display[y as usize][x as usize] = '#'
                            },
                            (x, y, 1) => {
                                self.display[y as usize][x as usize] = '|'
                            },
                            _ => {}
                        }
            
                        self.output.clear();
                    }
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
                    self.write_memory(target_address, (v1 < v2) as i64);
                    self.pc += 4;
                },
                8 => {
                    let (v1, v2) = (self.get_param(1), self.get_param(2));
                    let target_address = self.get_address(3);
                    self.write_memory(target_address, (v1 == v2) as i64);
                    self.pc += 4;
                },
                9 => {
                    let value = self.get_param(1);
                    self.relative_base = (self.relative_base as i64 + value) as usize;
                    self.pc += 2;
                },
                99 => { 
                    println!("Final score: {}", self.score);
                    break; 
                },
                _ => {
                    panic!("Invalid opcode: {}", opcode)
                },
            };
        }
    }
}

fn main() {
    let input = include_str!("input13.txt");
    let mut initial_memory: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    initial_memory[0] = 2;
    let mut machine = Machine::new(initial_memory, vec![2]);

    machine.execute_program();

}