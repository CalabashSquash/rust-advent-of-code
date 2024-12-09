use std::fs::read_to_string;

// Tracing crate good for debugging
// Print out every function call
fn search_line(numbers: &Vec<i64>) -> bool {
    let mut is_safe = true;
    let mut previous = -1;
    let mut increasing: Option<bool> = None;

    for i in 0..numbers.len() {
        let num = numbers[i];
        if previous == -1 {
            previous = num;
        } else {
            if num == previous {
                is_safe = false;
                break;
            }
            if increasing == Some(true) && num > previous && num - previous < 4 {
                previous = num;
                continue;
            }
            if increasing == Some(false) && num < previous && previous - num < 4 {
                previous = num;
                continue;
            }
            if increasing == None {
                match num > previous {
                    true => {
                        increasing = Some(true);
                        if num - previous > 3 {
                            is_safe = false;
                            break;
                        }
                    }
                    false => {
                        increasing = Some(false);
                        if previous - num > 3 {
                            is_safe = false;
                            break;
                        }
                    }
                }
                previous = num;
                continue;
            }
            is_safe = false;
            break;
        }
    }
    is_safe
}

pub fn problem2() -> i64 {
    let mut safe = 0;
    for line in read_to_string("src/day2.txt").unwrap().lines() {
        let numbers: std::str::Split<'_, &str> = line.split(" ");
        let vec: Vec<i64> = numbers.map(|f| f.parse().expect("not a num")).collect();
        match search_line(&vec) {
            true => {
                safe += 1;
            }
            false => {
                for i in 0..vec.len() {
                    let mut numbers_without_x = vec.clone();
                    numbers_without_x.remove(i);
                    match search_line(&numbers_without_x) {
                        true => {
                            safe += 1;
                            break;
                        }
                        false => {
                            continue;
                        }
                    }
                }
                // // Not safe. Remove one of the indices and try again.
                // let mut numbers_without_x = vec.clone();
                // numbers_without_x.remove(x);
                // match search_line(&numbers_without_x) {
                //     (true, _) => {
                //         safe += 1;
                //     },
                //     (false, _) => {
                //         let mut numbers_without_y = vec.clone();
                //         numbers_without_y.remove(y);
                //         match search_line(&numbers_without_y) {
                //             (true, _) => {
                //                 safe += 1;
                //             },
                //             (false, _) => {
                //                 continue;
                //             }
                //         }
                //     }
                // }
            }
        }
    }

    safe
}

pub fn problem1() -> i64 {
    let mut safe = 0;
    for line in read_to_string("src/day2.txt").unwrap().lines() {
        let numbers = line.split(" ");
        let vec: Vec<i64> = numbers.map(|f| f.parse().expect("not a num")).collect();
        match search_line(&vec) {
            true => {
                safe += 1;
            }
            false => {}
        }
    }
    safe
}
