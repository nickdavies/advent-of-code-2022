advent_of_code::solution!(1);

use advent_of_code::template::RunType;

use anyhow::{anyhow, Context};
use aoc_lib::parse::*;

use std::collections::BinaryHeap;

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let data: Vec<Vec<u32>> = parse_input(
        LineGroupSplitter::blankline(),
        ParseVec(ParseFromStr),
        input,
    )
    .context("failed to parse data")?;

    Ok(data.into_iter().map(|e| e.into_iter().sum()).max())
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let data: Vec<Vec<u32>> = parse_input(
        LineGroupSplitter::blankline(),
        ParseVec(ParseFromStr),
        input,
    )
    .context("failed to parse data")?;

    let mut h: BinaryHeap<u32> = BinaryHeap::new();
    for elf in data {
        h.push(elf.into_iter().sum());
    }
    if h.len() < 3 {
        return Err(anyhow!("Not enough elves!"));
    }
    Ok(Some(h.pop().unwrap() + h.pop().unwrap() + h.pop().unwrap()))
}

#[cfg(test)]
mod tests_day_1 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, Some(24000));
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, Some(45000));
        Ok(())
    }
}
