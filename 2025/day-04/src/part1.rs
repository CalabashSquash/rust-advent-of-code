use std::io::Empty;

use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::all_consuming,
    multi::{many1, separated_list1},
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Paper,
    Empty,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, tileboard) = all_consuming(parse)(_input).map_err(|e| miette!("Error! {e}"))?;
    // println!("{:#?}", tileboard);
    Ok(count(tileboard))
}

fn count(tileboard: Vec<Vec<Tile>>) -> String {
    tileboard
        .iter()
        .enumerate()
        .map(|(row_num, row)| {
            println!("{row_num}");
            row.iter()
                .enumerate()
                .filter(|(c_i, _)| {
                    println!("{row_num}{c_i}");
                    let x = is_valid(&tileboard, row_num, *c_i);
                    println!("{x}");
                    x
                })
                .count()
            //  {
            //     println!("{c_i:#?}");
            //     println!("{}", is_valid(&tileboard, &row_num, &c_i));
            // }
            // (0..row.len())
            //     .enumerate()
            //     .filter(|(col_num, _)| is_valid(&tileboard, &row_num, col_num))
        })
        .sum::<usize>()
        .to_string()
}

fn is_valid(tileboard: &Vec<Vec<Tile>>, row: usize, col: usize) -> bool {
    let row = row;
    let col = col;
    if tileboard[row][col] == Tile::Empty {
        return false;
    }

    let mut count = 0;
    for rel_row in -1..=1 {
        let row_to_check = row as i32 + rel_row;
        if row_to_check < 0 || row_to_check > (tileboard.len() - 1) as i32 {
            if row == 0 && col == 7 && rel_row == 0 {
                println!("===={}", row_to_check);
                println!("xx")
            }
            continue;
        }
        for rel_col in -1..=1 {
            if rel_col == 0 && rel_row == 0 {
                continue;
            }
            let col_to_check = col as i32 + rel_col;
            if col_to_check < 0
                || col_to_check > (tileboard[row_to_check as usize].len() - 1) as i32
            {
                continue;
            }
            if tileboard[row_to_check as usize][col_to_check as usize] == Tile::Paper {
                if row == 0 && col == 7 {
                    println!("===={}:{}", row_to_check, col_to_check);
                }
                count += 1;
            }
        }
    }

    count < 4
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(line_ending, line)(input)
}

fn line(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(nom::branch::alt((tag("@"), tag("."))).map(|c| match c {
        "@" => Tile::Paper,
        "." => Tile::Empty,
        _ => panic!("unexpected char"),
    }))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
