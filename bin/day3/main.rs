use std::collections::{HashSet, HashMap};

fn part1(input: &str) -> i32 {
    let commands: Vec<Vec<_>> = input.lines().map(|line| line.trim().split(",").collect()).collect();
    let mut all_moves = HashMap::new();

    for coms in &commands {
        let mut moves = HashSet::new();
        let mut coord: (i32, i32) = (0, 0);
        for com in coms {
            let (dir, steps) = com.split_at(1);
            let steps: i32 = steps.parse().expect("Invalid digit");
            for _ in 0..steps {
                match dir {
                    "R" => coord.0 += 1,
                    "L" => coord.0 -= 1,
                    "U" => coord.1 += 1,
                    "D" => coord.1 -= 1,
                    _ => {},
                }
                moves.insert(coord);
            }
        }

        for key in moves.drain() {
            *all_moves.entry(key).or_insert(0) += 1;
        }
    }

    let intersections: Vec<i32> = all_moves.iter()
        .filter_map(|(&(x, y), &value)| Some(x.abs() + y.abs()).filter(|_| value > 1))
        .collect();

    intersections.iter().min().copied().unwrap()
}
fn part2(input: &str) -> i32 {
    let commands: Vec<Vec<_>> = input.lines().map(|line| line.trim().split(",").collect()).collect();
    let mut common_values = Vec::new();
    let mut moves = [HashMap::new(), HashMap::new()];

    for (idx, coms) in commands.iter().enumerate() {
        let mut coord = (0, 0);
        let mut step_count = 0;
        for com in coms {
            let (dir, steps) = com.split_at(1);
            let steps: i32 = steps.parse().expect("Invalid digit");
            for _ in 0..steps {
                match dir {
                    "R" => coord.0 += 1,
                    "L" => coord.0 -= 1,
                    "U" => coord.1 += 1,
                    "D" => coord.1 -= 1,
                    _ => {}
                }
                step_count += 1;
                if !moves[idx].contains_key(&coord) {
                    moves[idx].insert(coord, step_count);
                }
            }
        }
    }
    for (&key, &value1) in &moves[0] {
        if let Some(&value2) = moves[1].get(&key) {
            common_values.push(value1 + value2);
        }
    }
    common_values.into_iter().min().unwrap()
}
fn main() {
    let input = include_str!("input3.txt");
    println!("{}, {}", part1(input), part2(input));
}