use miette::miette;
use nom::{character::complete::one_of, multi::fold_many0, IResult};

#[derive(Debug)]
enum State {
    NotMoved,
    Empty,
}

#[allow(clippy::never_loop)]
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (remaining, (mut disk, final_id)) =
        parse(_input).map_err(|e| miette!("Error parsing: {}", e))?;
    if !remaining.is_empty() {
        let (_, final_size) =
            parse_leftover(remaining).map_err(|e| miette!("Error parsing: {}", e))?;
        for _ in 0..final_size {
            disk.push(Some(final_id));
        }
    }

    // Reverse disk so we can iterate through the ones we want to move from the back.
    let disk_reversed: Vec<(State, Option<u64>)> = disk
        .clone()
        .iter()
        .rev()
        .map(|&slot| {
            if let Some(s) = slot {
                return (State::NotMoved, Some(s));
            }
            (State::Empty, None)
        })
        .collect();

    println!("disk: {:?}", disk);
    println!("inpuit: {:?}", _input);
    let (mut next_empty, _) = &disk.iter().enumerate().find(|(_, e)| e.is_none()).unwrap();

    for (i, (state, id)) in disk_reversed.iter().enumerate() {
        if matches!(state, State::Empty) {
            continue;
        }
        let disk_len = disk.len();
        if next_empty > disk_len - 1 - i {
            // We have overlapped.
            break;
        }
        disk[next_empty] = Some(id.unwrap());
        disk[disk_len - 1 - i] = None;
        let (new_next_empty, _) = &disk
            .iter()
            .skip(next_empty + 1)
            .enumerate()
            .find(|(_, e)| e.is_none())
            .unwrap();

        next_empty += *new_next_empty + 1;
    }

    let result: u64 = disk
        .iter()
        .enumerate()
        .map(|(i, e)| {
            if let Some(val) = e {
                return i as u64 * *val;
            }
            0
        })
        .sum();

    // let collected: Vec<u64> = disk.into_iter().flatten().collect();
    // let indices: Vec<u64> = (0..collected.len()).map(|x| x as u64).collect();

    Ok(result.to_string())
}

fn parse_leftover(_input: &str) -> IResult<&str, u64> {
    let (remaining, file_size) = one_of("0123456789")(_input)?;
    let file_size = file_size.to_digit(10).unwrap() as u64;
    Ok((remaining, file_size))
}

fn parse(_input: &str) -> IResult<&str, (Vec<Option<u64>>, u64)> {
    fold_many0(
        parse_pair,
        || (Vec::new(), 0),
        |(mut disk, id), (file_size, free_space)| {
            for _ in 0..file_size {
                disk.push(Some(id));
            }
            for _ in 0..free_space {
                disk.push(None)
            }
            (disk, id + 1)
        },
    )(_input)
}

// Return the next file and free space pair
fn parse_pair(_input: &str) -> IResult<&str, (u64, u64)> {
    let (remaining, file_size) = one_of("0123456789")(_input)?;
    let file_size = file_size.to_digit(10).unwrap() as u64; // todo propogate
    let (remaining, free_space) = one_of("0123456789")(remaining)?;
    let free_space = free_space.to_digit(10).unwrap() as u64;
    Ok((remaining, (file_size, free_space)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
