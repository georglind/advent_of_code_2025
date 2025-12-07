use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents =
        fs::read_to_string(args[1].clone()).expect("Should have been able to read file input.");

    let ranges = contents.split('\n').collect::<Vec<&str>>();

    // Solve.
    let solution: usize = part1(ranges.clone());
    println!("Solution to puzzle 3 part 1: {}", solution);

    let solution: usize = part2(ranges);
    println!("Solution to puzzle 3 part 2: {}", solution);
}

fn part1(lines: Vec<&str>) -> usize {
    println!("Solving puzzle 3 part 1: High joltage");

    let mut total = 0;
    for line in lines {
        let numbers = line
            .chars()
            .map(|s| s.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        let p1 = find_max_pos(&numbers[..numbers.len() - 1]);
        let p2 = find_max_pos(&numbers[p1 + 1..]);

        let joltage = numbers[p1] * 10 + numbers[p2 + p1 + 1];

        println!("{}: {} joltage", line, joltage);

        total += joltage
    }
    return total;
}

fn find_max_pos(numbers: &[usize]) -> usize {
    let mut max = numbers[0];
    let mut point = 0;
    for n in 0..numbers.len() {
        if numbers[n] > max {
            max = numbers[n];
            point = n;
        }
    }
    return point;
}

fn part2(lines: Vec<&str>) -> usize {
    println!("Solving puzzle 3 part 2: Higher joltage");

    let mut total = 0;
    for line in lines {
        let numbers = line
            .chars()
            .map(|s| s.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        let mut joltage: usize = 0;
        let mut p: usize = 0;
        for i in 0..12 {
            let q = find_max_pos(&numbers[p..numbers.len() - 11 + i]);
            joltage += numbers[p + q] * (10 as usize).pow(11 - i as u32);
            p = p + q + 1;
        }

        println!("{}: {} joltage", line, joltage);

        total += joltage
    }
    return total;
}
