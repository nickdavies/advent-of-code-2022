advent_of_code::solution!(6);

use advent_of_code::template::RunType;
use std::collections::VecDeque;

use anyhow::Result;

struct Stream<'a> {
    source: std::str::CharIndices<'a>,
    buffer: VecDeque<char>,
    len: usize,
}

impl<'a> Stream<'a> {
    fn new(source: &'a str, len: usize) -> Self {
        Stream {
            source: source.char_indices(),
            buffer: VecDeque::new(),
            len,
        }
    }
}

#[derive(Debug)]
struct StreamElement {
    idx: usize,
    ejected: Option<char>,
    added: char,
}

impl Iterator for Stream<'_> {
    type Item = StreamElement;

    fn next(&mut self) -> Option<Self::Item> {
        let new = self.source.next()?;
        self.buffer.push_back(new.1);

        let ejected = if self.buffer.len() > self.len {
            Some(self.buffer.pop_front().expect("I just added something :S"))
        } else {
            None
        };

        Some(StreamElement {
            idx: new.0,
            ejected,
            added: new.1,
        })
    }
}

fn run(input: &str, len: usize) -> Result<Option<usize>> {
    let mut hist = [0; 26];
    let mut count = 0;

    for element in Stream::new(input.trim(), len) {
        let char_idx = element.added as usize - 'a' as usize;
        if hist[char_idx] == 1 {
            count += 1;
        }
        hist[char_idx] += 1;

        if let Some(removed) = element.ejected {
            let char_idx = removed as usize - 'a' as usize;
            hist[char_idx] -= 1;
            if hist[char_idx] == 1 {
                count -= 1;
            }
        }

        if count == 0 && element.idx >= len {
            return Ok(Some(element.idx + 1));
        }
    }

    Ok(None)
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    run(input, 4)
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    run(input, 14)
}

#[cfg(test)]
mod tests_day_6 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(7);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(19);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
