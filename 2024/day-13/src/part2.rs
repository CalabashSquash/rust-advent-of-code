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
    x: i64,
    y: i64,
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

    let result: i64 = machines
        .iter()
        .map(|machine| find_cheapest(machine))
        .flatten()
        .sum();
    Ok(result.to_string())
}

fn find_cheapest(machine: &Machine) -> Option<i64> {
    // X: a*machine.button_a.x + b*machine.button_b.x = machine.prize.x
    // Y: a*machine.button_a.y + b*machine.button_b.y = machine.prize.y
    //1X: (a*machine.button_a.x * machine.button_b.y) + (b*machine.button_b.x * machine.button_b.y) = machine.prize.x * machine.button_b.y
    //1Y: (a*machine.button_a.y * machine.button_b.x) + (b*machine.button_b.y * machine.button_b.x) = machine.prize.y * machine.button_b.x

    //1D: (a*machine.button_a.x * machine.button_b.y) - (a*machine.button_a.y * machine.button_b.x) = (machine.prize.x * machine.button_b.y) - (machine.prize.y * machine.button_b.x)
    //2D: a*(machine.button_a.x*machine.button_b.y - machine.button_a.y*machine.button_b.x) = (machine.prize.x * machine.button_b.y) - (machine.prize.y * machine.button_b.x)
    //3D: a == divide right side by (machine.button_a.x*machine.button_b.y - machine.button_a.y*machine.button_b.x)
    //    If not a whole number (x%y!=0), no solution.

    let x_a_coeff = machine.button_a.x * machine.button_b.y;
    // let x_b_coeff = machine.button_b.x * machine.button_b.y;
    let x_solution =   machine.prize.x * machine.button_b.y;

    let y_a_coeff = machine.button_a.y * machine.button_b.x;
    // let y_b_coeff = machine.button_b.y * machine.button_b.x;
    let y_solution =   machine.prize.y * machine.button_b.x;

    let diff_a_coeff = x_a_coeff - y_a_coeff;
    let diff_solution = x_solution - y_solution;
    let a = diff_solution / diff_a_coeff;
    if diff_solution % diff_a_coeff != 0 {
        return None;
    }

    //e.g. 22b = 8400 - 94a
    let b_times_bx = machine.prize.x - a * machine.button_a.x;
    let b = b_times_bx / machine.button_b.x;
    if b_times_bx % machine.button_b.x != 0 {
        return None
    }

    Some(3 * a + b)
}

fn parse(_input: &str) -> IResult<&str, Vec<Machine>> {
    many0(parse_machine)(_input)
}

fn parse_x_y(_input: &str) -> IResult<&str, Coordinates> {
    let (remaining, _) = tag("X+")(_input)?;
    let (remaining, x_dist) = complete::i64(remaining)?;
    let (remaining, _) = tag(", Y+")(remaining)?;
    let (remaining, y_dist) = complete::i64(remaining)?;
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
    let (remaining, x) = complete::i64(remaining)?;
    let (remaining, _) = tag(", Y=")(remaining)?;
    let (remaining, y) = complete::i64(remaining)?;
    Ok((remaining, Coordinates { x: x + 10000000000000, y: y + 10000000000000 }))
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
        assert_eq!("", process(input)?);
        Ok(())
    }
}
