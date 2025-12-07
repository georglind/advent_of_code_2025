use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let solution: usize = part1();
    println!("Solution to puzzle 1 part 1: {}", solution);

    let solution: usize = part2();
    println!("Solution to puzzle 1 part 2: {}", solution);
}

fn part1() -> usize {
    println!("Solving part 1 - The safe password");

    let args: Vec<String> = env::args().collect();

    let mut count: usize = 0;
    let mut number: isize = 50;
    if let Ok(lines) = read_lines(args[1].clone()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let rotation: isize = line[1..].parse().unwrap();
            if line.starts_with("R") {
                number += rotation;
            } else {
                number -= rotation;
            }
            number = number % 100;
            if number == 0 {
                count += 1;
            }
        }
    }

    return count;
}

fn part2() -> usize {
    println!("Solving part 2 - The safer password");

    let args: Vec<String> = env::args().collect();

    let mut count: usize = 0;
    let mut number: isize = 50;

    if let Ok(lines) = read_lines(args[1].clone()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let rotation: isize = line[1..].parse().unwrap();

            let direction: isize = match line.get(0..1).unwrap() {
                "R" => 1,
                "L" => -1,
                _ => panic!("Invalid line"),
            };

            // Just count everything very stupidly.
            for _ in 1..=rotation {
                number += direction;
                number = number.rem_euclid(100);
                if number == 0 {
                    count += 1;
                }
            }

            println!("{}: {} {}", line, number, count);
        }
    }

    return count;
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
