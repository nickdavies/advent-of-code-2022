advent_of_code::solution!(5);

use advent_of_code::template::RunType;

use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
struct Command {
    source: usize,
    dest: usize,
    count: usize,
}

fn parse(grid_str: &str, commands_str: &str) -> Result<(Vec<Vec<char>>, Vec<Command>)> {
    let mut stacks: Vec<Vec<char>> = (0..10).map(|_| Vec::new()).collect();
    for line in grid_str.lines().rev().skip(1) {
        for (idx, char) in line.char_indices() {
            match char {
                '[' | ']' | ' ' => continue,
                'A'..='Z' => stacks[idx / 4].push(char),
                '1'..='9' => break,
                other => {
                    return Err(anyhow!("got unexpected value '{}' in stack", other));
                }
            }
        }
    }

    let mut commands = Vec::new();
    for line in commands_str.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() != 6 {
            return Err(anyhow!("got unexpected line '{}' in commands", line));
        }

        let count = parts[1].parse().context("failed to parse count")?;
        let source: usize = parts[3].parse().context("failed to parse source")?;
        let dest: usize = parts[5].parse().context("failed to parse dest")?;

        // We count from 0 in these parts
        commands.push(Command {
            count,
            source: source - 1,
            dest: dest - 1,
        });
    }

    Ok((stacks, commands))
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<String>, anyhow::Error> {
    let (grid_str, commands_str) = input
        .split_once("\n\n")
        .context("failled to split sections")?;

    let (mut grid, commands) = parse(grid_str, commands_str).context("failed to parse input")?;
    for command in commands {
        for _ in 0..command.count {
            let tmp = grid[command.source]
                .pop()
                .context("Ran out of objects following command")?;
            grid[command.dest].push(tmp);
        }
    }

    let mut out = String::new();
    for col in grid {
        if let Some(c) = col.last() {
            out.push(*c);
        }
    }
    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<String>, anyhow::Error> {
    let (grid_str, commands_str) = input
        .split_once("\n\n")
        .context("failled to split sections")?;

    let (mut grid, commands) = parse(grid_str, commands_str).context("failed to parse input")?;
    for command in commands {
        let mut tmp = Vec::new();
        for _ in 0..command.count {
            tmp.push(
                grid[command.source]
                    .pop()
                    .context("Ran out of objects following command")?,
            );
        }
        grid[command.dest].extend(tmp.iter().rev());
    }

    let mut out = String::new();
    for col in grid {
        if let Some(c) = col.last() {
            out.push(*c);
        }
    }
    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_5 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some("CMZ".to_string());
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some("MCD".to_string());
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
