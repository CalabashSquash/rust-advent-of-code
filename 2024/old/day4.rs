use std::{collections::HashMap, error::Error, fs::read_to_string};

fn do_thing(
    grid: &Vec<Vec<char>>,
    row_count: usize,
    col_count: usize,
    a_visited: &HashMap<(usize, usize), bool>,
) -> Vec<Direction> {
    let mut as_found = Vec::new();
    println!("Calling with {} and {}", row_count, col_count);
    match try_find(&grid, row_count - 1, col_count - 1, 'A') {
        Some(_) => match a_visited.get(&(row_count - 1, col_count - 1)) {
            Some(&true) => {}
            _ => match try_find(&grid, row_count - 2, col_count - 2, 'S') {
                Some(_) => {
                    as_found.push(Direction::UpLeft);
                }
                None => {}
            },
        },
        None => {}
    }
    match try_find(&grid, row_count - 1, col_count + 1, 'A') {
        Some(_) => match a_visited.get(&(row_count - 1, col_count + 1)) {
            Some(&true) => {}
            _ => match try_find(&grid, row_count - 2, col_count + 2, 'S') {
                Some(_) => {
                    as_found.push(Direction::UpRight);
                }
                None => {}
            },
        },
        None => {}
    }
    match try_find(&grid, row_count + 1, col_count - 1, 'A') {
        Some(_) => match a_visited.get(&(row_count + 1, col_count - 1)) {
            Some(&true) => {
                println!("DING DING");
            }
            _ => match try_find(&grid, row_count + 2, col_count - 2, 'S') {
                Some(_) => {
                    as_found.push(Direction::DownLeft);
                }
                None => {}
            },
        },
        None => {}
    }
    match try_find(&grid, row_count + 1, col_count + 1, 'A') {
        Some(_) => match a_visited.get(&(row_count + 1, col_count + 1)) {
            Some(&true) => {}
            _ => match try_find(&grid, row_count + 2, col_count + 2, 'S') {
                Some(_) => {
                    as_found.push(Direction::DownRight);
                }
                None => {}
            },
        },
        None => {}
    }
    as_found
}

