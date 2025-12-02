use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, newline},
    multi::separated_list1,
    IResult,
};

use miette::miette;

#[derive(Debug)]
enum Direction {
    L,
    R,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (remaining, rotations) = parse(_input).map_err(|e| miette!("Parse failed {}", e))?;
    assert!(remaining.len() == 0, "Some left unparsed!");

    let mut z_count = 0;
    let mut current_position = 50;

    for (_, (dir, dist)) in rotations.iter().enumerate() {
        let mut pass_zero = 0;
        match dir {
            Direction::L => {
                println!(
                    "current_position - dist: {current_position} - {dist} == {}, rem_euclid(100) == {}",
                    (current_position - dist),
                    (current_position - dist).rem_euclid(100)
                );
                if current_position - dist <= 0 {
                    let extra_subtraction = if current_position == 0 { 0 } else { 100 };
                    pass_zero += ((current_position - dist - extra_subtraction) / 100).abs();
                }
                current_position = (current_position - dist).rem_euclid(100);
            }
            Direction::R => {
                println!(
                    "current_position + dist: {current_position} + {dist} == {}, rem_euclid(100) == {}",
                    (current_position + dist),
                    (current_position + dist).rem_euclid(100)
                );
                if current_position + dist >= 100 {
                    pass_zero += ((current_position + dist) / 100).abs();
                }
                current_position = (current_position + dist).rem_euclid(100);
            }
        }

        z_count += pass_zero;
        println!("current_position: {current_position}. z_count = {z_count}");
    }

    Ok(z_count.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<(Direction, i32)>> {
    let (remaining, parsed) = separated_list1(newline, instruction)(input)?;
    assert!(remaining.len() == 0);
    Ok((remaining, parsed))
}

fn instruction(input: &str) -> IResult<&str, (Direction, i32)> {
    let (remaining, dir) = alt((tag("L"), tag("R")))(input)?;
    let (remaining, dist) = i32(remaining)?;
    let dir = match dir {
        "L" => Direction::L,
        "R" => Direction::R,
        _ => panic!("Parsed L or R but not found"),
    };
    Ok((remaining, (dir, dist)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
R300
L1
L99
R14
L82
";
        // assert_eq!("5", process(input)?);
        // process(input)?;
        // assert!(false);
        Ok(())
    }
}
