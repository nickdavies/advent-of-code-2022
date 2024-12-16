advent_of_code::solution!(7);

use advent_of_code::template::RunType;

use std::collections::{BTreeMap, BTreeSet};
use std::iter::Peekable;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
enum LsEntry {
    Dir,
    File(usize),
}

#[derive(Debug)]
enum Command {
    Cd(PathBuf),
    Ls(Vec<(PathBuf, LsEntry)>),
}

#[derive(Debug)]
struct Directory {
    dirs: BTreeSet<PathBuf>,
    files: BTreeMap<PathBuf, usize>,
}

impl Directory {
    fn new() -> Self {
        Self {
            dirs: BTreeSet::new(),
            files: BTreeMap::new(),
        }
    }
}

fn calc_size(path: &Path, dirs: &BTreeMap<PathBuf, Directory>) -> usize {
    let dir = dirs.get(path).unwrap();
    let mut total_size = 0;
    for size in dir.files.values() {
        total_size += size;
    }
    for subpath in &dir.dirs {
        total_size += calc_size(subpath, dirs);
    }
    total_size
}

impl<'a, I> TryFrom<&mut Peekable<I>> for Command
where
    I: Iterator<Item = &'a str>,
{
    type Error = anyhow::Error;

    fn try_from(other: &mut Peekable<I>) -> Result<Self> {
        let cmd = other
            .next()
            .context("Expected to find command!")?
            .strip_prefix("$ ")
            .context("Got unexpected non-command line in input")?;

        Ok(if cmd.starts_with("cd") {
            let (_, target) = cmd
                .split_once(" ")
                .context("expected to find dir after cd")?;

            Self::Cd(target.into())
        } else if cmd.starts_with("ls") {
            let mut entries = Vec::new();
            loop {
                match other.peek() {
                    None => break,
                    Some(line) => {
                        if line.starts_with("$") {
                            break;
                        }
                    }
                }
                let line = other.next().unwrap();
                let (first, second) = line
                    .trim()
                    .split_once(" ")
                    .context("Expected to find `<x> <name>` in ls entry")?;

                if first == "dir" {
                    entries.push((second.into(), LsEntry::Dir));
                } else {
                    entries.push((
                        second.into(),
                        LsEntry::File(first.parse().context("Invalid file size found")?),
                    ));
                }
            }
            Self::Ls(entries)
        } else {
            return Err(anyhow!("Invalid command {:?} found", cmd));
        })
    }
}

fn build_dirs(input: &str) -> Result<BTreeMap<PathBuf, Directory>> {
    let mut lines = input.lines().peekable();
    let mut commands = Vec::new();
    while lines.peek().is_some() {
        commands.push(Command::try_from(&mut lines).context("failed to parse command")?);
    }

    let mut cwd: PathBuf = "/".into();
    let mut dirs: BTreeMap<PathBuf, Directory> = BTreeMap::new();
    for command in &commands {
        match command {
            Command::Cd(path) => {
                if path == Path::new("/") {
                    cwd = "/".into();
                } else if path == Path::new("..") {
                    cwd.pop();
                } else {
                    cwd = cwd.join(path);
                }
            }
            Command::Ls(entries) => {
                let dir = dirs.entry(cwd.clone()).or_insert_with(Directory::new);
                for entry in entries {
                    let full_path = cwd.join(&entry.0);
                    match entry.1 {
                        LsEntry::Dir => {
                            dir.dirs.insert(full_path);
                        }
                        LsEntry::File(size) => {
                            dir.files.insert(full_path, size);
                        }
                    }
                }
            }
        }
    }
    Ok(dirs)
}

pub fn part_one(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let dirs = build_dirs(input)?;
    let mut total = 0;
    for dir in dirs.keys() {
        let size = calc_size(dir, &dirs);
        if size <= 100000 {
            total += size;
        }
    }
    Ok(Some(total))
}

pub fn part_two(input: &str, _run_type: RunType) -> Result<Option<usize>, anyhow::Error> {
    let dirs = build_dirs(input)?;
    let available = 70000000;
    let in_use = calc_size(Path::new("/"), &dirs);
    let free = available - in_use;
    let update_size = 30000000;
    let min_to_free = update_size - free;

    let mut dir_sizes = Vec::new();
    for dir in dirs.keys() {
        let size = calc_size(dir, &dirs);
        if size < min_to_free {
            continue;
        }
        dir_sizes.push((size, dir.clone()));
    }
    dir_sizes.sort();

    Ok(dir_sizes.into_iter().next().map(|(s, _)| s))
}

#[cfg(test)]
mod tests_day_7 {
    use super::*;

    #[test]
    fn test_part_one() -> anyhow::Result<()> {
        let expected = Some(95437);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 1);
        assert!(expected.is_none() || !input.is_empty(), "example 1 empty!");
        let result = part_one(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_part_two() -> anyhow::Result<()> {
        let expected = Some(24933642);
        let input = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert!(expected.is_none() || !input.is_empty(), "example 2 empty!");
        let result = part_two(input, RunType::Example)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
