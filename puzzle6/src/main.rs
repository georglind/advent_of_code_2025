use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents =
        fs::read_to_string(args[1].clone()).expect("Should have been able to read file input.");
    // Solve.
    let solution: usize = part1(contents.clone());
    println!("Solution to puzzle 6 part 1: {}", solution);

    let solution: usize = part2(contents);
    println!("Solution to puzzle 6 part 2: {}", solution);
}

struct Data1 {
    numbers: Vec<Vec<usize>>,
    operations: Vec<char>,
}

fn parse1(contents: String) -> Data1 {
    let mut numbers: Vec<Vec<usize>> = Vec::new();
    let operations: Vec<char>;

    let lines = contents.split('\n').collect::<Vec<&str>>();

    for line in &lines[..lines.len() - 1] {
        numbers.push(
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    operations = lines[lines.len() - 1]
        .split_whitespace()
        .map(|c| c.chars().next().unwrap())
        .collect();

    return Data1 {
        numbers: numbers,
        operations: operations,
    };
}

fn part1(contents: String) -> usize {
    println!("Solving puzzle 6 part 1: Math homework");

    let data = parse1(contents);

    let mut total = 0;
    for i in 0..data.operations.len() {
        let mut res = data.numbers[0][i];
        for row in &data.numbers[1..] {
            if data.operations[i] == '*' {
                res *= row[i];
            } else {
                res += row[i];
            }
        }
        total += res;
    }

    return total;
}

fn part2(contents: String) -> usize {
    println!("Solving puzzle 6 part 2: Inverted math homework");

    // transpose the contents.
    let lines: Vec<Vec<char>> = contents
        .split('\n')
        .map(|c| c.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Mainly for my sanity. We could probably just iterate accordingly.
    let mut transposed: Vec<Vec<char>> = vec![vec![' '; lines.len()]; lines[0].len()];
    for row in 0..lines.len() {
        for column in 0..lines[row].len() {
            transposed[column][row] = lines[row][column];
        }
    }

    let mut total = 0;

    let mut operation = '*';
    let mut ds: Vec<usize> = Vec::new();
    for column in 0..transposed.len() {
        let c = &transposed[column];

        // Update operation.
        operation = match c[c.len() - 1] {
            '*' => '*',
            '+' => '+',
            _ => operation,
        };

        // A column of ' ' -> Finish the calculation.
        if c.iter().all(|c| *c == ' ') {
            dbg!(&ds, operation);
            total += calc(&ds, operation);
            ds.clear();
        } else {
            let d: usize = c[0..c.len() - 1]
                .into_iter()
                .collect::<String>()
                .trim()
                .parse()
                .unwrap();

            ds.push(d);
        }
    }

    // Remember to add the last calculation
    return total + calc(&ds, operation);
}

// Simple helper for reducing vector using operation
fn calc(ds: &Vec<usize>, operation: char) -> usize {
    ds.clone()
        .into_iter()
        .reduce(|a, b| match operation {
            '*' => a * b,
            '+' => a + b,
            _ => panic!("uhoh!"),
        })
        .unwrap()
}
