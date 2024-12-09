use std::{collections::HashMap, fs::read_to_string};

pub fn problem1() -> i64 {
    let (mut col1, mut col2) = parse_file("src/day1/day1.txt");

    let mut distance = 0;
    col1.sort();
    col2.sort();
    for i in 0..col1.len() {
        distance += (col1[i] - col2[i]).abs();
    }

    // Another way to do it
    distance = std::iter::zip(col1, col2).map(|(x, y)| (x - y).abs()).sum();

    distance
}

pub fn problem2() -> usize {
    let mut similarity = 0;

    let (col1, col2) = parse_file("src/day1/day1.txt");
    let col2_counts = create_counts(&col2);

    // for id in col1{
    //     similarity += id * *(col2_counts.get(&id)).unwrap_or(&0);
    // }

    // println!("{}", similarity);

    similarity = col1
        .iter()
        .map(
            |x| {
                (*x as usize)
                    * col2
                        .iter()
                        .filter(|n| *n == x) // O(n)
                        .count()
            }, // O(n)
        )
        .sum();

    similarity
}

fn create_counts(column: &Vec<i64>) -> HashMap<i64, i64> {
    let mut counts: HashMap<i64, i64> = HashMap::new();
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

fn parse_file(filename: &str) -> (Vec<i64>, Vec<i64>) {
    let mut first_col = Vec::new();
    let mut second_col = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let mut numbers = line.split_whitespace();
        first_col.push(
            numbers
                .next()
                .expect("No first number in row")
                .parse::<i64>()
                .expect("Could not parse first number"),
        );
        second_col.push(
            numbers
                .next()
                .expect("No second number in row")
                .parse::<i64>()
                .expect("Could not parse second number"),
        );
    }
    (first_col, second_col)
}
