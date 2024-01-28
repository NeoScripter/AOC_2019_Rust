#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_variables)]
use itertools::Itertools;
use std::collections::HashMap;

const REPS: usize = 10000;

#[derive(Debug, Clone)]
struct FFT {
    output: Vec<i32>,
    pattern: Vec<i32>,
}

impl FFT {
    fn new(input: &str) -> Self {
        let output: Vec<i32> = input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        FFT { output, pattern: vec![0, 1, 0, -1] }
    }

    fn phase(&mut self) {
        let mut new_output = Vec::with_capacity(self.output.len());
        
        for i in 0..self.output.len() {
            let pattern: Vec<i32> = if i == 0 {
                self.pattern.iter().cloned().cycle().skip(1).take(self.output.len()).collect()
            } else {
                self.pattern.iter().flat_map(|&x| vec![x; i + 1]).cycle().skip(1).take(self.output.len()).collect()
            };
            
            let n: i32 = self.output.iter().zip(pattern.iter()).map(|(&a, &b)| a * b).sum();
            new_output.push((n.abs() % 10) as i32);
        }
        
        self.output = new_output;
    }

    fn running_total(&mut self) {
        let mut total = 0;
        self.output = self.output.iter().rev().map(|&x| {
            total = (total + x) % 10;
            total
        }).collect();
        self.output.reverse();
    }

    fn merge(&self) -> String {
        self.output.iter().take(8).filter_map(|&d| char::from_digit(d as u32, 10)).collect::<String>()
    }
}

fn part1() -> String {
    let input = include_str!("input_lib.txt");
    let mut fft = FFT::new(input);
    for _ in 0..100 { fft.phase(); }
    fft.merge()
}

fn part2() -> String {
    let input = include_str!("input_lib.txt");
    let mut fft = FFT::new(input);
    let offset = fft.output.iter().take(7).fold(0, |acc, &d| acc * 10 + d as usize);

    fft.output = fft.output.iter().cycle().take(fft.output.len() * REPS).skip(offset).cloned().collect();

    for _ in 0..100 { fft.running_total(); }
    fft.merge()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("78725270".to_string(), part2());
    }
}