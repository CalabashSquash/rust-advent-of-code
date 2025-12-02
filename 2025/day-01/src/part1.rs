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
    // let (remaining, rotations) = parse(_input).map_err(|e| miette!("Parse failed {}", e))?;
    let (remaining, rotations) = parse2(_input).map_err(|e| miette!("Parse failed {}", e))?;
    assert!(remaining.len() == 0, "Some left unparsed!");

    let (final_z_count, _) =
        rotations
            .iter()
            .fold((0, 50), |(z_count, current_position), (dir, dist)| {
                let new_current_position;
                match dir {
                    Direction::L => {
                        new_current_position = (current_position - dist).rem_euclid(100);
                    }
                    Direction::R => {
                        new_current_position = (current_position + dist).rem_euclid(100);
                    }
                }

                let mut new_z_count = z_count;
                if new_current_position == 0 {
                    new_z_count = z_count + 1;
                }

                (new_z_count, new_current_position)
            });

    Ok(final_z_count.to_string())
}

fn parse2(input: &str) -> IResult<&str, Vec<(Direction, i32)>> {
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
L1
L99
R14
L82";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
