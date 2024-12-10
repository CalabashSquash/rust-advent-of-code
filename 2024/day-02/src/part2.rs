use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{self, line_ending, space1},
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

#[derive(Debug)]
enum Direction {
    Increasing,
    Decreasing,
    NotStarted,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, reports) = parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;

    let safe: usize = reports
        .iter()
        .filter(|report| {
            let safe_combination = find_safe_combination(report);
            safe_combination.is_some()
        })
        .count();

    Ok(safe.to_string())
}

fn find_safe_combination(report: &Vec<i32>) -> Option<()> {
    let safe_permutation = report.iter().combinations(report.len() - 1).find(|p| {
        if is_report_safe(p).is_ok() {
            return true;
        }
        return false;
    });

    if let Some(_) = safe_permutation {
        return Some(());
    } else {
        None
    }
}

fn is_report_safe(report: &Vec<&i32>) -> Result<(), ()> {
    let mut direction = Direction::NotStarted;
    // Find an invalid pair.
    let has_invalidity = report.iter().tuple_windows::<(_, _)>().find(|(&l, &r)| {
        if (l - r).abs() > 3 {
            return true;
        }
        match direction {
            Direction::NotStarted => {
                if l < r {
                    direction = Direction::Increasing;
                    return false;
                } else if l > r {
                    direction = Direction::Decreasing;
                    return false;
                } else {
                    return true;
                }
            }
            Direction::Decreasing => {
                if l <= r {
                    return true;
                }
            }
            Direction::Increasing => {
                if l >= r {
                    return true;
                }
            }
        }

        return false;
    });

    if let Some(_) = has_invalidity {
        return Err(());
    }

    Ok(())
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
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
