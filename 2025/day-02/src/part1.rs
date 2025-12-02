use miette::miette;
use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list1,
    sequence::separated_pair, IResult,
};

#[derive(Debug)]
struct Range {
    low: u64,
    high: u64,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, ranges) = parse(_input).map_err(|err| miette!("Error: {err}"))?;

    let sum_of_invalid_ids: u64 = ranges
        .iter()
        .map(|range| {
            (range.low..range.high + 1)
                .into_iter()
                .filter(|&id| is_invalid(id))
                .sum::<u64>()
        })
        .sum();

    Ok(sum_of_invalid_ids.to_string())
}

fn is_invalid(num: u64) -> bool {
    let num = num.to_string();

    if num.len() % 2 != 0 {
        return false;
    }

    num.get(..num.len() / 2) == num.get(num.len() / 2..)
}

fn parse(input: &str) -> IResult<&str, Vec<Range>> {
    let (input, ranges) = separated_list1(tag(","), parse_range)(input)?;
    Ok((input, ranges))
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, (low, high)) = separated_pair(u64, tag("-"), u64)(input)?;
    return Ok((
        input,
        Range {
            low: low,
            high: high,
        },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // let input = "11-2,9999755745207-999999755766099,11111111-11111112,11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        // assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}
