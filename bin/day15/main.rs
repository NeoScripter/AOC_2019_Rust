use std::{
    collections::{HashMap, VecDeque, HashSet},
    fmt,
};

const CMDS: [i64; 4] = [1, 2, 3, 4];

#[derive(Debug, Clone)]
struct MachineState {
    steps: i64,
    mchn: Machine,
    crds: (usize, usize),
}

impl MachineState {
    fn new(steps: i64, mchn: Machine, crds: (usize, usize)) -> Self {
        Self { steps, mchn, crds }
    }
}

#[derive(Debug)]
enum OperationMode {
    Position,
    Immediate,
    Relative,
}
#[derive(Debug, Clone)]
struct Droid {
    steps: i64,
    grid: Vec<Vec<char>>,
    ox_tank: (usize, usize),
}

impl fmt::Display for Droid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for &ch in row {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Droid {
    fn new() -> Self {
        Self { steps: 0, grid: vec![vec!['.'; 50]; 50], ox_tank: (0, 0) }
    }
    fn nghs(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let mut nghs = Vec::new();
        if y > 0 {nghs.push((y - 1, x))}
        if x > 0 {nghs.push((y, x - 1))}
        if y < self.grid.len() - 1 {nghs.push((y + 1, x))}
        if x < self.grid[0].len() - 1 {nghs.push((y, x + 1))}
        nghs
    } 
    fn oxygen_spread(&mut self) -> u32 {
        let mut cache = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((0, self.ox_tank));
        let mut munites = 0;
        
        while let Some((m, (y, x))) = q.pop_front() {
            if !cache.insert((y, x)) { continue }
            munites = m;
            let nghs = self.nghs(y, x);
            
            for (ny, nx) in nghs {
                if self.grid[ny][nx] != '#' { 
                    q.push_back((m + 1, (ny, nx))); 
                    self.grid[ny][nx] = 'O'; 
                }
            }
        }
        munites
    }
    fn append_grid(&mut self) {
        let (mut min_x, mut max_x, mut min_y, mut max_y) = (usize::MAX, 0, usize::MAX, 0);
        (0..self.grid.len()).for_each(|y| {
            (0..self.grid[0].len()).for_each(|x| {
                if self.grid[y][x] == '#' { min_x = min_x.min(x); min_y = min_y.min(y); max_x = max_x.max(x); max_y = max_y.max(y); }
            });
        });
        self.grid.truncate(max_y + 1);
        self.grid.drain(0..min_y);
        for row in self.grid.iter_mut() {
            row.truncate(max_x + 1);
            row.drain(0..min_x);
        }
        self.ox_tank.0 -= min_y;
        self.ox_tank.1 -= min_x;
    }
    fn start_droid(&mut self, machine: Machine) {
        let mut cache = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(MachineState::new(0, machine, (24, 24)));
        while let Some(MachineState { steps, mchn: m, crds: (y, x) }) = q.pop_front() {
            if !cache.insert((y, x)) { continue }

            for cmd in CMDS {
                let mut copy = m.clone();
                let (ny, nx) = convert_cmds(cmd, y, x);
                copy.input.push_back(cmd);
                copy.execute_program();
    
                if let Some(v) = copy.output.pop_front() {
                    match v {
                        2 => {
                            self.grid[ny][nx] = 'O';
                            self.ox_tank = (ny, nx);
                            self.steps = steps + 1;
                        },
                        1 => {
                            q.push_back(MachineState::new(steps + 1, copy, (ny, nx)));
                        },
                        _ => { self.grid[ny][nx] = '#' },
                    }
                }
            }
        }
    }    
}
#[derive(Debug, Clone)]
struct Machine {
    pc: usize,
    memory: HashMap<usize, i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    relative_base: usize,
}

impl Machine {
    fn new(program: Vec<i64>, input: VecDeque<i64>) -> Self {
        let memory = program.into_iter().enumerate().map(|(i, v)| (i, v)).collect();
        Self {
            pc: 0,
            memory,
            input,
            output: VecDeque::new(),
            relative_base: 0,
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
                    match self.input.pop_front() {
                        Some(input_value) => {                        
                            let address = self.get_address(1);
                            self.write_memory(address, input_value);
                        },
                        None => break,
                    }
                    self.pc += 2;
                },
                4 => {
                    let value = self.get_param(1);
                    self.output.push_back(value);
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

fn convert_cmds(cmd: i64, y: usize, x: usize) -> (usize, usize) {
    match cmd {
        1 => (y - 1, x),
        2 => (y + 1, x),
        3 => (y, x - 1),
        _ => (y, x + 1),
    }
}
fn create_machine() -> Machine {
    let input = include_str!("input15.txt");
    let initial_memory: Vec<i64> = input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let machine = Machine::new(initial_memory, VecDeque::new());
    machine
}

fn part1() -> i64 {
    let machine = create_machine();
    let mut droid = Droid::new();
    droid.start_droid(machine);
    droid.steps
}

fn part2() -> u32 {
    let machine = create_machine();
    let mut droid = Droid::new();
    droid.start_droid(machine);
    droid.append_grid();
    droid.oxygen_spread()
}
fn main() {
    println!("{}", part1())
}