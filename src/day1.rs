use std::{collections::HashMap, fs::read_to_string, sync::Arc};

pub fn problem1() -> i64 {
    let (mut col1_ids, mut col2_ids) = parse_file("src/day1.txt");

    let mut distance = 0;
    col1_ids.sort();
    col2_ids.sort();
    for i in 0..col1_ids.len() {
        distance += ((col1_ids[i] - col2_ids[i]) as i64).abs();
    }

    // Another way to do it
    // distance = col1_ids
    //     .iter()
    //     .zip(col2_ids.iter())
    //     .map(|(x, y)| (x - y).abs())
    //     .sum();

    distance
}

pub fn problem2() -> usize {
    let mut similarity = 0;

    let (col1_ids, col2_ids) = parse_file("src/day1.txt");
    // let col2_counts = create_counts(&col2_ids);

    // for id in col1_ids {
    //     similarity += id * *(col2_counts.get(&id)).unwrap_or(&0);
    // }

    // let col2_ids_iter = col2_ids.iter();

    similarity = col1_ids.iter().map(|x| {
        x * col2_ids.iter().filter(|n| *n == x).count()
    }
    ).sum();

    similarity
}

fn create_counts(column: &Vec<usize>) -> HashMap<usize, usize> {
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for id in column {
        match counts.get(&id) {
            Some(f) => {
                counts.insert(*id, f + 1);
            }
            None => {
                counts.insert(*id, 1);
            }
        }
    }
    counts
}

fn parse_file(filename: &str) -> (Vec<usize>, Vec<usize>) {
    let mut first_col = Vec::new();
    let mut second_col = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let mut numbers = line.split_whitespace();
        first_col.push(
            numbers
                .next()
                .expect("No first number in row")
                .parse::<usize>()
                .expect("Could not parse first number"),
        );
        second_col.push(
            numbers
                .next()
                .expect("No second number in row")
                .parse::<usize>()
                .expect("Could not parse second number"),
        );
    }
    (first_col, second_col)
}
