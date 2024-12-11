use std::collections::HashMap;

use miette::miette;
use nom::{
    character::complete::{self, multispace1},
    multi::separated_list0,
    IResult,
};

struct Stones {
    cache: HashMap<(u64, u64), u64>,
}

impl Stones {
    fn new() -> Stones {
        let cache = HashMap::new();
        Stones { cache }
    }

    fn how_many_does_generate(&mut self, num: u64, current_depth: u64, max_depth: u64) -> u64 {
        if current_depth == max_depth {
            return 1;
        }
        if let Some(results_for_number) = self.cache.get(&(num, current_depth)) {
            // cache hit
            return *results_for_number;
        }

        if num == 0 {
            return self.how_many_does_generate(1, current_depth + 1, max_depth);
        } else if num.to_string().len() % 2 == 0 {
            let num_as_string = num.to_string();
            let (left, right) = num_as_string.split_at(num_as_string.len() / 2);
            let generated_from = self.how_many_does_generate(
                left.parse::<u64>().unwrap(),
                current_depth + 1,
                max_depth,
            ) + self.how_many_does_generate(
                right.parse::<u64>().unwrap(),
                current_depth + 1,
                max_depth,
            );
            self.cache.insert((num, current_depth), generated_from);
            return generated_from;
        } else {
            let generated_from =
                self.how_many_does_generate(num * 2024, current_depth + 1, max_depth);
            self.cache.insert((num, current_depth), generated_from);
            return generated_from;
        }
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, nums) = parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;
    println!("{:?}", nums);
    let mut stones = Stones::new();
    let res: u64 = nums
        .iter()
        .map(|num| stones.how_many_does_generate(*num, 0, 75))
        .sum();

    Ok(res.to_string())
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
