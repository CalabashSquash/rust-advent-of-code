use miette::miette;
use nom::{
    character::complete::{digit1, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    IResult,
};

const NUM_DIGITS: usize = 12;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, banks) = all_consuming(parse)(_input).map_err(|e| miette!("Error: {e}"))?;
    let x: u64 = banks.iter().map(|bank| max(bank)).sum();
    let y: u64 = banks
        .iter()
        .map(|&bank| second_solution(bank.to_string()))
        .sum();
    println!("second solution:{y}");
    return Ok(x.to_string());
}

fn second_solution(bank: String) -> u64 {
    let mut bank_copy: Vec<char> = bank.chars().collect();
    for _ in 0..bank.len() - 12 {
        for j in 0..bank.len() {
            if j == bank_copy.len() - 1 {
                // last elem
                bank_copy.remove(j);
                break;
            } else if bank_copy[j..j + 1] < bank_copy[j + 1..j + 2] {
                bank_copy.remove(j);
                break;
            }
        }
    }
    bank_copy.iter().collect::<String>().parse::<u64>().unwrap()
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, digit1)(input)
}

fn max(bank: &str) -> u64 {
    let mut running_total: Vec<u64> = Vec::new();
    let mut prev_best_idx_plus_one = 0;

    for i in 1..=NUM_DIGITS {
        let Best {
            digit: new_best,
            idx: new_best_idx,
        } = get_best(&bank[(prev_best_idx_plus_one)..bank.len() - (NUM_DIGITS - i)]);

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
    let (idx, digit) = search
        .chars()
        .enumerate()
        // Can't use Iterator::max because that returns the last max. We need first max.
        .fold((0, 0), |(max_idx, max), (idx, digit)| {
            let digit = digit.to_string().parse::<u64>().unwrap();
            if digit > max {
                return (idx, digit);
            }
            (max_idx, max)
        });
    Best { digit: digit, idx }
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
