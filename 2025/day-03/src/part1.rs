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
    let x: u32 = banks.iter().map(|bank| max(bank)).sum();
    return Ok(x.to_string());
}

fn max(bank: &str) -> u32 {
    let mut largest = 0;
    let mut largest_index = 0;
    let mut second_largest = 0;
    for i in 0..bank.len() - 1 {
        let current_digit: u32 = bank[i..i + 1].to_string().parse().unwrap();
        if current_digit > largest {
            largest = current_digit;
            largest_index = i;
        }
    }
    for i in largest_index + 1..bank.len() {
        let current_digit: u32 = bank[i..i + 1].to_string().parse().unwrap();
        if current_digit > second_largest {
            second_largest = current_digit;
        }
    }

    return format!("{}{}", largest.to_string(), second_largest.to_string())
        .parse()
        .unwrap();
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, digit1)(input)
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
        assert_eq!("357", process(input)?);
        Ok(())
    }
}
