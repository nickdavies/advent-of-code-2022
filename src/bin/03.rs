advent_of_code::solution!(3);

use advent_of_code::template::RunType;

use aoc_lib::parse::preamble::*;

use anyhow::{anyhow, Context, Result};
use std::collections::BTreeSet;

fn get_priority(c: char) -> Result<u32> {
    Ok(match c {
        'a'..='z' => u32::from(c) - u32::from('a') + 1,
        'A'..='Z' => u32::from(c) - u32::from('A') + 27,
        other => {
            return Err(anyhow!("got unexpected char: {}", other));
        }
    })
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let data: Vec<(Vec<char>, Vec<char>)> = parse_input(
        LineSplitter,
        SplitMiddle(Chars(Identity), Chars(Identity)),
        input,
    )
    .context("failed to parse input")?;

    let mut out = 0;
    for (l, r) in data {
        let l: BTreeSet<char> = l.into_iter().collect();
        let r: BTreeSet<char> = r.into_iter().collect();

        for v in l.intersection(&r) {
            out += get_priority(*v)?;
        }
    }

    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let data: Vec<Vec<char>> =
        parse_input(LineSplitter, Chars(Identity), input).context("failed to parse input")?;

    let mut out = 0;
    for chunk in data.chunks(3) {
        assert!(chunk.len() == 3);

        let a: BTreeSet<&char> = chunk[0].iter().collect();
        let b: BTreeSet<&char> = chunk[1].iter().collect();
        let c: BTreeSet<&char> = chunk[2].iter().collect();

        let ab: BTreeSet<&char> = a.intersection(&b).cloned().collect();
        let abc: Vec<&char> = ab.intersection(&c).cloned().collect();
        assert!(abc.len() == 1);
        out += get_priority(**abc.first().context("failed to find badge!")?)
            .context("failed to get priority")?;
    }
    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_3 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(157);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(70);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