pub fn problem2() -> i64 {
    // For every X, look for an M at:
    //      - +0 column, +1 row
    //      - +0 column, -1 row
    //      - +1 column, -1 row
    //      - +1 column, +0 row
    //      - +1 column, +1 row
    //      - -1 column, -1 row
    //      - -1 column, +0 row
    //      - -1 column, +1 row

    // For the next M, look for an A at:
    //      ....

    let text = read_to_string("src/day4_test.txt").unwrap();
    let lines = text.lines();
    let ding_dong = lines.map(|line| line.chars().collect::<Vec<char>>());
    let grid: Vec<Vec<char>> = ding_dong.collect();
    println!("{:?}", grid);
    // let text = text.chars();

    let mut a_visited: HashMap<(usize, usize), bool> = HashMap::new();

    let mut count = 0;

    for (row_count, row) in grid.iter().enumerate() {
        if row_count == 0 {
            continue;
        }
        for (col_count, column) in row.iter().enumerate() {
            if col_count == 0 {
                continue;
            }
            //println!("row: {},{:?}  col: {},{}", row_count, row, col_count, column);
            if *column == 'M' {
                let as_found = do_thing(&grid, row_count, col_count, &a_visited);
                println!("{:?}", as_found);
                for direction in as_found {
                    match direction {
                        Direction::DownRight => {
                            if grid[row_count + 2][col_count] == 'M' {
                                match try_find(&grid, row_count, col_count + 2, 'S') {
                                    Some(_) => {
                                        a_visited.insert((row_count + 1, col_count + 1), true);
                                        count += 1
                                    }
                                    None => {}
                                }
                            } else if grid[row_count][col_count + 2] == 'M' {
                                match try_find(&grid, row_count + 2, col_count, 'S') {
                                    Some(_) => {
                                        a_visited.insert((row_count + 1, col_count + 1), true);
                                        count += 1
                                    }
                                    None => {}
                                }
                            }
                        }
                        Direction::DownLeft => {
                            if grid[row_count + 2][col_count] == 'M' {
                                match try_find(&grid, row_count, col_count - 2, 'S') {
                                    Some(_) => {
                                        a_visited.insert((row_count + 1, col_count - 1), true);
                                        count += 1
                                    }
                                    None => {}
                                }
                            }
                            if grid[row_count][col_count - 2] == 'M' {
                                match try_find(&grid, row_count - 2, col_count, 'S') {
                                    Some(_) => {
                                        a_visited.insert((row_count + 1, col_count - 1), true);
                                        count += 1
                                    }
                                    None => {}
                                }
                            }
                        }
                        Direction::UpRight => {
                            if grid[row_count + 2][col_count] == 'M' {
                                match try_find(&grid, row_count - 2, col_count, 'S') {
                                    Some(_) => {
                                        a_visited.insert((row_count - 1, col_count + 1), true);
                                        count += 1
                                    }
                                    None => {}
                                }
                            }
                            if grid[row_count][col_count + 2] == 'M' {
                                match try_find(&grid, row_count - 2, col_count, 'S') {
                                    Some(_) => {
                                        a_visited.insert((row_count - 1, col_count + 1), true);
                                        count += 1
                                    }
                                    None => {}
                                }
                            }
                        }
                        Direction::UpLeft => {
                            if grid[row_count - 2][col_count] == 'M' {
                                match try_find(&grid, row_count, col_count - 2, 'S') {
                                    Some(_) => {
                                        a_visited.insert((row_count - 1, col_count - 1), true);
                                        count += 1
                                    }
                                    None => {}
                                }
                            }
                            if grid[row_count][col_count - 2] == 'M' {
                                match try_find(&grid, row_count - 2, col_count, 'S') {
                                    Some(_) => {
                                        a_visited.insert((row_count - 1, col_count - 1), true);
                                        count += 1
                                    }
                                    None => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }

                // match look_up_right_for('A', row_count as i64, col_count as i64, &grid) {
                //     Some(coords) => {
                //         do_thing(coords, &grid, row_count, col_count);
                //         for (row_coord, col_coord) in coords {
                //             println!("row_coord: {}, col_coord: {}", row_coord, col_coord);
                //             if row_coord < row_count {
                //                 // Up, Up Left, or Up Right
                //                 if col_coord == col_count {
                //                     // Up
                //                     match try_find(&grid, row_coord - 1, col_coord, 'A') {
                //                         Some(_) => {
                //                             match try_find(&grid, row_coord - 2, col_coord, 'S') {
                //                                 Some(_) => {
                //                                     count += 1;
                //                                     println!("Count: {}", count);
                //                                 },
                //                                 None => {}
                //                             }
                //                         },
                //                         None => {}
                //                     }
                //                 } else if col_coord < col_count {
                //                     // Up Left
                //                     match try_find(&grid, row_coord - 1, col_coord - 1, 'A') {
                //                         Some(_) => {
                //                             match try_find(&grid, row_coord - 2, col_coord - 2, 'S') {
                //                                 Some(_) => {
                //                                     count += 1;
                //                                     println!("Count: {}", count);
                //                                 },
                //                                 None => {}
                //                             }
                //                         },
                //                         None => {}
                //                     }
                //                 } else if col_coord > col_count {
                //                     // Up Right
                //                     match try_find(&grid, row_coord - 1, col_coord + 1, 'A') {
                //                         Some(_) => {
                //                             match try_find(&grid, row_coord - 2, col_coord + 2, 'S') {
                //                                 Some(_) => {
                //                                     count += 1;
                //                                     println!("Count: {}", count);
                //                                 },
                //                                 None => {}
                //                             }
                //                         },
                //                         None => {}
                //                     }
                //                 }
                //             }
                //             if row_coord > row_count {
                //                 // Down, Down Right, or Down Left
                //                 if col_coord > col_count {
                //                     // DownRight
                //                     match try_find(&grid, row_coord + 1, col_coord + 1, 'A') {
                //                         Some(_) => {
                //                             match try_find(&grid, row_coord + 2, col_coord + 2, 'S') {
                //                                 Some(_) => {
                //                                     count += 1;
                //                                     println!("Count: {}", count);
                //                                 },
                //                                 None => {}
                //                             }
                //                         },
                //                         None => {}
                //                     }
                //                 }
                //                 if col_coord == col_count {
                //                     // Down
                //                     match try_find(&grid, row_coord + 1, col_coord, 'A') {
                //                         Some(_) => {
                //                             match try_find(&grid, row_coord + 2, col_coord, 'S') {
                //                                 Some(_) => {
                //                                     count += 1;
                //                                     println!("Count: {}", count);
                //                                 },
                //                                 None => {}
                //                             }
                //                         },
                //                         None => {}
                //                     }
                //                 }
                //                 if col_coord < col_count {
                //                     // Down Left
                //                     match try_find(&grid, row_coord + 1, col_coord - 1, 'A') {
                //                         Some(_) => {
                //                             match try_find(&grid, row_coord + 2, col_coord - 2, 'S') {
                //                                 Some(_) => {
                //                                     count += 1;
                //                                     println!("Count: {}", count);
                //                                 },
                //                                 None => {}
                //                             }
                //                         },
                //                         None => {}
                //                     }
                //                 }
                //             }

                //             if row_coord == row_count {
                //                 // Left or Right
                //                 if col_coord == col_count {
                //                     panic!("huh");
                //                 } else if col_coord < col_count {
                //                     // Left
                //                     match try_find(&grid, row_coord, col_coord - 1, 'A') {
                //                         Some(_) => {
                //                             match try_find(&grid, row_coord, col_coord - 2, 'S') {
                //                                 Some(_) => {
                //                                     count += 1;
                //                                     println!("Count: {}", count);
                //                                 },
                //                                 None => {}
                //                             }
                //                         },
                //                         None => {}
                //                     }
                //                 } else if col_coord > col_count {
                //                     // Left
                //                     match try_find(&grid, row_coord, col_coord + 1, 'A') {
                //                         Some(_) => {
                //                             match try_find(&grid, row_coord, col_coord + 2, 'S') {
                //                                 Some(_) => {
                //                                     count += 1;
                //                                     println!("Count: {}", count);
                //                                 },
                //                                 None => {}
                //                             }
                //                         },
                //                         None => {}
                //                     }
                //                 }
                //             }
                //             // println!("M COORD: {},{}", row_coord, col_coord);
                //             // match search_surrounding('A', row_coord as i64, col_coord as i64, &grid) {
                //             //     Some(coords) => {
                //             //         for (row_coord, col_coord) in coords {
                //             //             println!("A COORD: {},{}", row_coord, col_coord);
                //             //         }
                //             //     },
                //             //     None => {}
                //             // }
                //         }
                //     },
                //     None => {}
                // }

                // let combinations = Vec::new();
            }
        }
    }

    count
}

fn try_get(grid: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> Option<char> {
    match grid.get(row_index) {
        Some(row) => match row.get(col_index) {
            Some(x) => Some(*x),
            None => None,
        },
        None => None,
    }
}

pub fn problem1() -> i64 {
    // For every X, look for an M at:
    //      - +0 column, +1 row
    //      - +0 column, -1 row
    //      - +1 column, -1 row
    //      - +1 column, +0 row
    //      - +1 column, +1 row
    //      - -1 column, -1 row
    //      - -1 column, +0 row
    //      - -1 column, +1 row

    // For the next M, look for an A at:
    //      ....

    let text = read_to_string("src/day4.txt").unwrap();
    let lines = text.lines();
    let ding_dong = lines.map(|line| line.chars().collect::<Vec<char>>());
    let grid: Vec<Vec<char>> = ding_dong.collect();
    println!("{:?}", grid);
    // let text = text.chars();

    let mut count = 0;

    for (row_count, row) in grid.iter().enumerate() {
        if row_count == 0 {
            continue;
        }
        for (col_count, column) in row.iter().enumerate() {
            if col_count == 0 {
                continue;
            }
            //println!("row: {},{:?}  col: {},{}", row_count, row, col_count, column);
            if *column == 'X' {
                match search_surrounding('M', row_count as i64, col_count as i64, &grid) {
                    Some(coords) => {
                        for (row_coord, col_coord) in coords {
                            println!("row_coord: {}, col_coord: {}", row_coord, col_coord);
                            if row_coord < row_count {
                                // Up, Up Left, or Up Right
                                if col_coord == col_count {
                                    // Up
                                    match try_find(&grid, row_coord - 1, col_coord, 'A') {
                                        Some(_) => {
                                            match try_find(&grid, row_coord - 2, col_coord, 'S') {
                                                Some(_) => {
                                                    count += 1;
                                                    println!("Count: {}", count);
                                                }
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                } else if col_coord < col_count {
                                    // Up Left
                                    match try_find(&grid, row_coord - 1, col_coord - 1, 'A') {
                                        Some(_) => {
                                            match try_find(&grid, row_coord - 2, col_coord - 2, 'S')
                                            {
                                                Some(_) => {
                                                    count += 1;
                                                    println!("Count: {}", count);
                                                }
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                } else if col_coord > col_count {
                                    // Up Right
                                    match try_find(&grid, row_coord - 1, col_coord + 1, 'A') {
                                        Some(_) => {
                                            match try_find(&grid, row_coord - 2, col_coord + 2, 'S')
                                            {
                                                Some(_) => {
                                                    count += 1;
                                                    println!("Count: {}", count);
                                                }
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                }
                            }
                            if row_coord > row_count {
                                // Down, Down Right, or Down Left
                                if col_coord > col_count {
                                    // DownRight
                                    match try_find(&grid, row_coord + 1, col_coord + 1, 'A') {
                                        Some(_) => {
                                            match try_find(&grid, row_coord + 2, col_coord + 2, 'S')
                                            {
                                                Some(_) => {
                                                    count += 1;
                                                    println!("Count: {}", count);
                                                }
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                }
                                if col_coord == col_count {
                                    // Down
                                    match try_find(&grid, row_coord + 1, col_coord, 'A') {
                                        Some(_) => {
                                            match try_find(&grid, row_coord + 2, col_coord, 'S') {
                                                Some(_) => {
                                                    count += 1;
                                                    println!("Count: {}", count);
                                                }
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                }
                                if col_coord < col_count {
                                    // Down Left
                                    match try_find(&grid, row_coord + 1, col_coord - 1, 'A') {
                                        Some(_) => {
                                            match try_find(&grid, row_coord + 2, col_coord - 2, 'S')
                                            {
                                                Some(_) => {
                                                    count += 1;
                                                    println!("Count: {}", count);
                                                }
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                }
                            }

                            if row_coord == row_count {
                                // Left or Right
                                if col_coord == col_count {
                                    panic!("huh");
                                } else if col_coord < col_count {
                                    // Left
                                    match try_find(&grid, row_coord, col_coord - 1, 'A') {
                                        Some(_) => {
                                            match try_find(&grid, row_coord, col_coord - 2, 'S') {
                                                Some(_) => {
                                                    count += 1;
                                                    println!("Count: {}", count);
                                                }
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                } else if col_coord > col_count {
                                    // Left
                                    match try_find(&grid, row_coord, col_coord + 1, 'A') {
                                        Some(_) => {
                                            match try_find(&grid, row_coord, col_coord + 2, 'S') {
                                                Some(_) => {
                                                    count += 1;
                                                    println!("Count: {}", count);
                                                }
                                                None => {}
                                            }
                                        }
                                        None => {}
                                    }
                                }
                            }
                            // println!("M COORD: {},{}", row_coord, col_coord);
                            // match search_surrounding('A', row_coord as i64, col_coord as i64, &grid) {
                            //     Some(coords) => {
                            //         for (row_coord, col_coord) in coords {
                            //             println!("A COORD: {},{}", row_coord, col_coord);
                            //         }
                            //     },
                            //     None => {}
                            // }
                        }
                    }
                    None => {}
                }

                // let combinations = Vec::new();
            }
        }
    }

    0
}

fn try_find(grid: &Vec<Vec<char>>, row_index: usize, col_index: usize, c: char) -> Option<char> {
    match grid.get(row_index) {
        Some(row) => match row.get(col_index) {
            Some(x) => {
                if *x == c {
                    return Some(c);
                }
                return None;
            }
            None => {
                return None;
            }
        },
        None => {
            return None;
        }
    };
}

#[derive(Debug)]
enum Direction {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

fn search_surrounding(
    c: char,
    row: i64,
    column: i64,
    grid: &Vec<Vec<char>>,
) -> Option<Vec<(usize, usize)>> {
    let mut found_indices = Vec::new();

    for row_mod in -1..2 {
        for col_mod in -1..2 {
            if row_mod == 0 && col_mod == 0 {
                continue;
            }
            let usize_row = (row + row_mod) as usize;
            let usize_col = (column + col_mod) as usize;
            if grid[usize_row][usize_col] == c {
                found_indices.push((usize_row, usize_col));
            }
            // println!("row_mod: {}, col_mod: {}", row_mod, col_mod);
        }
    }

    if found_indices.len() == 0 {
        return None;
    };

    Some(found_indices)
}
