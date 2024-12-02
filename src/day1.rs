use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Column {
    ids: Vec<i64>,
    min: i64,
    counts: HashMap<i64, i64>
}

pub fn problem1() -> i64 {
    println!("hi");
    let (col1_ids, col2_ids) = parse_file("src/day1.txt");
    let col1_counts = create_counts(&col1_ids);
    let col2_counts = create_counts(&col2_ids);

    let col1_min = std::i64::MAX;
    let col2_min = std::i64::MAX;

    let mut col1 = Column { ids: col1_ids, min: col1_min, counts: col1_counts };
    let mut col2 = Column { ids: col2_ids, min: col2_min, counts: col2_counts };

    let mut distance = 0;
    while col1.ids.len() > 0 {
        let x;
        let y;
        (x, y, col1, col2) = find_next_smallest_pair(col1, col2);
        col1.min = x;
        col2.min = y;
        println!("{}, {}", x, y);
        distance += (x - y).abs();
    }
    distance
}

pub fn problem2() -> i64 {
    let (col1_ids, col2_ids) = parse_file("src/day1.txt");
    let col1_counts = create_counts(&col1_ids);
    let col2_counts = create_counts(&col2_ids);

    let col1_min = std::i64::MAX;
    let col2_min = std::i64::MAX;

    let col1 = Column { ids: col1_ids, min: col1_min, counts: col1_counts };
    let col2 = Column { ids: col2_ids, min: col2_min, counts: col2_counts };

    let mut similarity = 0;
    for id in col1.ids {
        similarity += id * *(col2.counts.get(&id)).unwrap_or(&0);
    }

    similarity
}

fn find_next_smallest_pair(mut col1: Column, mut col2: Column) -> (i64, i64, Column, Column) {
    // You could skip some iterations by using the hashmap (not currently done)
    let mut col1_current_smallest: i64 = std::i64::MAX;
    let mut col2_current_smallest: i64 = std::i64::MAX;
    let mut col1_smallest_index: Option<usize> = None;
    let mut col2_smallest_index: Option<usize> = None;
    for row in 0..col1.ids.len() {
        if col1.ids[row] < col1_current_smallest {
            col1_current_smallest = col1.ids[row];
            col1_smallest_index = Some(row);
        }

        if col2.ids[row] < col2_current_smallest {
            col2_current_smallest = col2.ids[row];
            col2_smallest_index = Some(row);
        }
    }

    match col1_smallest_index {
        Some(i) => col1.ids.remove(i),
        None => panic!("No smallest number found for column 1"),
    };
    match col2_smallest_index {
        Some(i) => col2.ids.remove(i),
        None => panic!("No smallest number found for column 2"),
    };

    (col1_current_smallest, col2_current_smallest, col1, col2)
}

fn create_counts(column: &Vec<i64>) -> HashMap<i64, i64> {
    let mut counts: HashMap<i64, i64> = HashMap::new();
    for id in column {
        match counts.get(&id) {
            Some(f) => {
                counts.insert(*id, f + 1);
            },
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
        let mut numbers = line.split("   ");
        first_col.push(numbers.next().expect("No number in row").parse::<i64>().expect("Could not parse first number"));
        second_col.push(numbers.next().expect("No second number in row").parse::<i64>().expect("Could not parse first number"));
    }
    (first_col, second_col)

}