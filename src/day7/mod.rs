use std::{str::FromStr, num::ParseIntError};
use std::collections::HashMap;
use std::path::PathBuf;

use aoc_downloader::download_day;

const DAY: u32 = 7;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

#[derive(Debug)]
enum Terminal {
    Cd(String),
    Ls,
    Dir(String),
    File(String, u32),
}

impl FromStr for Terminal {
    type Err = ParseIntError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ls_re = regex!(r"\$ ls");
        let cd_re = regex!(r"\$ cd (.*)");
        let dir_re = regex!(r"dir (\w+)");
        let file_re = regex!(r"(\d+) ([\w.]+)");

        if ls_re.is_match(s) {
            Ok(Terminal::Ls)
        } else if let Some(cd) = cd_re.captures(s).map(|captured| {
            Terminal::Cd(String::from(&captured[1]))
        }) {
            Ok(cd)
        } else if let Some(dir) = dir_re.captures(s).map(|captured| {
            Terminal::Dir(String::from(&captured[1]))
        }) {
            Ok(dir)
        } else if let Some(file) = file_re.captures(s).map(|captured| {
            Terminal::File(
                    String::from(&captured[2]),
                    captured[1].parse::<u32>().unwrap(),
            )
        }) {
            Ok(file)
        } else {
            panic!();
        }
    }
}

type Input = Terminal;

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|v| Terminal::from_str(v).unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn get_dir_paths(input: &Vec<Input>) -> HashMap<PathBuf, u32> {
    let mut dir_sizes: HashMap<PathBuf, u32> = HashMap::new();
    let mut cur_path = PathBuf::new();
    for line in input {
        match line {
            Terminal::Cd(dir) if dir == ".." => cur_path = cur_path.parent().unwrap().to_owned(),
            Terminal::Cd(dir) => {
                cur_path = cur_path.join(dir);
                dir_sizes.entry(cur_path.clone()).or_insert(0);
            },
            Terminal::Ls => (),
            Terminal::Dir(dir) => {
                dir_sizes.entry(cur_path.join(dir)).or_insert(0);
            },
            Terminal::File(_, size) => {
                dir_sizes.entry(cur_path.clone())
                    .and_modify(|dsize| *dsize += size)
                    .or_insert(*size);
            },
        }
    }
    dir_sizes
}

fn size_dirs(dirs: HashMap<PathBuf, u32>) -> HashMap<PathBuf, u32> {
    let mut dirs = dirs;
    let fix_dir = dirs.clone();
    for dir in &mut dirs {
        for other_dir in &fix_dir {
            if other_dir.0 == dir.0 {
                continue;
            }
            if other_dir.0.as_path().starts_with(dir.0.as_path()) {
                *dir.1 += other_dir.1;
            }
        }
    }
    dirs
}

fn part1(input: &Vec<Input>) -> u32 {
    let dir_sizes = get_dir_paths(input);
    let dir_sizes = size_dirs(dir_sizes);

    dir_sizes.values()
        .filter(|size| **size <= 100_000)
        .sum()
}

fn part2(input: &Vec<Input>) -> u32 {
    const AVAILABLE: u32 = 70_000_000;
    const REQUIRED: u32 = 30_000_000;

    let dir_sizes = get_dir_paths(input);
    let dir_sizes = size_dirs(dir_sizes);

    let total_size = dir_sizes.get(&PathBuf::from("/")).unwrap().to_owned();
    let unused = AVAILABLE - total_size;

    *dir_sizes.values()
        .filter(|size| (unused + **size) >= REQUIRED)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(1844187, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(4978279, part2(&input));
    }
}
