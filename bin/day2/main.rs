use std::str::FromStr;

fn output(input_v: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut input = input_v.clone();
    input[1] = noun;
    input[2] = verb;

    let mut pos: usize = 0;
    loop {
        let op = input[pos];
        let a = input[pos + 1];
        let b = input[pos + 2];
        let c = input[pos + 3];

        match op {
            1 => { input[c] = input[a] + input[b]; }
            2 => { input[c] = input[a] * input[b]; }
            99 => { break; }
            _ => { unreachable!(); }
        }
        pos += 4;
    }
    input[0]
}

fn main() {
    let input: Vec<usize> = include_str!("input2.txt")
        .split(",")
        .filter_map(|d| usize::from_str(d).ok())
        .collect();

        println!("part 1 {:?}", output(&input, 12, 2));

        for noun in 0..100 {
            for verb in 0..100 {
                if output(&input, noun, verb) == 19690720 {
                    println!("part 2 {:?}", 100 * noun + verb);
                    break;
                }
            }
        }
}
