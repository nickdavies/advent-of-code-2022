advent_of_code::solution!(2);

use advent_of_code::template::RunType;

use anyhow::{anyhow, Context};
use aoc_lib::parse::preamble::*;

#[derive(Debug)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl TryFrom<char> for Outcome {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            other => {
                return Err(anyhow!("Unexpected char: {}", other));
            }
        })
    }
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

#[derive(Debug)]
enum Turn {
    Rock,
    Paper,
    Scissor,
}

impl Turn {
    fn vs(&self, opponent: &Turn) -> Outcome {
        match (self, opponent) {
            (Self::Rock, Self::Rock) => Outcome::Draw,
            (Self::Rock, Self::Paper) => Outcome::Loss,
            (Self::Rock, Self::Scissor) => Outcome::Win,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Paper) => Outcome::Draw,
            (Self::Paper, Self::Scissor) => Outcome::Loss,
            (Self::Scissor, Self::Rock) => Outcome::Loss,
            (Self::Scissor, Self::Paper) => Outcome::Win,
            (Self::Scissor, Self::Scissor) => Outcome::Draw,
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    fn force_outcome(&self, outcome: &Outcome) -> Self {
        match (self, outcome) {
            (Self::Rock, Outcome::Draw) => Self::Rock,
            (Self::Rock, Outcome::Win) => Self::Paper,
            (Self::Rock, Outcome::Loss) => Self::Scissor,
            (Self::Paper, Outcome::Loss) => Self::Rock,
            (Self::Paper, Outcome::Draw) => Self::Paper,
            (Self::Paper, Outcome::Win) => Self::Scissor,
            (Self::Scissor, Outcome::Win) => Self::Rock,
            (Self::Scissor, Outcome::Loss) => Self::Paper,
            (Self::Scissor, Outcome::Draw) => Self::Scissor,
        }
    }
}

impl TryFrom<char> for Turn {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissor,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissor,
            other => {
                return Err(anyhow!("Unexpected char: {}", other));
            }
        })
    }
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let data: Vec<(Turn, Turn)> = parse_input(
        LineSplitter,
        ParseTuple2(SingleChar(TryFromChar), SingleChar(TryFromChar), " "),
        input,
    )
    .context("Failed to parse input")?;

    let mut out = 0;
    for (op, me) in data {
        let outcome = me.vs(&op);
        out += me.score() + outcome.score();
    }

    Ok(Some(out))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<u32>, anyhow::Error> {
    let data: Vec<(Turn, Outcome)> = parse_input(
        LineSplitter,
        ParseTuple2(SingleChar(TryFromChar), SingleChar(TryFromChar), " "),
        input,
    )
    .context("Failed to parse input")?;
    let mut out = 0;
    for (op, outcome) in data {
        let me = &op.force_outcome(&outcome);
        out += me.score() + outcome.score();
    }

    Ok(Some(out))
}

#[cfg(test)]
mod tests_day_2 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(15);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(
            expected.is_none() || !input.is_empty(),
            "example 1 is empty!"
        );
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(12);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(
            expected.is_none() || !input.is_empty(),
            "example 2 is empty!"
        );
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
