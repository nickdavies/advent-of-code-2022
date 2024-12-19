advent_of_code::solution!(9);

use advent_of_code::template::RunType;

use anyhow::{anyhow, Context, Result};
use aoc_lib::grid::{Direction, UnboundLocation};
use aoc_lib::parse::preamble::*;
use std::collections::BTreeSet;

fn parse(input: &str) -> Result<Vec<(Direction, usize)>> {
    parse_input(
        LineSplitter,
        ParseTuple2(
            SingleChar(ParseFn(|c| {
                Ok(match c {
                    'U' => Direction::North,
                    'R' => Direction::East,
                    'D' => Direction::South,
                    'L' => Direction::West,
                    other => {
                        return Err(anyhow!("Got unexpected direction char {:?}", other));
                    }
                })
            })),
            ParseFromStr,
            " ",
        ),
        input,
    )
    .context("Failed to parse input")
}

fn calc_tail(head: &UnboundLocation, tail: &UnboundLocation) -> UnboundLocation {
    let delta_row = tail.0 - head.0;
    let delta_col = tail.1 - head.1;

    match (delta_row.abs(), delta_col.abs()) {
        (0, 0) | (1, 0) | (0, 1) | (1, 1) => {
            return tail.clone();
        }
        _ => {}
    };
    if head.0 == tail.0 || delta_col.abs() > delta_row.abs() {
        UnboundLocation(head.0, head.1 + (delta_col) / delta_col.abs())
    } else if head.1 == tail.1 || delta_row.abs() > delta_col.abs() {
        UnboundLocation(head.0 + (delta_row) / delta_row.abs(), head.1)
    } else {
        UnboundLocation(
            head.0 + (delta_row) / delta_row.abs(),
            head.1 + (delta_col) / delta_col.abs(),
        )
    }
}

fn run(input: &str, rope_len: usize) -> Result<Option<usize>> {
    let data = parse(input)?;
    let mut locations: BTreeSet<UnboundLocation> = BTreeSet::new();
    let mut head = UnboundLocation(0, 0);
    let mut rope = Vec::new();
    for _ in 0..(rope_len - 1) {
        rope.push(UnboundLocation(0, 0));
    }

    locations.insert(rope.last().unwrap().clone());
    for (direction, distance) in &data {
        for _ in 0..*distance {
            head = head.go_direction(direction, 1);
            let mut prev = head.clone();
            let mut new_tail = Vec::with_capacity(rope.len());
            for knot in rope {
                let new_knot = calc_tail(&prev, &knot);
                new_tail.push(new_knot.clone());
                prev = new_knot;
            }

            rope = new_tail;
            locations.insert(rope.last().unwrap().clone());
        }
    }
    Ok(Some(locations.len()))
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    run(input, 2)
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    run(input, 10)
}

#[cfg(test)]
mod tests_day_9 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(13);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(1);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two_extra() -> anyhow::Result<()> {
        let expected = Some(36);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 3);
        assert!(expected.is_none() || !input.is_empty(), "example 3 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
