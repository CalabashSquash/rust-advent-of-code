use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace1, u128},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range {
    low: u128,
    high: u128,
}

// Assumes sorted
fn scan_in_front(from_idx: usize, range: Range, valid_id_ranges: &mut Vec<Option<Range>>) {
    for i in from_idx..valid_id_ranges.len() {
        if let Some(Range { low, high }) = valid_id_ranges[i] {
            if low > range.high {
                return;
            }
            if high <= range.high {
                valid_id_ranges[i] = None;
                continue;
            }
            valid_id_ranges[i] = Some(Range {
                low: range.high + 1,
                high,
            })
        }
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, (mut valid_id_ranges, _)) =
        all_consuming(parse)(_input).map_err(|e| miette::miette!("Error: {e}"))?;
    valid_id_ranges.sort_by(|&l, &r| {
        if l == None || r == None {
            panic!("None at sorting time!");
        }
        if let Some(l) = l {
            if let Some(r) = r {
                if l.low < r.low {
                    return Ordering::Less;
                } else if l.low > r.low {
                    return Ordering::Greater;
                }
                if l.high > r.high {
                    return Ordering::Greater;
                } else if l.high < r.high {
                    return Ordering::Less;
                } else {
                    return Ordering::Equal;
                }
            }
        }
        panic!("None???");
    });

    for idx in 0..valid_id_ranges.len() {
        if let Some(range) = valid_id_ranges[idx] {
            scan_in_front(idx + 1, range, &mut valid_id_ranges);
        } else {
            continue;
        }
    }

    Ok(valid_id_ranges
        .iter()
        .map(|range| match range {
            None => 0,
            Some(Range { low, high }) => high - low + 1,
        })
        .sum::<u128>()
        .to_string())
}

fn parse(input: &str) -> IResult<&str, (Vec<Option<Range>>, Vec<u128>)> {
    let (input, valid_id_ranges) = separated_list1(
        line_ending,
        separated_pair(u128, tag("-"), u128).map(|(low, high)| Some(Range { low, high })),
    )(input)?;

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
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
