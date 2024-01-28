

fn part1(input: &str) -> usize {
    let (start, end) = input.split_once("-").unwrap();
    let start = start.parse::<u32>().unwrap();
    let end = end.parse::<u32>().unwrap();
    (start..=end).filter(|num| {
        let digits: Vec<u8> = num.to_string().bytes().map(|b| b - b'0').collect();
        let has_adjacent_same_digits = digits.windows(2).any(|window| window[0] == window[1]);
        let non_decreasing = digits.windows(2).all(|window| window[0] <= window[1]);
        has_adjacent_same_digits && non_decreasing
    }).count()
}

fn part2(input: &str) -> usize {
    let (start, end) = input.split_once("-").unwrap();
    let start = start.parse::<u32>().unwrap();
    let end = end.parse::<u32>().unwrap();
    (start..=end).filter(|num| {
        let digits: Vec<u8> = num.to_string().bytes().map(|b| b - b'0').collect();
        let non_decreasing = digits.windows(2).all(|window| window[0] <= window[1]);

        let mut counts = [0; 10];
        for &digit in &digits {
            counts[digit as usize] += 1;
        }
        let has_isolated_pair = counts.iter().any(|&count| count == 2);

        non_decreasing && has_isolated_pair
    }).count()
}
fn main() {
    let input = "136760-595730";
    println!("{}, {}", part1(input), part2(input));
}