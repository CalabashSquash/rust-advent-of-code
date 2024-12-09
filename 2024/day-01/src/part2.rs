#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (col1, col2) = parse_file(_input);

    let similarity: usize = col1
        .iter()
        .map(
            |x| {
                (*x as usize)
                    * col2
                        .iter()
                        .filter(|n| *n == x) // O(n)
                        .count()
            }, // O(n)
        )
        .sum();

    Ok(similarity.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}

fn parse_file(_input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut first_col = Vec::new();
    let mut second_col = Vec::new();

    for line in _input.lines() {
        let mut numbers = line.split_whitespace();
        first_col.push(
            numbers
                .next()
                .expect("No first number in row")
                .parse::<i64>()
                .expect("Could not parse first number"),
        );
        second_col.push(
            numbers
                .next()
                .expect("No second number in row")
                .parse::<i64>()
                .expect("Could not parse second number"),
        );
    }
    (first_col, second_col)
}
