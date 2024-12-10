use std::collections::HashSet;

use miette::miette;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many0, many_till},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, map) = parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;

    let mut result: u32 = 0;
    result = map.iter().enumerate().map(|(row_num, row)| {
        row.iter().enumerate().filter(|(_, &height)| {
            height == 0
        }).map(|(col_num, _)| {
            let mut ends_of_paths: HashSet<(usize, usize)> = HashSet::new();
            do_searching_around(0, (row_num, col_num), &map, &mut ends_of_paths);
            ends_of_paths.len() as u32
        }).sum::<u32>()

        // for (col_num, &height) in row.iter().enumerate() {
        //     if height == 0 {

        //         let mut ends_of_paths: HashSet<(usize, usize)> = HashSet::new();
        //         do_searching_around(0, (row_num, col_num), &map, &mut ends_of_paths);
        //         sum += ends_of_paths.len();
        //     }
        // }
    }).sum();

    Ok(result.to_string())
}

fn do_searching_around(
    elem: u32,
    coords: (usize, usize),
    map: &Vec<Vec<u32>>,
    set: &mut HashSet<(usize, usize)>,
) {
    if elem == 9 {
        set.insert(coords);
    }

    let surrounding_found = search_surrounding(elem + 1, coords, &map);
    surrounding_found
        .iter()
        .map(|&coord| do_searching_around(elem + 1, coord, map, set))
        .collect()
}

fn search_surrounding(
    find: u32,
    coords: (usize, usize),
    map: &Vec<Vec<u32>>,
) -> Vec<(usize, usize)> {
    let row = coords.0;
    let col = coords.1;
    let mut found_at = Vec::new();
    if row >= 1 {
        // Look up
        if map[row - 1][col] == find {
            found_at.push((row - 1, col));
        }
    }
    if col >= 1 {
        // Look left
        if map[row][col - 1] == find {
            found_at.push((row, col - 1));
        }
    }
    if row < map.len() - 1 {
        // Look down
        if map[row + 1][col] == find {
            found_at.push((row + 1, col));
        }
    }
    if col < map[0].len() - 1 {
        // Look right
        if map[row][col + 1] == find {
            found_at.push((row, col + 1));
        }
    }

    found_at
}

fn parse(_input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    many0(parse_line)(_input)
}

fn parse_line(_input: &str) -> IResult<&str, Vec<u32>> {
    let (remaining, (numbers, _)) = many_till(
        one_of("0123456789").map(|c| c.to_digit(10).unwrap()),
        line_ending,
    )(_input)?;
    dbg!(&numbers);
    Ok((remaining, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        //         let input = "9990999
        // 9991999
        // 9992999
        // 6543456
        // 7111117
        // 8111118
        // 9111119
        // ";

        //         let input = "1190919
        // 9951598
        // 9992917
        // 6543456
        // 7651987
        // 8761111
        // 9871111
        // ";
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
