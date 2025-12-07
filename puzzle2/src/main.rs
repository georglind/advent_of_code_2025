use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents =
        fs::read_to_string(args[1].clone()).expect("Should have been able to read file input.");

    let ranges = contents.split(',').collect::<Vec<&str>>();

    // Solve.
    let solution: usize = part1(ranges.clone());
    println!("Solution to puzzle 2 part 1: {}", solution);

    let solution: usize = part2(ranges);
    println!("Solution to puzzle 2 part 2: {}", solution);
}

fn part1(parts: Vec<&str>) -> usize {
    println!("Solving part 1 - The invalid ids");

    let mut total = 0;
    for part in parts {
        println!("Part: {}", part);
        let range = part.split('-').collect::<Vec<&str>>();
        let bottom: usize = range[0].parse().expect("Should be number?");
        let top: usize = range[1].parse().expect("Should be number?");
        for n in bottom..=top {
            if is_invalid1(n.to_string().as_str()) {
                println!("  {}", n);
                total += n;
            }
        }
    }

    return total;
}

fn is_invalid1(value: &str) -> bool {
    // Length must be even.
    if value.len() % 2 == 1 {
        return false;
    }

    // Compare halves.
    let half_length = value.len() / 2;
    return value[0..half_length] == value[half_length..];
}

fn part2(parts: Vec<&str>) -> usize {
    println!("Solving part 2 - More invalid ids");

    let mut total = 0;
    for part in parts {
        println!("Part: {}", part);
        let range = part.split('-').collect::<Vec<&str>>();
        let bottom: usize = range[0].parse().expect("Should be number?");
        let top: usize = range[1].parse().expect("Should be number?");
        for n in bottom..=top {
            if is_invalid2(n.to_string().as_str()) {
                println!("  {}", n);
                total += n;
            }
        }
    }

    return total;
}

fn is_invalid2(value: &str) -> bool {
    for count in 2..=value.len() {
        if is_invalid_count(value, count) {
            return true;
        }
    }
    return false;
}

fn is_invalid_count(value: &str, count: usize) -> bool {
    // Length must be commensurate with number of divisoins (count).
    if value.len() % count != 0 {
        return false;
    }

    // Compare all parts.
    let length = value.len() / count;
    for n in 1..count {
        if value[0..length] != value[n * length..(n + 1) * length] {
            return false;
        }
    }
    return true;
}
