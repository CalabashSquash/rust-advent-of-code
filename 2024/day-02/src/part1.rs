use miette::miette;
use nom::{
    character::complete::{self, line_ending, multispace1, space1},
    combinator::opt,
    multi::{many1, separated_list0, separated_list1},
    sequence::terminated,
    IResult,
};

#[derive(Debug)]
enum Direction {
    Increasing,
    Decreasing,
    OnlyFirst,
    NotStarted,
    Invalid,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, reports) = parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;
    let safe = reports.iter().fold(0, |safe_count, report| {
        let report_result = report.iter().fold(
            (Direction::NotStarted, 0),
            |(direction, previous_level), level| match direction {
                Direction::NotStarted => (Direction::OnlyFirst, *level),
                Direction::OnlyFirst => {
                    if *level > previous_level {
                        return (Direction::Increasing, *level);
                    } else if *level == previous_level {
                        return (Direction::Invalid, *level);
                    }
                    (Direction::Decreasing, *level)
                }
                Direction::Increasing => {
                    if *level > previous_level && *level - previous_level < 4 {
                        return (Direction::Increasing, *level);
                    }
                    (Direction::Invalid, *level)
                }
                Direction::Decreasing => {
                    if *level < previous_level && previous_level - *level < 4 {
                        return (Direction::Decreasing, *level);
                    }
                    (Direction::Invalid, *level)
                }
                Direction::Invalid => (Direction::Invalid, *level),
            },
        );
        match report_result {
            (Direction::Invalid, _) => {
                return safe_count;
            }
            _ => {
                return safe_count + 1;
            }
        }
    });
    Ok(safe.to_string())
}

fn parse(_input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    many1(parse_line)(_input)
}

fn parse_line(_input: &str) -> IResult<&str, Vec<i32>> {
    terminated(separated_list1(space1, complete::i32), opt(line_ending))(_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
