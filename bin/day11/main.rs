use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug)]
enum OperationMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
struct Bot {
    grid: Vec<Vec<i64>>,
    seen: HashSet<(usize, usize)>,
    degree: i32,
    x: usize,
    y: usize,
}

impl fmt::Display for Bot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for &cell in row {
                let symbol = match cell {
                    0 => '.',
                    _ => '#',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Bot {
    fn new() -> Self {
        let mut grid = vec![vec![0; 45]; 10];
        grid[2][0] = 1;
        Self {
            grid: grid,
            seen: HashSet::new(),
            degree: 270,
            x: 0,
            y: 2,
        }
    }
    fn count_panels(&self) -> usize {
        self.seen.len()
    }
}
struct Machine {
    pc: usize,
    memory: HashMap<usize, i64>,
    output: Vec<i64>,
    relative_base: usize,
    bot: Bot,
}

impl Machine {
    pub fn new(program: Vec<i64>) -> Self {
        let memory = program.into_iter().enumerate().map(|(i, v)| (i, v)).collect();
        Self {
            pc: 0,
            memory,
            output: Vec::new(),
            relative_base: 0,
            bot: Bot::new(),
        }
    }

    fn paint(&mut self) {
        if let Some(dir) = self.output.pop() {
            let color = self.output.pop().expect("Output should contains two numbers");
            self.bot.grid[self.bot.y][self.bot.x] = color;

            self.bot.seen.insert((self.bot.y, self.bot.x));

            self.bot.degree += match dir {
                0 => -90,
                _ => 90,
            };
            match self.bot.degree.rem_euclid(360) {
                0 => self.bot.x += 1,
                90 => self.bot.y += 1,
                180 => self.bot.x -= 1,
                270 => self.bot.y -= 1,
                _ => println!("The number shouldn't exceed 360"),
            }
        }
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
                    let input_value = self.bot.grid[self.bot.y][self.bot.x];
                    let address = self.get_address(1);
                    self.write_memory(address, input_value);
                    self.pc += 2;
                },
                4 => {
                    let value = self.get_param(1);
                    self.output.push(value);
                    if self.output.len() == 2 {
                        self.paint();
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
                99 => { break; },
                _ => {
                    panic!("Invalid opcode: {}", opcode)
                },
            };
        }
    }
}

fn main() {
    let input = include_str!("input11.txt");
    let initial_memory: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut machine = Machine::new(initial_memory);
    machine.execute_program();

    println!("{}", machine.bot);
}
