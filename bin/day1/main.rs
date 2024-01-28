fn part1(input: &str) -> u32 {
    let result: u32 = input
    .lines()
    .map(|line| {
        let mass = line.parse::<u32>().unwrap();
        mass / 3 - 2
    }).sum();
    result
}
fn part2(input: &str) -> u32 {
    let result: u32 = input
    .lines()
    .map(|line| line.parse::<u32>().unwrap())
    .fold(0, |mut fuel, mut x| {
        while x >= 9 {
            x = x / 3 - 2;
            fuel += x;
        }
        fuel
    });
    result
}
fn main() {
    let input = include_str!("input1.txt");
    println!("{}, {}", part1(input), part2(input));
}