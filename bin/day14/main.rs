use std::collections::{HashMap, VecDeque};

const MINED: u64 = 1000000000000;

#[derive(Debug, Clone)]
struct List (HashMap<String, (u64, Vec<(String, u64)>)>);

impl List {
    fn new() -> Self {
        Self (HashMap::new())
    }
    fn find_ore(&self, fuel: u64) -> u64 {
        let mut leftovers: HashMap<String, u64> = HashMap::new();
        let mut sum = 0;
        let mut queue = VecDeque::new();
        queue.push_back(("FUEL".to_string(), fuel));
    
        while let Some((name, mut qty)) = queue.pop_front() {
            if name == "ORE" {
                sum += qty;
                continue;
            }
    
            if let Some(leftover) = leftovers.get_mut(&name) {
                let used = qty.min(*leftover);
                *leftover -= used;
                qty -= used;
            }
    
            let (min_qty, rqd) = self.0[&name].clone();
            let times_to_run = (qty + min_qty - 1) / min_qty;
    
            for (n, q) in rqd {
                let total_q = q * times_to_run;
                queue.push_back((n, total_q));
            }
    
            let leftover = times_to_run * min_qty - qty;
            if leftover > 0 {
                leftovers.entry(name).and_modify(|e| *e += leftover).or_insert(leftover);
            }
        }
    
        sum
    }    
}

fn parse_input() -> List {
    let input = include_str!("input14.txt");
    let mut list = List::new();

    for line in input.lines() {
        let (from, to) = line.split_once(" => ").unwrap();
        let (to_value, to_key) = to.split_once(" ").unwrap();

        let reactants = from.split(", ").filter_map(|s| {
            s.split_once(' ').map(|(qty, name)| (name.to_string(), qty.parse::<u64>().unwrap()))
        }).collect::<Vec<_>>();

        list.0.entry(to_key.to_string())
            .or_insert_with(|| (to_value.parse::<u64>().unwrap(), Vec::new()))
            .1.extend(reactants);
    }
    list
}

fn part1() -> u64 {
    let list = parse_input();
    list.find_ore(1)
}

fn part2() -> u64 {
    let list = parse_input();
    let ore_per_fuel = list.find_ore(1);
    let mut min = 0;
    let mut max = (MINED / ore_per_fuel) * 2;

    while min < max {
        let mid = min + (max - min) / 2;
        let output = list.find_ore(mid);

        if output > MINED {
            max = mid;
        } else {
            min = mid + 1;
        }
    }

    min - 1
}
fn main() {
    println!("{}", part2());
}