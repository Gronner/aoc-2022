use std::{str::FromStr, num::ParseIntError, collections::{HashMap, HashSet}};

use aoc_downloader::download_day;

const DAY: u32 = 9;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Command;
type Output = usize;

enum Command {
    Right(usize),
    Left(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"(\w) (\d+)");

        Ok(re.captures(s).map(|captured| {
            let step = captured[2].parse::<usize>().unwrap();
            match &captured[1] {
                "R" => Command::Right(step),
                "L" => Command::Left(step),
                "D" => Command::Down(step),
                "U" => Command::Up(step),
                e => panic!("Unkown input: {}", e),
            }
        }).unwrap())
    }
}

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|v| Command::from_str(v).unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn update_tail(head: &(isize, isize), tail: &(isize, isize)) -> Option<(isize, isize)> {
    match (head.0 - tail.0, head.1 - tail.1) {
        (0, 0) | (1,0) | (-1,0) | (1, 1) | (1, -1) | (-1, 1) | (-1, -1) | (0, 1) | (0, -1) => {
            None
        },
        (x, y) => {
            Some((tail.0 + x.signum(), tail.1 + y.signum()))
        },
    }
}

fn part1(input: &[Input]) -> Output {
    let mut visited = HashSet::new();
    let mut current_head: (isize, isize) = (0, 0);
    let mut current_tail: (isize, isize) = (0, 0);
    visited.insert(current_tail);
    for command in input {
       match command {
            Command::Right(s) => {
                for _ in 0..*s {
                    current_head = (current_head.0 + 1, current_head.1);
                    if let Some(new_tail) = update_tail(&current_head, &current_tail) {
                        visited.insert(new_tail);
                        current_tail = new_tail;
                    }
                }
            },
            Command::Left(s) => {
                for _ in 0..*s {
                    current_head = (current_head.0 - 1, current_head.1);
                    if let Some(new_tail) = update_tail(&current_head, &current_tail) {
                        visited.insert(new_tail);
                        current_tail = new_tail;
                    }
                }
            },
            Command::Down(s) => {
                for _ in 0..*s {
                    current_head = (current_head.0, current_head.1 - 1);
                    if let Some(new_tail) = update_tail(&current_head, &current_tail) {
                        visited.insert(new_tail);
                        current_tail = new_tail;
                    }
                }
            },
            Command::Up(s) => {
                for _ in 0..*s {
                    current_head = (current_head.0, current_head.1 + 1);
                    if let Some(new_tail) = update_tail(&current_head, &current_tail) {
                        visited.insert(new_tail);
                        current_tail = new_tail;
                    }
                }
            },
       }
    }
    visited.len()
}

fn part2(input: &[Input]) -> Output {
    const ROPE_LENGTH: usize = 10;
    let mut visited = HashSet::new();
    let mut rope: Vec<(isize, isize)> = vec![(0, 0); ROPE_LENGTH];
    visited.insert(rope[ROPE_LENGTH - 1]);
    for command in input {
       match command {
            Command::Right(s) => {
                for _ in 0..*s {
                    rope[0] = (rope[0].0 + 1, rope[0].1);
                    for i in 1..ROPE_LENGTH {
                        if let Some(new_rope_pos) = update_tail(&rope[i-1], &rope[i]) {
                            rope[i] = new_rope_pos;
                        }
                    }
                    visited.insert(rope[9]);
                }
            },
            Command::Left(s) => {
                for _ in 0..*s {
                    rope[0] = (rope[0].0 - 1, rope[0].1);
                    for i in 1..ROPE_LENGTH {
                        if let Some(new_rope_pos) = update_tail(&rope[i-1], &rope[i]) {
                            rope[i] = new_rope_pos;
                        }
                    }
                    visited.insert(rope[9]);
                }
            },
            Command::Down(s) => {
                for _ in 0..*s {
                    rope[0] = (rope[0].0, rope[0].1 - 1);
                    for i in 1..ROPE_LENGTH {
                        if let Some(new_rope_pos) = update_tail(&rope[i-1], &rope[i]) {
                            rope[i] = new_rope_pos;
                        }
                    }
                    visited.insert(rope[9]);
                }
            },
            Command::Up(s) => {
                for _ in 0..*s {
                    rope[0] = (rope[0].0, rope[0].1 + 1);
                    for i in 1..ROPE_LENGTH {
                        if let Some(new_rope_pos) = update_tail(&rope[i-1], &rope[i]) {
                            rope[i] = new_rope_pos;
                        }
                    }
                    visited.insert(rope[9]);
                }
            },
       }
    }
    visited.len()
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
