use miette::miette;
use nom::{
    character::complete::{digit1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    IResult,
};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, banks) = all_consuming(parse)(_input).map_err(|e| miette!("Error: {e}"))?;
    let x: u64 = banks.iter().map(|bank| max(bank)).sum();
    return Ok(x.to_string());
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, digit1)(input)
}

fn max(bank: &str) -> u64 {
    let mut running_total: Vec<u64> = Vec::new();
    let mut prev_best_idx_plus_one = 0;

    for i in 0..12 {
        let Best {
            digit: new_best,
            idx: new_best_idx,
        } = get_best(&bank[(prev_best_idx_plus_one)..bank.len() - (11 - i)]);

        running_total.push(new_best);
        prev_best_idx_plus_one += new_best_idx + 1; // We need to increment on top of the previous one, because the new best index is relative to the beginning of the truncated sub-string.
    }
    let result_str = running_total
        .iter()
        .map(|digit| digit.to_string())
        .collect::<String>();

    return result_str.parse::<u64>().unwrap();
}

struct Best {
    digit: u64,
    idx: usize,
}
fn get_best(search: &str) -> Best {
    let mut largest = 0;
    let mut largest_index = 0;
    for i in 0..search.len() {
        let current_digit: u64 = search[i..i + 1].to_string().parse().unwrap();
        if current_digit > largest {
            largest = current_digit;
            largest_index = i;
        }
    }

    return Best {
        digit: largest,
        idx: largest_index,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }
}
