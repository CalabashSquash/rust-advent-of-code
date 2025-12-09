use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{line_ending, multispace0},
    },
    combinator::all_consuming,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

const X_LARGEST_CIRCUITS: usize = 3;
const X_SHORTEST_CONNECTIONS: usize = 1000;

// #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
type Point = (u64, u64, u64);

// Return true if they were separate circuits.
// Return false if they were already in the same circuit
// This is basically to help me debug ^
fn add_point_pair(dry_run: bool, p1: Point, p2: Point, circuits: &mut Vec<Vec<Point>>) -> bool {
    let p1_circuit = (0..circuits.len())
        .into_iter()
        .find(|idx| circuits[*idx].iter().find(|point| **point == p1).is_some());
    let p2_circuit = (0..circuits.len())
        .into_iter()
        .find(|idx| circuits[*idx].iter().find(|point| **point == p2).is_some());

    if let Some(p1_idx) = p1_circuit {
        if let Some(p2_idx) = p2_circuit {
            if p1_idx != p2_idx {
                if !dry_run {
                    let mut p2_copy = circuits[p2_idx].clone();
                    circuits[p1_idx].append(&mut p2_copy);
                    circuits.remove(p2_idx);
                }
                return true;
            }
            return false;
            // If they are the same circuit, do nothing
        } else {
            // p2 not in any circuit. Add it to p1's.
            if !dry_run {
                circuits[p1_idx].push(p2);
            }
            return true;
        }
    } else {
        if let Some(p2_idx) = p2_circuit {
            // p1 not in any circuit. Add to p2's
            if !dry_run {
                circuits[p2_idx].push(p1);
            }
            return true;
        }
        // Neither point are in a circuit. Create new circuit
        if !dry_run {
            circuits.push(vec![p1, p2]);
        }
        return true;
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let (_, coords) = all_consuming(parse)(_input).map_err(|e| miette::miette!("Error! {e}"))?;

    let mut circuits: Vec<Vec<Point>> = vec![];

    let mut already_added: HashMap<(&Point, &Point), bool> = HashMap::new();
    let distances: Vec<(u64, &Point, &Point)> = (0..coords.len())
        .into_iter()
        .map(|point| {
            let mut distances = vec![];
            for inner_point in 0..coords.len() {
                distances.push((
                    get_distance(&coords[point], &coords[inner_point]),
                    &coords[point],
                    &coords[inner_point],
                ));
            }
            distances
        })
        .flatten()
        .sorted_by(|(dist1, _, _), (dist2, _, _)| u64::cmp(dist1, dist2))
        .collect();

    // Transform distances into only the pairs we are concerned with.
    let distances = distances
        .into_iter()
        .skip(20)
        .step_by(2)
        .take(X_SHORTEST_CONNECTIONS)
        .collect::<Vec<(u64, &Point, &Point)>>();

    for (_, p1, p2) in distances {
        if already_added.contains_key(&(p1, p2)) || already_added.contains_key(&(p2, p1)) {
            continue;
        }
        let add_res = add_point_pair(false, *p1, *p2, &mut circuits);
        already_added.insert((p1, p2), true);
    }

    let x = circuits
        .iter()
        .map(|circuit| circuit.len())
        .sorted()
        .rev()
        .take(X_LARGEST_CIRCUITS)
        .fold(1 as u128, |accum, len| accum * (len as u128));

    Ok(x.to_string())
}

fn get_distance(p1: &Point, p2: &Point) -> u64 {
    let operand = (p2.0 as i128 - p1.0 as i128).pow(2)
        + (p2.1 as i128 - p1.1 as i128).pow(2)
        + (p2.2 as i128 - p1.2 as i128).pow(2);

    operand as u64
}

fn parse(input: &str) -> IResult<&str, Vec<Point>> {
    terminated(separated_list1(line_ending, parse_coord), multispace0)(input)
}

fn parse_coord(input: &str) -> IResult<&str, Point> {
    let (input, coords) = separated_list1(tag(","), character::complete::u64)(input)?;
    assert!(coords.len() == 3);
    Ok((input, (coords[0], coords[1], coords[2])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
