use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents =
        fs::read_to_string(args[1].clone()).expect("Should have been able to read file input.");

    let data = parse(contents);

    // Solve.
    let solution: usize = part1(data.clone());
    println!("Solution to puzzle 4 part 1: {}", solution);

    let solution: usize = part2(data);
    println!("Solution to puzzle 4 part 2: {}", solution);
}

// Parse and pad data.
fn parse(contents: String) -> Vec<Vec<bool>> {
    let ranges = contents.split('\n').collect::<Vec<&str>>();

    let mut data = vec![vec![false; ranges[0].len() + 2]; ranges.len() + 2];

    for (row, range) in ranges.iter().enumerate() {
        for (column, char) in range.chars().enumerate() {
            if char == '@' {
                data[row + 1][column + 1] = true;
            }
        }
    }

    return data;
}

fn display(data: Vec<Vec<char>>) {
    for row in 0..data.len() {
        for column in 0..data[row].len() {
            print!("{}", data[row][column]);
        }
        println!("");
    }
}

fn part1(data: Vec<Vec<bool>>) -> usize {
    println!("Solving puzzle 4 part 1: Truckers");

    // For display.
    let mut results = vec![vec!['.'; data[0].len()]; data.len()];

    let mut total = 0;

    for row in 0..data.len() {
        for column in 0..data[row].len() {
            if data[row][column] {
                let mut count = 0;
                for dr in row - 1..=row + 1 {
                    for dc in column - 1..=column + 1 {
                        count += data[dr][dc] as usize;
                    }
                }
                if count - 1 < 4 {
                    results[row][column] = 'X';
                    total += 1;
                } else {
                    results[row][column] = '@';
                }
            }
        }
    }

    display(results);

    return total;
}

fn removables(data: &Vec<Vec<bool>>) -> Vec<Vec<usize>> {
    let mut coords = Vec::new();

    for row in 0..data.len() {
        for column in 0..data[row].len() {
            if data[row][column] {
                let mut count = 0;
                for dr in row - 1..=row + 1 {
                    for dc in column - 1..=column + 1 {
                        count += data[dr][dc] as usize;
                    }
                }
                if count - 1 < 4 {
                    coords.push(vec![row, column]);
                }
            }
        }
    }
    return coords;
}

fn part2(mut data: Vec<Vec<bool>>) -> usize {
    println!("Solving puzzle 4 part 2: Tuck on");

    let mut total = 0;
    loop {
        let coords = removables(&data);
        if coords.len() == 0 {
            break;
        }

        total += coords.len();

        for coord in coords {
            data[coord[0]][coord[1]] = false;
        }
    }

    // Display
    let mut results = vec![vec!['.'; data[0].len()]; data.len()];
    for row in 0..data.len() {
        for column in 0..data[row].len() {
            if data[row][column] {
                results[row][column] = '@';
            }
        }
    }
    display(results);

    return total;
}
