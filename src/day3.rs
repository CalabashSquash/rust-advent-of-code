use std::fs::read_to_string;

pub fn problem1() -> i64 {
    let text = parse_file("src/day3.txt");

    let mut sum = 0;

    let split = text.split("mul(");
    for mul_split in split {
        let pair = mul_split.split(",");
        let pair_vec: Vec<&str> = pair.collect();
        if pair_vec.len() < 2 {
            println!("pair_vec: {:#?}, not len 2", pair_vec);
            continue;
        }
        match pair_vec[0].parse::<i64>() {
            Ok(first) => {
                if !pair_vec[1].contains(')') {
                    continue;
                }
                let second_num_split: Vec<&str> = pair_vec[1].split(')').collect();
                let second_num = second_num_split[0].parse::<i64>();
                match second_num {
                    Ok(second) => {
                        if second < 0 || second > 999 {
                            continue;
                        }
                        println!("multiplying {} and {}", first, second);
                        sum += first * second;
                    },
                    Err(_) => {
                        println!("pair_vec: {:#?}, no second num", pair_vec);
                        continue;
                    }
                }
            },
            Err(_) => {
                println!("pair_vec: {:#?}, no first num", pair_vec);
                continue;
            }
        }
        // println!("pair {:#?}", pair_vec);
    }

    println!("{}", text);
    sum
}

fn sum_muls(split: std::str::Split<'_, &str>) -> i64 {
    let mut sum = 0;
    for mul_split in split {
        let pair = mul_split.split(",");
        let pair_vec: Vec<&str> = pair.collect();
        if pair_vec.len() < 2 {
            println!("pair_vec: {:#?}, not len 2", pair_vec);
            continue;
        }
        match pair_vec[0].parse::<i64>() {
            Ok(first) => {
                if !pair_vec[1].contains(')') {
                    continue;
                }
                let second_num_split: Vec<&str> = pair_vec[1].split(')').collect();
                let second_num = second_num_split[0].parse::<i64>();
                match second_num {
                    Ok(second) => {
                        if second < 0 || second > 999 {
                            continue;
                        }
                        println!("multiplying {} and {}", first, second);
                        sum += first * second;
                    },
                    Err(_) => {
                        println!("pair_vec: {:#?}, no second num", pair_vec);
                        continue;
                    }
                }
            },
            Err(_) => {
                println!("pair_vec: {:#?}, no first num", pair_vec);
                continue;
            }
        }
        // println!("pair {:#?}", pair_vec);
    }
    sum
}

pub fn problem2() -> i64 {
    let mut text = parse_file("src/day3.txt");

    // xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    // 1. Split at first don't()
    //      xmul(2,4)&mul[3,7]!^   [(don't()]   _mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    // 2. Do all the muls in first half
    // 3. In second string, split at first do()
    //      _mul(5,5)+mul(32,64](mul(11,8)un    [do()]     ?mul(8,5))
    // 4. Repeat step 1

    let mut sum = 0;

    loop {
        let split = split_at_nth_char_ex(text.as_str(), String::from("don't()"), 0, 7);
        println!("SPLIT: {:#?}", split);
        match split {
            Some((a, b)) => {
                sum += sum_muls(a.split("mul("));
                text = match split_at_nth_char_ex(b, String::from("do()"), 0, 4) {
                    Some((_, b)) => b.to_string(),
                    None => break
                };
                println!("new text: {}", text);
            },
            None => {
                sum += sum_muls(text.split("mul("));
                break;
            }
        }

        // let after_dont: Vec<&str> = split[1]
    }
    // let split: std::str::Split<'_, &str> = text.split("mul(");
    // sum += sum_muls(split);
    sum
}

/// Search `s` for the `n`th occurrence of `p`, then split the string
/// into two halves around that point.
fn split_at_nth_char(s: &str, p: String, n: usize) -> Option<(&str, &str)> {
    s.match_indices(&p).nth(n).map(|(index, _)| s.split_at(index))
}

/// Same as `split_at_nth_char` but don't include the character.
fn split_at_nth_char_ex(s: &str, p: String, n: usize, len: usize) -> Option<(&str, &str)> {
    split_at_nth_char(s, p, n).map(|(left, right)| {
        (
            left,
            // Trim 1 character.
            &right[right.char_indices().nth(len).unwrap().0..],
        )
    })
}

fn parse_file(filename: &str) -> String {
    read_to_string(filename).unwrap()
}