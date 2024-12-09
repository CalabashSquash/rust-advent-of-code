use std::{collections::HashMap};

use nom::{bytes::complete::{is_not, take_till, take_until, take_while}, character::complete::anychar, multi::{many0, many1}, sequence::terminated, IResult, InputIter};

#[derive(Debug)]
struct Field {
    grid: Vec<Vec<char>>,
    pos: (usize,usize),
    facing: Direction,
    visited: HashMap<(usize, usize), Direction>,
}

impl Field {
    fn find_next_stop(field: &mut Self) -> Action {
        match field.facing {
            Direction::Down => Self::look_down(field),
            Direction::Left => Self::look_left(field),
            Direction::Up => Self::look_up(field),
            Direction::Right => Self::look_right(field),
        }

    }

    fn solve(field: &mut Self) -> Action {
        for _ in 0..2000000 {
            let next_stop = Field::find_next_stop(field);
            field.facing = field.facing.turn_right();
            match next_stop {
                Action::Turn(next_stop) => {
                    // println!("Next stop: {:?}", next_stop);
                    // println!("Facing: {:?}", field.facing);
                    field.pos = next_stop;
                    // println!("Grid: ");
                    // for row in field.grid.clone() {
                    //     println!("{:?}", row.iter().collect::<String>())
                    // }
                },
                Action::Loop => {
                    return Action::Loop
                },
                Action::End => {
                    return Action::End
                }
            }
        }
        Action::End
    }

    fn look_up(field: &mut Self) -> Action {
        let (start_row, column_number) = field.pos;
        let mut new_grid = field.grid.clone();
        let mut looped = false;

        let res = field.grid.iter().take(start_row + 1).enumerate().rev().find(|(row_number, row)| {
            // println!("row_number: {}, row: {:?}", row_number, row.iter().collect::<String>());
            if row[column_number] == '#' {
                return true
            } else {
                // Mark as visited
                if let None = field.visited.get(&(*row_number, column_number)) {
                    field.visited.insert((*row_number, column_number), Direction::Up);
                    new_grid[*row_number][column_number] = '8';
                } else if let Some(x) = field.visited.get(&(*row_number, column_number)) {
                    if let Direction::Up = *x {
                        looped = true;
                    }
                }
                return false;
            }
        });

        if looped {
            return Action::Loop;
        }
        if let Some((row_number, _)) = res {
            // field.grid = new_grid;
            return Action::Turn((row_number + 1, column_number)); // +1 because we want to stop just before
        } else {
            // Found the end
            Action::End
        }
    }

    fn look_down(field: &mut Self) -> Action {
        let (start_row, column_number) = field.pos;
        // println!("Column number: {}", column_number);
        let mut looped = false;
        // println!("start_row: {}", start_row);
        // println!("Field.grid: {:?}", field.grid);
        let mut new_grid = field.grid.clone();

        let res = field.grid.iter().enumerate().skip(start_row + 1).find(|(row_number, row)| {
            // println!("row_number: {}", row_number);
            // println!("row: {:?}", row.iter().collect::<String>());

            if row[column_number] == '#' {
                return true
            } else {
                // Mark as visited
                if let None = field.visited.get(&(*row_number, column_number)) {
                    field.visited.insert((*row_number, column_number), Direction::Down);
                    new_grid[*row_number][column_number] = '8';
                } else if let Some(x) = field.visited.get(&(*row_number, column_number)) {
                    if let Direction::Down = *x {
                        looped = true;
                    }
                }
                return false;
            }
        });

        if looped {
            return Action::Loop;
        }
        if let Some((row_number, _)) = res {
            // field.grid = new_grid;
            return Action::Turn((row_number - 1, column_number)); // -1 because we want to stop just before
        } else {
            // Found the end
            Action::End
        }
    }

    fn look_right(field: &mut Self) -> Action {
        let (row_number, start_column) = field.pos;
        let mut new_grid = field.grid.clone();
        let mut looped = false;

        let res = field.grid[row_number].iter().skip(start_column + 1).enumerate().find(|(column_number, char)| {
            let actual_index = start_column + column_number + 1;
            // println!("col_number: {}, actual col number: {}, row: {:?}", column_number, actual_index, field.grid[row_number].iter().collect::<String>());
            if **char== '#' {
                return true
            } else {
                // Mark as visited
                if let None = field.visited.get(&(row_number, actual_index)) {
                    field.visited.insert((row_number, actual_index), Direction::Right);
                    new_grid[row_number][actual_index] = '8';
                } else if let Some(x) = field.visited.get(&(row_number, actual_index)) {
                    if let Direction::Right = *x {
                        looped = true;
                    }
                }
            }
            return false;
        });

        if looped {
            return Action::Loop
        }
        if let Some((column_number, _)) = res {
            // field.grid = new_grid;
            Action::Turn((row_number, start_column + column_number))
        } else {
            Action::End
        }
    }

