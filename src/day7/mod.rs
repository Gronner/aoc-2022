use std::{str::FromStr, string::ParseError};
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
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ls_re = regex!(r"\$ ls");
        let cd_re = regex!(r"\$ cd (.*)");
        let dir_re = regex!(r"dir (\w+)");
        let file_re = regex!(r"(\d+) ([\w.]+)");

        if ls_re.is_match(s) {
            return Ok(Terminal::Ls);
        }
        if let Some(cd) = cd_re.captures(s).and_then(|captured| {
            Some(Terminal::Cd(String::from(&captured[1])))
        }) {
            return Ok(cd);
        }
        if let Some(dir) = dir_re.captures(s).and_then(|captured| {
            Some(Terminal::Dir(String::from(&captured[1])))
        }) {
            return Ok(dir);
        }
        if let Some(file) = file_re.captures(s).and_then(|captured| {
            Some(Terminal::File(
                    String::from(&captured[2]),
                    captured[1].parse::<u32>().unwrap()
            ))

        }) {
            return Ok(file);
        }
        panic!();
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

fn part1(input: &Vec<Input>) -> u32 {
    let mut dir_sizes: HashMap<PathBuf, u32> = HashMap::new();
    // let mut sub_dirs: HashMap<&Path, Vec<&Path>> = HashMap::new();
    let mut cur_path = PathBuf::new();
    for line in input {
        match line {
            Terminal::Cd(dir) => {
                if dir != ".." {
                    cur_path = cur_path.join(&dir);
                    dir_sizes.entry(cur_path.clone()).or_insert(0);
                } else {
                    cur_path = cur_path.parent().unwrap().to_owned();
                }
            },
            Terminal::Ls => (),
            Terminal::Dir(dir) => {
                dir_sizes.entry(cur_path.join(dir)).or_insert(0);
            },
            Terminal::File(file, size) => {
                dir_sizes.entry(cur_path.clone())
                    .and_modify(|dsize| *dsize += size)
                    .or_insert(*size);
            },
        }
    }
    let mut total_size = 0;
    for dir in &dir_sizes {
        println!("{:?}", dir);
        let mut dir_size = *dir.1;
        for other_dir in &dir_sizes {
            if other_dir == dir {
                continue;
            }
            if other_dir.0.as_path().starts_with(dir.0.as_path()) {
                dir_size += other_dir.1;
            }
        }
        println!("{:?}", dir_size);
        if dir_size <= 100_000 {
            total_size += dir_size;
        }
    }
    total_size
}

fn part2(input: &Vec<Input>) -> u32 {
    let mut dir_sizes: HashMap<PathBuf, u32> = HashMap::new();
    // let mut sub_dirs: HashMap<&Path, Vec<&Path>> = HashMap::new();
    let mut cur_path = PathBuf::new();
    for line in input {
        match line {
            Terminal::Cd(dir) => {
                if dir != ".." {
                    cur_path = cur_path.join(&dir);
                    dir_sizes.entry(cur_path.clone()).or_insert(0);
                } else {
                    cur_path = cur_path.parent().unwrap().to_owned();
                }
            },
            Terminal::Ls => (),
            Terminal::Dir(dir) => {
                dir_sizes.entry(cur_path.join(dir)).or_insert(0);
            },
            Terminal::File(file, size) => {
                dir_sizes.entry(cur_path.clone())
                    .and_modify(|dsize| *dsize += size)
                    .or_insert(*size);
            },
        }
    }

    let tmp_dir_sizes = dir_sizes.clone();
    for dir in &mut dir_sizes {
        for other_dir in &tmp_dir_sizes {
            if other_dir.0 == dir.0 {
                continue;
            }
            if other_dir.0.as_path().starts_with(dir.0.as_path()) {
                *dir.1 += other_dir.1;
            }
        }
    }

    let total_size = dir_sizes.get(&PathBuf::from("/")).unwrap().to_owned();
    let unused = 70_000_000 - total_size;

    let mut minimal_size = u32::MAX;
    for dir in dir_sizes {
        if (unused + dir.1) >= 30_000_000 {
            if minimal_size >= dir.1 {
                minimal_size = dir.1;
            }            
        }
    }

    minimal_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(744475, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(70276940, part2(&input));
    }
}
