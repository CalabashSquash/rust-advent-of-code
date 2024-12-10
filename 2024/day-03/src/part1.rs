use std::ops::Mul;

use nom::{Parser, branch::alt, bytes::complete::{tag, take_until, take_while}, character::complete::{self, anychar}, combinator::{map, map_res, opt}, multi::{fold_many0, many0, many1, many_till}, sequence::{delimited, separated_pair, terminated}, IResult};
use miette::miette;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, pairs) = parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;
    let solution: i32 = pairs.iter().map(|op| {
        let Operation::Mul(x, y) = op;
        return x * y;
    }
    ).sum();

    Ok(solution.to_string())
}

enum Operation {
    Mul(i32,i32)
}

fn parse(_input: &str) -> IResult<&str, Vec<Operation>> {
        many1(
            many_till(anychar, parse_pair)
                .map(|(_discard, pair)| {
                    pair
                })
        )(_input)
}

// Takes: "123,456"
// Returns: (123, 456)
fn parse_pair(_input: &str) -> IResult<&str, Operation> {
    let (remaining, _) = tag("mul")(_input)?;
    let (remaining, pair) = delimited(
        tag("("),
        separated_pair(complete::i32, tag(","), complete::i32),
        tag(")")
    )(remaining)?;

    Ok((remaining, Operation::Mul(pair.0, pair.1)))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
