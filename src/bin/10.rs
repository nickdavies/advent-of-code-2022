#![allow(unused_imports)]
advent_of_code::solution!(10);

use advent_of_code::template::RunType;
use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone)]
enum Command {
    Noop,
    Addx(i32),
}

fn parse(input: &str) -> Result<Vec<Command>> {
    let mut out = Vec::new();
    for line in input.lines() {
        if line == "noop" {
            out.push(Command::Noop);
            continue;
        } else {
            let (cmd, x) = line
                .split_once(" ")
                .context("expected to find 2 part cmd if not noop")?;
            if cmd == "addx" {
                out.push(Command::Addx(x.parse().context("failed to parse X")?));
            } else {
                return Err(anyhow!("Unknown command {:?}", line));
            }
        }
    }

    Ok(out)
}

fn is_special(cycle: u32) -> bool {
    if cycle < 20 {
        false
    } else {
        (cycle - 20) % 40 == 0
    }
}

fn run_program<F: FnMut(u32, i64)>(commands: &[Command], callback: &mut F) {
    let mut cycle: u32 = 1;
    let mut register: i64 = 1;

    for command in commands {
        match command {
            Command::Noop => {
                callback(cycle, register);
                cycle += 1;
            }
            Command::Addx(x) => {
                callback(cycle, register);
                cycle += 1;
                callback(cycle, register);
                cycle += 1;
                register += *x as i64;
            }
        }
    }
}
pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<i64>, anyhow::Error> {
    let commands = parse(input).context("Failed to parse input")?;
    let mut signal_strength = 0;

    run_program(&commands, &mut |cycle, register| {
        if is_special(cycle) {
            signal_strength += register * cycle as i64;
        }
    });

    Ok(Some(signal_strength))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<String>, anyhow::Error> {
    let commands = parse(input).context("Failed to parse input")?;

    let mut out = String::new();
    let mut current = String::new();
    run_program(&commands, &mut |cycle, register| {
        let cycle = (cycle - 1) % 40;
        if cycle as i64 == register - 1 || cycle as i64 == register || cycle as i64 == register + 1
        {
            current.push('#');
        } else {
            current.push('.');
        }
        if cycle == 39 {
            out.push_str(&current);
            out.push('\n');
            current = String::new();
        }
    });
    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_10 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(13140);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            .to_string(),
        );
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
