use miette::miette;
use nom::{
    character::complete::{self, multispace1},
    multi::separated_list0,
    IResult,
};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, mut nums) = parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;
    // Naive solution. See part2.rs for optimised.
    for _ in 0..25 {
        let mut necc_vec = Vec::new();
        for num in nums.iter() {
            if *num == 0 {
                necc_vec.push(1);
            } else if num.to_string().len() % 2 == 0 {
                let num_as_string = num.to_string();
                let (left, right) = num_as_string.split_at(num_as_string.len() / 2);
                necc_vec.push(left.parse::<u64>().unwrap());
                necc_vec.push(right.parse::<u64>().unwrap());
            } else {
                necc_vec.push(num * 2024);
            }
        }
        nums = necc_vec;
    }

    Ok(nums.len().to_string())
}

fn parse(_input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(multispace1, complete::u64)(_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
