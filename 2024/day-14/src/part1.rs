use core::num;
use miette::miette;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace0},
    combinator::opt,
    multi::{fold_many0, many0},
    sequence::{separated_pair, terminated},
    IResult,
};
use tracing::span::Id;

#[derive(Debug)]
struct Coords {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Tile {
    occupants: Vec<Robot>,
}

#[derive(Debug)]
struct Robot {
    velocity: Velocity,
    id: u32,
}

#[tracing::instrument]
pub fn process(_input: &str, rows: usize, columns: usize) -> miette::Result<String> {
    let mut grid = generate_initial_grid(rows, columns);

    let (remaining, mut robots) = parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;

    let new_robots = play_simulation(robots, 100, rows, columns);
    println!("{:?}", &new_robots);

    populate_grid(&mut grid, new_robots);
    print_grid(&grid);
    dbg!(remaining);
    let safety = count_safety_score(&grid);
    dbg!(&safety);
    Ok(safety.to_string())
}

fn play_simulation(
    robots: Vec<(Robot, Coords)>,
    moves: i32,
    rows: usize,
    columns: usize,
) -> Vec<(Robot, Coords)> {
    let mut new_robots = Vec::new();
    for (robot, robot_pos) in &robots {
        let start_x = robot_pos.x;
        let start_y = robot_pos.y;
        let end_x = (((start_x as i32 + (robot.velocity.x * moves)) % columns as i32)
            + columns as i32)
            % columns as i32;

        println!(
            "start_y: {}, velocity: {}, moves: {}, rows: {}",
            start_y as i32, robot.velocity.y, moves, rows as i32
        );
        println!("Result: {}", (start_y as i32 + (robot.velocity.y * moves)));
        println!(
            "Result mod: {}",
            (start_y as i32 + (robot.velocity.y * moves)) % rows as i32
        );
        let end_y = (((start_y as i32 + (robot.velocity.y * moves)) % rows as i32) + rows as i32)
            % rows as i32;
        println!("end_y : {}", end_y);

        let new_pos = Coords {
            x: end_x as usize,
            y: end_y as usize,
        };
        new_robots.push((
            Robot {
                velocity: Velocity {
                    x: robot.velocity.x,
                    y: robot.velocity.y,
                },
                id: robot.id,
            },
            new_pos,
        ));
    }

    new_robots

    // robots[0] = (Robot {velocity: Velocity {x: 0, y: -1}, id: 0}, Coords {x: 0, y: 0});
}

fn count_safety_score(grid: &Vec<Vec<Tile>>) -> usize {
    // Top left quadrant

    let top_left: usize = (0..(grid.len() / 2))
        .map(|column| {
            (0..(grid[column].len() / 2)).map(move |row| grid[column][row].occupants.len())
            //.sum::<usize>()
        })
        .flatten()
        .sum();

    let top_right: usize = (((grid.len() / 2) + 1)..grid.len())
        .map(|column| {
            (0..(grid[column].len() / 2)).map(move |row| grid[column][row].occupants.len())
        })
        .flatten()
        .sum();

    let bottom_left: usize = (0..(grid.len() / 2))
        .map(|column| {
            (((grid[column].len() / 2) + 1)..grid[column].len())
                .map(move |row| grid[column][row].occupants.len())
        })
        .flatten()
        .sum();

    let bottom_right: usize = (((grid.len() / 2) + 1)..grid.len())
        .map(|column| {
            (((grid[column].len() / 2) + 1)..grid[column].len())
                .map(move |row| grid[column][row].occupants.len())
        })
        .flatten()
        .sum();

    top_left * top_right * bottom_left * bottom_right
}

fn populate_grid(grid: &mut Vec<Vec<Tile>>, robots: Vec<(Robot, Coords)>) {
    for (id, (robot, robot_position)) in robots.into_iter().enumerate() {
        grid[robot_position.x][robot_position.y]
            .occupants
            .push(Robot {
                id: id as u32,
                ..robot
            });
    }
}

// fn parse(_input: &str, rows: usize, columns: usize) -> IResult<&str, Vec<Vec<Tile>>> {
fn parse(_input: &str) -> IResult<&str, Vec<(Robot, Coords)>> {
    let (remaining, robots) = many0(terminated(parse_robot, opt(line_ending)))(_input)?;

    Ok((remaining, robots))
}

fn generate_initial_grid(rows: usize, columns: usize) -> Vec<Vec<Tile>> {
    vec![false; columns]
        .into_iter()
        .map(|_| generate_empty_column(rows))
        .collect()
}

fn generate_empty_column(columns: usize) -> Vec<Tile> {
    vec![false; columns]
        .into_iter()
        .map(|_| Tile {
            occupants: Vec::new(),
        })
        .collect()
}

fn parse_robot(_input: &str) -> IResult<&str, (Robot, Coords)> {
    let (remaining, position) = parse_position(_input)?;
    let (remaining, velocity) = parse_velocity(remaining)?;

    Ok((remaining, (Robot { velocity, id: 0 }, position)))
}

fn parse_position(_input: &str) -> IResult<&str, Coords> {
    let (remaining, _) = tag("p=")(_input)?;
    let (remaining, coords) = terminated(
        separated_pair(complete::u32, tag(","), complete::u32),
        multispace0, // Discard whitespace too
    )(remaining)?;
    Ok((
        remaining,
        Coords {
            x: coords.0 as usize,
            y: coords.1 as usize,
        },
    ))
}

fn parse_velocity(_input: &str) -> IResult<&str, Velocity> {
    let (remaining, _) = tag("v=")(_input)?;
    let (remaining, coords) = separated_pair(complete::i32, tag(","), complete::i32)(remaining)?;
    Ok((
        remaining,
        Velocity {
            x: coords.0,
            y: coords.1,
        },
    ))
}

fn print_grid_pretty(grid: &Vec<Vec<Tile>>) {
    for row in 0..grid[0].len() {
        for column in 0..grid.len() {
            let number_of_occupants = grid[column][row].occupants.len();
            // if number_of_occupants == 0 {
            //     print!(". ");
            // } else if number_of_occupants >= 10 {
            //     print!("{}", number_of_occupants);
            // } else {
            //     print!("{} ", number_of_occupants);
            // }
            if number_of_occupants == 0 {
                print!(".");
            } else {
                print!("X");
            }
        }
        println!("");
    }
}
fn print_grid(grid: &Vec<Vec<Tile>>) {
    for row in 0..grid[0].len() {
        for column in 0..grid.len() {
            let number_of_occupants = grid[column][row].occupants.len();
            // if number_of_occupants == 0 {
            //     print!(". ");
            // } else if number_of_occupants >= 10 {
            //     print!("{}", number_of_occupants);
            // } else {
            //     print!("{} ", number_of_occupants);
            // }
            if number_of_occupants == 0 {
                print!(" __ ");
            } else if number_of_occupants == 1 {
                let id = grid[column][row].occupants[0].id;
                if id >= 10 {
                    print!("i{} ", grid[column][row].occupants[0].id);
                } else {
                    print!("i{}_ ", grid[column][row].occupants[0].id);
                }
            } else if number_of_occupants >= 10 {
                print!("{}_ ", number_of_occupants);
            } else {
                print!("{}__ ", number_of_occupants);
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input, 7, 11)?);
        Ok(())
    }
}
