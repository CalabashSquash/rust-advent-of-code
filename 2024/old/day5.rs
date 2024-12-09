use std::{collections::HashMap, fs::read_to_string};

use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{char, digit0, i32, multispace0, newline},
    error::context,
    multi::{fold_many0, many0, separated_list1},
    number,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

pub fn problem1() -> i32 {
    let text = read_to_string("src/day5.txt").unwrap();

    let pairs: Vec<(i32, i32)>;
    let pair_map: HashMap<(i32, i32), bool>;
    let levels: Vec<Vec<i32>>;
    let mut sum = 0;

    match parsey_warsey(&text) {
        Ok((_, (_pairs, _levels))) => {
            pairs = _pairs;
            pair_map = pairs.into_iter().map(|a | {
                (a, true)
            }).collect();
            println!("pair_map: {:?}", pair_map);
            levels = _levels;
            for level in levels {
                // if check_level_v2(&level, &pair_map) {
                //     sum += level[level.len() / 2];
                // }
            }
        }
        Err(e) => {
            panic!("Error parsing: {}", e);
        }
    }

    // for level in levels {
    //     match check_level(&level, pairs.clone()) {
    //         (true, _) => {
    //             sum += level[level.len() / 2];
    //         }
    //         (false, _) => {}
    //     }
    // }
    sum
}

pub fn problem2() -> i32 {
    let text = read_to_string("src/day5.txt").unwrap();

    let pairs: Vec<(i32, i32)>;
    let levels: Vec<Vec<i32>>;
    let pair_map: HashMap<(i32,i32), bool>;
    match parsey_warsey(&text) {
        Ok((_, (_pairs, _levels))) => {
            pairs = _pairs;
            levels = _levels;
            pair_map = pairs.into_iter().map(|a| {
                (a, true)
            }).collect();
        }
        Err(e) => {
            panic!("Error parsing: {}", e);
        }
    }

    let mut sum = 0;
    for mut level in levels {
        // println!("OLD: {:?}", level);
        let mut was_level_fixed = false;
        loop {
            match check_level_v2(&level, &pair_map) {
                (true, _) => {
                    break;
                }
                (false, (a_index, b_index)) => {
                    println!("FALSE. a: {}, b: {}", level[a_index], level[b_index]);
                    println!("level: {:?}", level);
                    let a = level[a_index];
                    level[a_index] = level[b_index];
                    level[b_index] = a;
                    was_level_fixed = true;
                }
            }
        }
        // println!("NEW: {:?}", level);
        // println!("Middle value: {}", level[level.len() / 2]);
        // let (is_still_legit, _) = check_level_v2(&level, pairs.clone());
        if was_level_fixed {
            sum += level[level.len() / 2];
        }

        // if level_is_valid {
        //     println!("Middle value: {}", level[level.len() / 2]);
        //     sum += level[level.len() / 2];
        // } else {
        //     println!("INVALID");
        //     println!("OLD: {:?}", level);
        //     println!("NEW: {:?}", new_valid_level);
        //     println!("Middle value: {}", new_valid_level[level.len() / 2]);
        //     sum += level[level.len() / 2];
        // }

        // println!("is valid: . Level: {:?}", level);
    }
    sum
}

fn check_level_v2(level: &Vec<i32>, pairs: &HashMap<(i32,i32), bool>) -> (bool, (usize, usize)) {
    let mut a: usize = 0;
    let mut b: usize = 0;
    (level.windows(2).all(|window| {
        println!("window: {:?}", window);
        match pairs.get(&(window[1], window[0])) {
            Some(true) => {
                // a = window[0];
                // b = window[1];
                return false;
            },
            Some(false) => {
                panic!("Shouldn't happen");
            }
            None => {
                return true;
            }
        }
    }), (a, b))
}

fn check_level(level: &Vec<i32>, pairs: Vec<(i32, i32)>) -> (bool, (usize, usize)) {
    for (a, b) in &pairs {
        let a_index;
        let b_index;

        match level.iter().position(|&r| r == *a) {
            Some(index) => {
                a_index = index;
            }
            None => {
                // a is not in level, can skip
                continue;
            }
        }
        match level.iter().position(|&r| r == *b) {
            Some(index) => {
                b_index = index;
            }
            None => {
                // b is not in level, can skip
                continue;
            }
        }
        if a_index > b_index {
            return (false, (a_index, b_index));
        }
    }
    (true, (0, 0))
}

fn parsey_warsey(text: &String) -> IResult<&str, (Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    let (remaining, order_pairs) = parse_all_pairs_v2(text)?;
    let (remaining, _) = context("multispace0", multispace0)(remaining)?;
    let (remaining, levels) = parse_all_levels(remaining)?;

    // println!(
    //     "Remaining: {:?}, order_pairs: {:?}, levels: {:?}",
    //     remaining, order_pairs, levels
    // );

    Ok((remaining, (order_pairs, levels)))
}

fn parse_all_levels(text: &str) -> IResult<&str, Vec<Vec<i32>>> {
    context(
        "parse_all_levels",
        separated_list1(newline, separated_list1(char(','), i32)),
    )(text)
}

fn parse_all_pairs_v2(text: &String) -> IResult<&str, Vec<(i32, i32)>> {
    context("parse_all_pairs_v2", separated_list1(newline, parse_pair))(text)
}

fn parse_pair(input: &str) -> IResult<&str, (i32, i32)> {
    context("parse_pair", separated_pair(i32, tag("|"), i32))(input)
}
