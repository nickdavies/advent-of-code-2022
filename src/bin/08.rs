advent_of_code::solution!(8);

use advent_of_code::template::RunType;

use anyhow::{Context, Result};
use aoc_lib::grid::{CountingMap, Direction, Map};

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let map: Map<u32> = Map::parse(input, |c| c.to_digit(10).context("failed to parse digit"))
        .context("failed to parse input")?;

    let mut visible: CountingMap = (&map).into();
    for (mut current, direction) in map.get_edges() {
        visible.mark(&current);
        let mut max = map.get(&current);
        loop {
            current = match map.go_direction(&current, &direction) {
                Some(next) => next,
                None => break,
            };
            let height = map.get(&current);
            if height > max {
                visible.mark(&current);
            }

            max = std::cmp::max(max, height);
            if *max == 9 {
                break;
            }
        }
    }
    Ok(Some(visible.unique()))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let map: Map<u32> = Map::parse(input, |c| c.to_digit(10).context("failed to parse digit"))
        .context("failed to parse input")?;

    let mut best = None;
    for row in map.iter() {
        for (loc, current_height) in row {
            let mut senic = 1;
            for direction in Direction::all() {
                let mut dist = 0;
                let mut current = loc.clone();
                while let Some(next) = map.go_direction(&current, direction) {
                    let next_height = map.get(&next);
                    dist += 1;
                    if next_height >= current_height {
                        break;
                    }
                    current = next;
                }
                senic *= dist
            }
            match best {
                Some(current) => {
                    if senic > current {
                        best = Some(senic);
                    }
                }
                None => {
                    best = Some(senic);
                }
            }
        }
    }

    Ok(best)
}

#[cfg(test)]
mod tests_day_8 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(21);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(8);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
