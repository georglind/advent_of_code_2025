use rand::seq::SliceRandom;
use std::cmp::min;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents =
        fs::read_to_string(args[1].clone()).expect("Should have been able to read file input.");

    let lines = contents.split('\n').collect::<Vec<&str>>();

    let points = parse(lines.clone());

    // The gridding cutoff (depending on the problem).
    let gridding: f32 = args[2].clone().parse().unwrap();

    // Cap cutoff at max.
    let cutoff: usize = min(
        args[3].clone().parse().unwrap(),
        points.len() * (points.len() - 1) / 2 as usize,
    );

    // Solve.
    let solution: usize = part1(points.clone(), gridding, cutoff);
    println!("Solution to puzzle 8 part 1: {}", solution);

    // The gridding cutoff (depending on the problem).
    let gridding: f32 = args[4].clone().parse().unwrap();

    // Cap cutoff at max.
    let cutoff: usize = points.len() * (points.len() - 1) / 2 as usize;

    // Solve.
    let solution: isize = part2(points, gridding, cutoff);
    println!("Solution to puzzle 8 part 2: {}", solution);
}

// For fun, let us see if we can do something slightly optimized.
#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
struct Point(isize, isize, isize);

impl Point {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self(x, y, z)
    }

    pub fn from_vec(vec: Vec<isize>) -> Self {
        Self(vec[0], vec[1], vec[2])
    }

    pub fn rounded(&self, distance: f32) -> Point {
        Self(
            (self.0 as f32 / distance) as isize,
            (self.1 as f32 / distance) as isize,
            (self.2 as f32 / distance) as isize,
        )
    }

    pub fn distance(self, other: &Self) -> f32 {
        let d: f32 = ((self.0 - other.0).pow(2)
            + (self.1 - other.1).pow(2)
            + (self.2 - other.2).pow(2)) as f32;

        return d.sqrt();
    }

    pub fn displaced(self, x: isize, y: isize, z: isize) -> Self {
        Self(self.0 + x, self.1 + y, self.2 + z)
    }
}

fn parse(lines: Vec<&str>) -> Vec<Point> {
    // Could allocate as we know the size.
    let mut points: Vec<Point> = Vec::new();

    for line in lines {
        let value = line
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<isize>>();
        points.push(Point::from_vec(value));
    }

    return points;
}

#[allow(dead_code)]
fn guess_gridding(points: &Vec<Point>, sample_size: usize) -> f32 {
    // Random pairs. Crude.
    let sample: Vec<_> = points
        .choose_multiple(&mut rand::thread_rng(), 2 * sample_size)
        .collect();

    // Lowest value.
    let mut d: f32 = f32::INFINITY;
    for i in 0..sample_size {
        let dd: f32 = sample[i].distance(sample[i + sample_size]);
        if dd < d {
            d = dd;
        }
    }
    return d;
}

// Could probably be made recursive.
#[allow(dead_code)]
fn daq(points: Vec<Point>, distance: f32) -> (f32, (Point, Point)) {
    // Build lookup.
    let mut lookup: HashMap<Point, Vec<Point>> = HashMap::new();
    for point in points {
        let gridded = point.rounded(distance);
        lookup
            .entry(gridded)
            .or_insert(Vec::new())
            .push(point.clone());
    }

    // Remember minimum distance.
    let mut d = distance.clone();
    let mut pair: (Point, Point) = (Point::new(0, 0, 0), Point::new(0, 0, 0));

    // Iterate and check all distances in Moore neighborhood.
    for (gridded, grid_points) in &lookup {
        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    let other = gridded.displaced(i, j, k);
                    if !lookup.contains_key(&other) {
                        continue;
                    }
                    // Use some ordering in order to not double calculate when hitting other.
                    if gridded.0 + 10 * gridded.1 + 100 * gridded.2
                        > other.0 + 10 * other.1 + 100 * other.2
                    {
                        continue;
                    }
                    for gp in grid_points {
                        for op in lookup.get(&other).unwrap() {
                            if gp == op {
                                continue;
                            }
                            let god = gp.distance(op);
                            if god < d {
                                pair = (gp.clone(), op.clone());
                                d = god;
                            }
                        }
                    }
                }
            }
        }
    }
    return (d, pair);
}

// Could probably be made recursive.
fn closest_candidates(points: &Vec<Point>, distance: f32, cutoff: usize) -> Vec<(usize, usize)> {
    // Build lookup.
    let mut lookup: HashMap<Point, Vec<usize>> = HashMap::new();
    for (index, point) in points.iter().enumerate() {
        let gridded = point.rounded(distance);
        lookup.entry(gridded).or_insert(Vec::new()).push(index);
    }

    // Remember minimum distance.
    let mut pairs: Vec<(usize, usize)> = Vec::new();

    // Iterate and check all distances in Moore neighborhood.
    for (gridded, grid_points) in &lookup {
        // Add all paris within box.
        for i in 0..grid_points.len() {
            for j in i + 1..grid_points.len() {
                pairs.push((grid_points[i], grid_points[j]));
            }
        }

        for i in -1..=1 {
            for j in -1..=1 {
                for k in -1..=1 {
                    let other = gridded.displaced(i, j, k);
                    // No grid there.
                    if !lookup.contains_key(&other) {
                        continue;
                    }
                    // Use some ordering in order to not double calculate when hitting in reverse.
                    if gridded.0 + 10 * gridded.1 + 100 * gridded.2
                        >= other.0 + 10 * other.1 + 100 * other.2
                    {
                        continue;
                    }
                    for gi in grid_points {
                        for oi in lookup.get(&other).unwrap() {
                            pairs.push((*gi, *oi));
                        }
                    }
                    if pairs.len() > cutoff {
                        return pairs;
                    }
                }
            }
        }
    }
    return pairs;
}

