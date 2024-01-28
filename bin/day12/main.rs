use num_integer::lcm;

#[derive(Debug, Clone)]
struct Moons {
    moons: Vec<Moon>,
}

impl Moons {
    fn new() -> Self {
        Moons {
            moons: Vec::new(),
        }
    }
    fn apply_gravity(&mut self) {
        for i in 0..4 {
            for j in (i+1)..4 {
                for idx in 0..3 {
                    if self.moons[i].pos[idx] > self.moons[j].pos[idx] {
                        self.moons[i].vel[idx] -= 1;
                        self.moons[j].vel[idx] += 1;
                    } else if self.moons[i].pos[idx] < self.moons[j].pos[idx] {
                        self.moons[i].vel[idx] += 1;
                        self.moons[j].vel[idx] -= 1;
                    }
                }
            }
        }
    }    
    fn apply_velocity(&mut self) {
        (0..self.moons.len()).for_each(|idx| {
            (0..3).for_each(|x| {
                self.moons[idx].pos[x] += self.moons[idx].vel[x]
            })
        });
    }
    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }
    fn find_total_energy(&mut self) -> i32 {
        self.moons.iter_mut().map(|moon| {
            moon.find_energy();
            moon.kin * moon.pot
        }).sum()
    }
    fn apply_gravity_axis(&mut self, axis: usize) {
        for to in 0..self.moons.len() {
            for from in 0..self.moons.len() {
            if to == from {continue}
                if self.moons[to].pos[axis] > self.moons[from].pos[axis] {
                    self.moons[to].vel[axis] -= 1
                } else if self.moons[to].pos[axis] < self.moons[from].pos[axis] {
                    self.moons[to].vel[axis] += 1
                }
            }
        }
    }
    fn apply_velocity_axis(&mut self, axis: usize) {
        (0..self.moons.len()).for_each(|idx| {
            self.moons[idx].pos[axis] += self.moons[idx].vel[axis]
        });
    }
    fn find_state(&self, axis: usize) -> Vec<(i32, i32)> {
        self.moons.iter().map(|moon| (moon.pos[axis], moon.vel[axis])).collect()
    }
}

#[derive(Debug, Clone)]
struct Moon {
    pos: Vec<i32>,
    vel: Vec<i32>,
    pot: i32,
    kin: i32,
}

impl Moon {
    fn new(input: Vec<i32>) -> Self {
        Moon {
            pos: input,
            vel: vec![0; 3],
            pot: 0,
            kin: 0,
        }
    }
    fn find_energy(&mut self) {
        self.pot = self.pos.iter().map(|x| x.abs()).sum();
        self.kin = self.vel.iter().map(|x| x.abs()).sum();

    }
}

fn parse_input() -> Moons {
    let input = include_str!("input12.txt");
    let mut moons = Moons::new();
    input.lines().for_each(|line| {
        let mut stack = String::new();
        for c in line.chars() {
            if c == '-' || c.is_digit(10) {
                stack.push(c)
            } else {
                stack.push(' ')
            }
        }
        let stack: Vec<i32> = stack.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        let moon = Moon::new(stack);
        moons.moons.push(moon)
    });
    moons
}
fn part1() -> i32 {
    let mut moons = parse_input();
    for _ in 0..1000 {
        moons.step()
    }
    moons.find_total_energy()
}

fn find_period(moons: &mut Moons, axis: usize) -> i64 {
    let initial_state: Vec<(i32, i32)> = moons.find_state(axis);
    let mut states = std::collections::HashSet::new();

    for step in 1.. {
        moons.apply_gravity_axis(axis);
        moons.apply_velocity_axis(axis);

        let current_state: Vec<(i32, i32)> = moons.find_state(axis);
        if current_state == initial_state || !states.insert(current_state) {
            return step;
        }
    }
    0
}

fn part2() -> i64 {
    let mut moons = parse_input();
    let x_period = find_period(&mut moons, 0);
    let y_period = find_period(&mut moons, 1);
    let z_period = find_period(&mut moons, 2);

    lcm(x_period, lcm(y_period, z_period))
}

fn main() {
    println!("{}", part1());
}