use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace0},
    combinator::opt,
    multi::many0,
    sequence::terminated,
    IResult,
};

#[derive(Debug)]
struct Coordinates {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Machine {
    button_a: Coordinates,
    button_b: Coordinates,
    prize: Coordinates,
}

// Brute force solution
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, machines) = parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;

    let result: u32 = machines
        .iter()
        .map(|machine| find_cheapest(machine))
        .flatten()
        .into_iter()
        .sum();
    Ok(result.to_string())
}

fn find_cheapest(machine: &Machine) -> Option<u32> {
    let mut min: u32 = std::u32::MAX;
    for a in 0..1000 {
        for b in 0..1000 {
            let x = a * machine.button_a.x + b * machine.button_b.x;
            let y = a * machine.button_a.y + b * machine.button_b.y;
            if x == machine.prize.x && y == machine.prize.y {
                // Found a valid solution
                let cost = 3 * a + b;
                if cost < min {
                    min = cost;
                }
            }
        }
    }
    return if min == std::u32::MAX {
        None
    } else {
        Some(min)
    };
}

fn parse(_input: &str) -> IResult<&str, Vec<Machine>> {
    many0(parse_machine)(_input)
}

fn parse_x_y(_input: &str) -> IResult<&str, Coordinates> {
    let (remaining, _) = tag("X+")(_input)?;
    let (remaining, x_dist) = complete::u32(remaining)?;
    let (remaining, _) = tag(", Y+")(remaining)?;
    let (remaining, y_dist) = complete::u32(remaining)?;
    Ok((
        remaining,
        Coordinates {
            x: x_dist,
            y: y_dist,
        },
    ))
}

fn parse_prize(_input: &str) -> IResult<&str, Coordinates> {
    let (remaining, _) = tag("Prize: X=")(_input)?;
    let (remaining, x) = complete::u32(remaining)?;
    let (remaining, _) = tag(", Y=")(remaining)?;
    let (remaining, y) = complete::u32(remaining)?;
    Ok((remaining, Coordinates { x, y }))
}

fn parse_machine(_input: &str) -> IResult<&str, Machine> {
    let (remaining, _) = tag("Button A: ")(_input)?;
    let (remaining, button_a) = terminated(parse_x_y, line_ending)(remaining)?;
    let (remaining, _) = tag("Button B: ")(remaining)?;
    let (remaining, button_b) = terminated(parse_x_y, line_ending)(remaining)?;
    let (remaining, prize) = terminated(parse_prize, opt(line_ending))(remaining)?;
    let (remaining, _) = multispace0(remaining)?;

    Ok((
        remaining,
        Machine {
            button_a,
            button_b,
            prize,
        },
    ))
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
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
