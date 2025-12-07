use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents =
        fs::read_to_string(args[1].clone()).expect("Should have been able to read file input.");

    let data = parse(contents);

    // Solve.
    let solution: usize = part1(&data);
    println!("Solution to puzzle 5 part 1: {}", solution);

    let solution: usize = part2(&data);
    println!("Solution to puzzle 5 part 2: {}", solution);
}

struct Data {
    ranges: Vec<Vec<usize>>,
    ids: Vec<usize>,
}

fn parse(contents: String) -> Data {
    let mut ranges: Vec<Vec<usize>> = Vec::new();
    let mut ids: Vec<usize> = Vec::new();

    let mut is_range = true;
    let lines = contents.split('\n').collect::<Vec<&str>>();
    for line in lines {
        if line == "" {
            is_range = false;
        } else if is_range {
            let range = line
                .split('-')
                .map(|v| v.parse().unwrap())
                .collect::<Vec<usize>>();
            ranges.push(range);
        } else {
            let id = line.parse().unwrap();
            ids.push(id);
        }
    }

    ranges.sort_by(|a, b| a[0].cmp(&b[0]));

    return Data {
        ranges: ranges,
        ids: ids,
    };
}

fn part1(data: &Data) -> usize {
    println!("Solving puzzle 5 part 1: Spoils of christmas");

    // Just stupid iteration
    let mut total = 0;
    // let mut valid_ids: Vec<usize> = Vec::new();
    for id in &data.ids {
        for range in &data.ranges {
            if (id >= &range[0]) && (id <= &range[1]) {
                total += 1;
                break;
            }
        }
    }
    return total;
}

fn part2(data: &Data) -> usize {
    println!("Solving puzzle 5 part 2: Spoils on");

    let mut ends: HashSet<usize> = HashSet::new();

    let mut total = 0;
    let mut count = 0;

    for r1 in 0..data.ranges.len() {
        let range = &data.ranges[r1];

        // Create edges that turn on and off. Turn on current segment.
        let mut edges = vec![(range[0], true), (range[1], false)];

        // Turn off all segments below.
        for r2 in r1 + 1..data.ranges.len() {
            let subtract = &data.ranges[r2];
            edges.push((subtract[0], false));
            edges.push((subtract[1], true));
        }

        // Sort edges.
        edges.sort_by(|a, b| a.0.cmp(&b.0));

        // Add all sub-segements whenever it is turned on.
        let mut on: isize = 0;
        let mut previous = edges[0].0;
        for edge in edges {
            if on > 0 {
                // When on, split length = edge.0 - previous - 1 + # unique edges.

                total += edge.0 - previous;
                if edge.0 - previous > 0 {
                    // Stupid way to handle underflow form the - 1 contribution.
                    count += 1;
                }
                // Add unique edges.
                ends.insert(previous);
                ends.insert(edge.0);
            }
            // Switch on/off.
            on += match edge.1 {
                true => 1,
                false => -1,
            };
            previous = edge.0;
        }
    }
    // Finish the calculation by removing -1s and adding the # unique edges.
    total += ends.len() - count;

    return total;
}
