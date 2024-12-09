use miette::miette;
use nom::{
    branch::alt,
    character::complete::{self, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many0, many0},
    sequence::{separated_pair, terminated},
    IResult,
};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, (mut col1, mut col2)) = parser(_input).map_err(|e| miette!("Parse failed {}", e))?;

    let mut distance = 0;
    col1.sort();
    col2.sort();
    for i in 0..col1.len() {
        distance += (col1[i] - col2[i]).abs();
    }

    // Another way to do it
    distance = std::iter::zip(col1, col2).map(|(x, y)| (x - y).abs()).sum();

    Ok(distance.to_string())
}

fn parser(_input: &str) -> IResult<&str, (Vec<i32>, Vec<i32>)> {
    fold_many0(
        terminated(
            separated_pair(complete::i32, multispace1, complete::i32),
            alt((line_ending, eof)),
        ),
        || (Vec::new(), Vec::new()),
        |(mut l_accum, mut r_accum), (l, r)| {
            l_accum.push(l);
            r_accum.push(r);
            (l_accum, r_accum)
        },
    )(_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}

// This was from before I learnt how to parse properly in rust
fn parse_file(_input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut first_col = Vec::new();
    let mut second_col = Vec::new();

    for line in _input.lines() {
        let mut numbers = line.split_whitespace();
        first_col.push(numbers.next().unwrap().parse::<i64>().unwrap());
        second_col.push(numbers.next().unwrap().parse::<i64>().unwrap());
    }
    (first_col, second_col)
}
