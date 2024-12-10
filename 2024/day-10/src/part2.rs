use miette::miette;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many0, many_till},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, map) = parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;

    // Filter out the zeros and search from there.
    // Find number of
    let sum: i32 = map
        .iter()
        .enumerate()
        .map(|(row_num, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &height)| height == 0)
                .map(|(col_num, _)| find_trails(0, (row_num, col_num), &map))
                .sum::<i32>()
        })
        .sum();

    Ok(sum.to_string())
}

fn find_trails(elem: u32, coords: (usize, usize), map: &Vec<Vec<u32>>) -> i32 {
    if elem == 9 {
        return 1;
    }

    let surrounding_found = search_surrounding(elem + 1, coords, &map);
    surrounding_found
        .iter()
        .map(|&coord| find_trails(elem + 1, coord, map))
        .sum()
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
    Ok((remaining, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "012345
123456
234567
345678
416789
567891
";
        assert_eq!("227", process(input)?);
        Ok(())
    }
}
