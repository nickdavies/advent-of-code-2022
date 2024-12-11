advent_of_code::solution!(4);

use advent_of_code::template::RunType;

use anyhow::{Context, Result};
use aoc_lib::parse::preamble::*;

fn sort_pairs(a: (u32, u32), b: (u32, u32)) -> ((u32, u32), (u32, u32)) {
    if a.0 == b.0 {
        if a.1 >= b.1 {
            (a, b)
        } else {
            (b, a)
        }
    } else if a.0 < b.0 {
        (a, b)
    } else {
        (b, a)
    }
}

fn run(input: &str, overlap_fn: fn((u32, u32), (u32, u32)) -> bool) -> Result<u32> {
    let data: Vec<((u32, u32), (u32, u32))> = parse_input(
        LineSplitter,
        ParseTuple2(
            ParseTuple2(ParseFromStr, ParseFromStr, "-"),
            ParseTuple2(ParseFromStr, ParseFromStr, "-"),
            ",",
        ),
        input,
    )
    .context("failed to parse input")?;

    let mut out = 0;
    for (a, b) in data {
        let (a, b) = sort_pairs(a, b);
        if overlap_fn(a, b) {
            out += 1;
        }
    }

    Ok(out)
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    Ok(Some(run(input, |a, b| b.1 <= a.1)?))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    Ok(Some(run(input, |a, b| b.1 >= a.0 && b.0 <= a.1)?))
}

#[cfg(test)]
mod tests_day_4 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(2);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(4);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
