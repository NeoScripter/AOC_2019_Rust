use itertools::Itertools;
use std::fmt;

struct Message(Vec<char>);

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for chunk in self.0.chunks(25) {
            for &num in chunk {
                write!(f, "{: >0} ", num)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
const HEIGHT: usize = 6;
const WIDTH: usize = 25;

fn part1(input: &str) -> usize {
    let lenght = HEIGHT * WIDTH;
    let layers: Vec<Vec<u32>> = input.chars().filter_map(|c| c.to_digit(10)).chunks(lenght).into_iter().map(|c| c.collect()).collect();
    let min_layer = layers.iter().min_by_key(|vec| vec.iter().filter(|&&v| v == 0).count()).unwrap();
    let ones = min_layer.iter().filter(|&&v| v == 1).count();
    let twos = min_layer.iter().filter(|&&v| v == 2).count();
    ones * twos
}
fn part2(input: &str) -> Vec<char> {
    let lenght = HEIGHT * WIDTH;
    let layers: Vec<Vec<u32>> = input.chars().filter_map(|c| c.to_digit(10)).chunks(lenght).into_iter().map(|c| c.collect()).collect();
    let mut final_image: Vec<char> = Vec::new();
    'outer: for idx in 0..lenght {
        for layer in layers.iter() {
            let pixel = match layer[idx] {
                0 => '.', 
                1 => '#',
                _ => continue,
            };
            final_image.push(pixel);
            continue 'outer;
        }
    }
    final_image
}
fn main() {
    let input = include_str!("input8.txt");
    let final_image = part2(input);
    println!("{}", part1(input));
    println!("{}", Message(final_image));
}