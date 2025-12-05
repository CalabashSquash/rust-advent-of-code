use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, u128},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, (valid_ids, ids)) =
        all_consuming(parse)(_input).map_err(|e| miette::miette!("Error: {e}"))?;

    Ok(ids
        .iter()
        .filter(|&&id| is_valid_id(id, &valid_ids))
        .count()
        .to_string())
}

fn is_valid_id(id: u128, valid_ids: &Vec<(u128, u128)>) -> bool {
    valid_ids
        .iter()
        .find(|(low, high)| id >= *low && id <= *high)
        .is_some()
}

fn parse(input: &str) -> IResult<&str, (Vec<(u128, u128)>, Vec<u128>)> {
    let (input, valid_id_ranges) =
        separated_list1(line_ending, separated_pair(u128, tag("-"), u128))(input)?;

    let (input, _) = multispace1(input)?;

    let (remaining, ids) = separated_list1(line_ending, u128)(input)?;

    Ok((remaining, (valid_id_ranges, ids)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
