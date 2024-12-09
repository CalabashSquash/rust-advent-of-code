mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
use std::error::Error;
use std::fs::read_to_string;

// use crate::day1::{problem1, problem2};
use crate::day2::{problem1 as d2p1, problem2 as d2p2};
use crate::day3::problem2 as d3p2;
use crate::day4::{problem1 as d4p1, problem2 as d4p2};
use crate::day5::problem1 as d5p1;
use crate::day6::problem1 as d6p1;
use crate::day7::{problem1 as d7p1, process};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_till, take_until, take_while};
use nom::character::complete::{alpha0, alphanumeric1, anychar, digit1, i64, space0};
use nom::combinator::value;
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::{number, IResult};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // let (remaining, output) = do_nothing_parser("input(hello)")?;
    // let result = simple_tag_parser("input(hello)")?;
    // let result = alpha0_parser("input(hello)")?;
    // let result = alpha0_parser("()123")?;

    // let input = "abmul(3,4)don't()abcdo()";
    // // let result = alt_parser(input)?;
    // let input = "mul(don't()do()";
    // let result = alt_parser(input)?;
    // let input = "mul(don't()do()";
    // let result = tuple_parser(input)?;

    // // println!("{:#?}", problem1_attempt_2());

    // let result = take_until_parser("hello. hi. yeet")?;
    // println!("{:#?}", result);

    // // let result = take_while_parser(". hi. yeet")?;
    // // println!("{:#?}", result);

    // assert_eq!(terminated_parser("defabcYEETefg"), Ok(("", "abc")));

    // println!("{:#?}", problem1_attempt_2());
    // println!("Day 1 problem 1: {}", problem1());
    // println!("Day 1 problem 2: {}", problem2());
    // println!("Day 2 problem 2: {}", d2p2());
    // println!("Day 3 problem 2: {}", d3p2());
    // println!("Day 4 problem 1: {}", d4p1());
    // println!("Day 4 problem 2: {}", d4p2());
    // println!("Day 6 problem 1: {}", d6p1());
    let text = std::fs::read_to_string("src/day7.txt").unwrap();
    let text = text.as_str();
    println!("Day 7 problem 1: {:?}", process(text));
    // assert_eq!(remaining, "input");
    Ok(())
}

fn terminated_parser(text: &str) -> IResult<&str, &str> {
    terminated(take_until("abc"), take_until("efg"))(text)
}

fn take_while_parser(text: &str) -> IResult<&str, &str> {
    take_while(|c| c == '.' || c == ' ')(text)
}

fn take_until_parser(text: &str) -> IResult<&str, &str> {
    take_until(".")(text)
}

fn parse_file(filename: &str) -> String {
    read_to_string(filename).unwrap()
}

pub fn problem1_attempt_2() -> i64 {
    let text = parse_file("src/day3_test.txt");
    let mut text = text.as_str();

    let running_total = 0;

    loop {
        match parse_mul(text) {
            Ok((remaining, consumed)) => {
                println!("Hello");
                match parse_mul_args(remaining) {
                    Ok((remaining, (l, r))) => {
                        println!("remaining: {}, consumed: {:#?}", remaining, consumed);
                        println!("Multiplying {} and {}", l, r);
                        text = remaining;
                    }
                    Err(e) => {
                        println!("Err: {}", e);
                        text = remaining;
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
    }
    (0)
}

fn optional_space_comma(text: &str) -> IResult<&str, (&str, &str)> {
    tuple((tag(","), space0))(text)
}

fn parse_mul_args(text: &str) -> IResult<&str, (i64, i64)> {
    delimited(
        tag("("),
        separated_pair(i64, optional_space_comma, i64),
        tag(")"),
    )(text)
}

fn parse_mul(text: &str) -> IResult<&str, &str> {
    tag("mul")(text)
}

fn tuple_parser(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((alt_parser, alt_parser, alt_parser))(input)
}

fn alt_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("mul("), tag("don't()"), tag("do()")))(input)
}

fn alpha0_parser(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

fn simple_tag_parser(input: &str) -> IResult<&str, &str> {
    tag("input(")(input)
}

fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
    Ok((input, ""))
}