fn recolor(markers: &mut Vec<usize>, color: usize, recolor: usize) {
    for index in 0..markers.len() {
        if markers[index] == color {
            markers[index] = recolor.clone();
        }
    }
}

fn part1(points: Vec<Point>, gridding: f32, cutoff: usize) -> usize {
    println!("Solving puzzle 8 part 1: Now it gets fun");

    // One can easily brute force this, but the gridding was more fun.
    // let mut cs = Vec::new();
    // for i in 0..points.len() {
    //     for j in i + 1..points.len() {
    //         cs.push((i, j));
    //     }
    // }

    let cs = closest_candidates(&points, gridding, 100 * cutoff);

    let mut paired_distances: Vec<(usize, usize, f32)> = Vec::with_capacity(cs.len());
    for (i1, i2) in cs {
        let dd = points[i1].distance(&points[i2]);
        paired_distances.push((i1, i2, dd));
    }

    paired_distances.sort_by(|a, b| a.2.total_cmp(&b.2));

    let mut markers: Vec<usize> = vec![0; points.len()];
    let mut current_color = 0;

    for index in 0..cutoff {
        if paired_distances[index].2 > gridding {
            panic!(
                "Increase precision. The gridding {} is too small.",
                gridding
            )
        }

        let a = paired_distances[index].0;
        let b = paired_distances[index].1;

        // Two ends are in same circuit -> skip connection.
        if markers[a] > 0 && markers[a] == markers[b] {
            continue;
        }

        // Connect the rest.
        if markers[a] > 0 && markers[b] > 0 {
            let colora = markers[a];
            let colorb = markers[b];
            recolor(&mut markers, colora, colorb);
        } else if markers[a] > 0 {
            markers[b] = markers[a];
        } else if markers[b] > 0 {
            markers[a] = markers[b];
        } else {
            current_color += 1;
            markers[a] = current_color;
            markers[b] = current_color;
        }
    }

    let mut subgraphs: Vec<usize> = vec![0; current_color + 1];

    for i in 0..markers.len() {
        if markers[i] > 0 {
            subgraphs[markers[i]] += 1;
        }
    }
    subgraphs.sort();
    subgraphs.reverse();

    println!("There are {} circuits", subgraphs.len());

    return subgraphs[0] * subgraphs[1] * subgraphs[2];
}

fn part2(points: Vec<Point>, gridding: f32, cutoff: usize) -> isize {
    println!("Solving puzzle 8 part 2: So fun");

    // One can easily brute force this, but the gridding was more fun.
    // let mut cs: Vec<(usize, usize)> = Vec::new();
    // for i in 0..points.len() {
    //     for j in i + 1..points.len() {
    //         cs.push((i, j));
    //     }
    // }

    let cs = closest_candidates(&points, gridding, 100 * cutoff);

    let mut paired_distances: Vec<(usize, usize, f32)> = Vec::with_capacity(cs.len());
    for (i1, i2) in cs {
        let dd = points[i1].distance(&points[i2]);
        paired_distances.push((i1, i2, dd));
    }

    paired_distances.sort_by(|a, b| a.2.total_cmp(&b.2));

    let mut markers: Vec<usize> = vec![0; points.len()];
    let mut current_color = 0;

    let mut index = 0;
    loop {
        if paired_distances[index].2 > gridding {
            panic!(
                "Increase precision. The gridding {} is too small.",
                gridding
            )
        }

        let a = paired_distances[index].0;
        let b = paired_distances[index].1;

        index += 1;

        // Two ends are in same circuit -> skip connection.
        if markers[a] > 0 && markers[a] == markers[b] {
            continue;
        }

        // Connect the rest.
        if markers[a] > 0 && markers[b] > 0 {
            let colora = markers[a];
            let colorb = markers[b];
            recolor(&mut markers, colora, colorb);
        } else if markers[a] > 0 {
            markers[b] = markers[a];
        } else if markers[b] > 0 {
            markers[a] = markers[b];
        } else {
            current_color += 1;
            markers[a] = current_color;
            markers[b] = current_color;
        }

        // Abort if we are one big circuit.
        if markers.iter().all(|&item| item == markers[0]) {
            break;
        }
    }
    return points[paired_distances[index - 1].0].0 * points[paired_distances[index - 1].1].0;
}