    fn look_left(field: &mut Self) -> Action {
        let (row_number, start_column) = field.pos;
        let mut new_grid = field.grid.clone();

        let mut looped = false;

        let res = field.grid[row_number].iter().take(start_column).enumerate().rev().find(|(column_number, char)| {
            if **char == '#' {
                return true
            } else {
                // Mark as visited
                if let None = field.visited.get(&(row_number, *column_number)) {
                    field.visited.insert((row_number, *column_number), Direction::Left);
                    new_grid[row_number][*column_number] = '8';
                } else if let Some(x) = field.visited.get(&(row_number, *column_number)) {
                    if let Direction::Left = *x {
                        looped = true;
                    }
                }
            }

            return false;
        });

        if looped {
            return Action::Loop;
        }
        if let Some((column_number, _)) = res {
            // field.grid = new_grid;
            return Action::Turn((row_number, column_number + 1));
        } else {
            return Action::End;
        }
    }



}

enum Action {
    Loop, // An infinite loop
    Turn((usize, usize)), // Turn and keep going
    End,  // Reached the end!
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Copy for Direction {

}

impl Clone for Direction {
    fn clone(&self) -> Direction {
        *self
    }
}

impl Direction{
    fn turn_right(self: &Self) -> Self {
        match self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }
}

pub fn problem1() -> i64 {
    let text = match std::fs::read_to_string("src/day6.txt") {
        Ok(t) => t,
        Err(_) => panic!("Error reading file")
    };

    let mut grid = if let  Ok((remaining, grid)) = parser(&text) {
        if remaining.len() > 0 {
            panic!("Not all text was parsed");
        }
        grid
    } else {
        panic!("Error parsing")
    };

    let (coords, facing) = get_starting_position_and_direction(&grid);

    grid[coords.0][coords.1] = '.';

    // println!("GRID: {:?}", grid);
    
    let mut field = Field {
        grid,
        pos: coords,
        facing,
        visited: HashMap::new()
    };
    // return Field::solve(&mut field);

    // println!("pos: {:?}, facing: {:?}", field.pos, field.facing);


    let mut looped = 0;
    let original_pos = field.pos;
    let original_facing = field.facing;

    for (row_num, row) in field.grid.clone().iter().enumerate() {
        for (column_number, char) in row.iter().enumerate() {
            if true == true || row_num == 6 && column_number == 3 {
                if *char == '.' {
                    field.grid[row_num][column_number] = '#';
                    // println!("Adding # to ({},{})", row_num, column_number);
                    // println!("Grid: ");
                    // for row in field.grid.clone() {
                    //     println!("{:?}", row.iter().collect::<String>())
                    // }
                    match Field::solve(&mut field) {
                        Action::Loop => {
                            println!("LOOPED at ({},{})", row_num, column_number);
                            looped += 1;
                        }
                        _ => {}
                    }
                    field.grid[row_num][column_number] = '.';
                    field.visited = HashMap::new();
                    field.pos = original_pos;
                    field.facing = original_facing;
                }
            }
        }
    }

    looped
}

fn get_starting_position_and_direction(grid: &Vec<Vec<char>>) -> ((usize,usize), Direction) {

    println!("Grid length: {}", grid.len());

    let coords = grid.iter().enumerate().find_map(|(row_number, row)| {
        row.iter().enumerate().find_map(|(column_number, &char) | {
            if "^><v".contains(&String::from(char)) {
                return Some(((row_number, column_number), char))
            } else {
                None
            }
        })
    });

    let (coords, char) = match coords {
        Some((coordinates, char)) => (coordinates, char),
        None => {panic!("Starting coords not found");}
    };

    let direction = match char {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '>' => Direction::Right,
        '<' => Direction::Left,
        _ => panic!("Not correct starting char??")
    };

    (coords, direction)
}

fn parser(text: &str) -> IResult<&str, Vec<Vec<char>>> {
    let res = many0(parse_line)(text)?;

    Ok(res)
}

fn parse_line(text: &str) -> IResult<&str, Vec<char>> {
    // This way parses into a vec of strings vvv
    let (remaining, line) = terminated(take_until("\n"), take_while(|c| c == '\n'))(text)?;
    // println!("LINE: {}", line);
    Ok((remaining, line.chars().collect()))
}


