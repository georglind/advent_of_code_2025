use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents =
        fs::read_to_string(args[1].clone()).expect("Should have been able to read file input.");

    let lines = contents.split('\n').collect::<Vec<&str>>();

    // Solve.
    let solution: usize = part1(lines.clone());
    println!("Solution to puzzle 7 part 1: {}", solution);

    let solution: usize = part2(lines);
    println!("Solution to puzzle 7 part 2: {}", solution);
}

fn part1(lines: Vec<&str>) -> usize {
    println!("Solving puzzle 7 part 1: Tachyon manyfolds");

    let mut beams: Vec<bool> = vec![false; lines[0].len()];

    let mut total = 0;
    for line in lines {
        for (index, char) in line.chars().enumerate() {
            if beams[index] {
                if char == '^' {
                    total += 1;
                    beams[index - 1] = true;
                    beams[index + 1] = true;
                    beams[index] = false;
                }
            }
            if char == 'S' {
                beams[index] = true;
            }
        }
    }
    return total;
}

fn part2(lines: Vec<&str>) -> usize {
    println!("Solving puzzle 7 part 2: Quantum tachyon many-worlds");

    let mut beams: Vec<usize> = vec![0; lines[0].len()];

    for line in lines {
        for (index, char) in line.chars().enumerate() {
            if beams[index] > 0 {
                if char == '^' {
                    beams[index - 1] += beams[index];
                    beams[index + 1] += beams[index];
                    beams[index] = 0;
                }
            }
            if char == 'S' {
                beams[index] = 1;
            }
        }
    }
    return beams.into_iter().sum();
}
