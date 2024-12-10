use nom::{branch::alt, bytes::complete::{tag, take_until, take_while}, character::complete::{self}, combinator::opt, multi::{fold_many0, many0}, sequence::{delimited, separated_pair, terminated}, IResult};
use miette::miette;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, pairs) = parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;

    dbg!(pairs);


    Ok("hi".to_string())
}

fn parse(_input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let (remaining, consumed) = fold_many0(
            parse_next_mul
        ,
        Vec::new, |mut accum, new| {
            if new.is_some() {
                accum.push(new);
            }
            accum
        }
    )(_input)?;

    Ok((
        remaining,
        consumed.iter().filter(|o| {
            o.is_some()
        }).map(|o| {
            match o {
                Some(pair) => *pair,
                None => panic!("Should not have Nones after filtering")
            }
        }).collect()
    ))
}
    
fn parse_next_mul(_input: &str) -> IResult<&str, Option<(i32, i32)>> {
    let (remaining, _) = take_until("mul(")(_input)?;
    dbg!(&_input);
    let pair_parsed = delimited(tag("mul("), parse_pair, tag(")"))(remaining);
    match pair_parsed {
        Ok((remaining, consumed)) => {
            Ok((remaining, Some(consumed)))
        },
        Err(e) => {
            Ok(("No pair found", None))
        }
    }
}

// Takes: "123,456"
// Returns: (123, 456)
fn parse_pair(_input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(complete::i32, tag(","), complete::i32)(_input)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
