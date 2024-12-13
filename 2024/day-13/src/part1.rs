use nom::{bytes::complete::tag, character::complete::{self, line_ending, multispace0}, combinator::opt, multi::many0, sequence::terminated, IResult};
use miette::miette;

#[derive(Debug)]
struct Coordinates {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Machine {
    button_a: Coordinates,
    button_b: Coordinates,
    prize: Coordinates
}

// Brute force solution
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (remaining, machines) = parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;
    println!("remaining: {}",remaining);
    println!("machines: {:#?}",machines);

    
    todo!("day 01 - part 1");
}

fn parse(_input: &str) -> IResult<&str, Vec<Machine>> {
    many0(parse_machine)(_input)
}


fn parse_x_y(_input: &str) -> IResult<&str, Coordinates> {
    let (remaining, _) = tag("X+")(_input)?;
    let (remaining, x_dist) = complete::u32(remaining)?;
    let (remaining, _) = tag(", Y+")(remaining)?;
    let (remaining, y_dist) = complete::u32(remaining)?;
    Ok((remaining, Coordinates {x: x_dist, y: y_dist}))
}

fn parse_prize(_input: &str) -> IResult<&str, Coordinates> {
    let (remaining, _) = tag("Prize: X=")(_input)?;
    let (remaining, x) = complete::u32(remaining)?;
    let (remaining, _) = tag(", Y=")(remaining)?;
    let (remaining, y) = complete::u32(remaining)?;
    Ok((remaining, Coordinates {x, y}))
}

fn parse_machine(_input: &str) -> IResult<&str, Machine> {
    let (remaining, _) = tag("Button A: ")(_input)?;
    let (remaining, button_a) = terminated(parse_x_y, line_ending)(remaining)?;
    let (remaining, _) = tag("Button B: ")(remaining)?;
    let (remaining, button_b) = terminated(parse_x_y, line_ending)(remaining)?;
    let (remaining, prize) = terminated(parse_prize, opt(line_ending))(remaining)?;
    let (remaining, _) = multispace0(remaining)?;

    Ok((remaining, Machine {button_a, button_b, prize}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
