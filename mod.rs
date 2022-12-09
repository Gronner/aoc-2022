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

impl Command {
    pub fn get_inner(&self) -> usize {
        match self {
            Command::Right(inner) => *inner,
            Command::Left(inner) => *inner,
            Command::Down(inner) => *inner,
            Command::Up(inner) => *inner,
        }
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

fn add(lhs: isize, rhs: isize) -> isize {
    lhs + rhs
}

fn sub(lhs: isize, rhs: isize) -> isize {
    lhs - rhs
}

fn mul(lhs: isize, rhs: isize) -> isize {
    lhs * rhs
}

fn pull_rope(rope: &mut Vec<(isize, isize)>, direction: (fn(isize, isize) -> isize, fn(isize, isize) -> isize)) {
    rope[0] = (direction.0(rope[0].0, 1), direction.1(rope[0].1, 1));
    for i in 1..rope.len(){
        if let Some(new_rope_pos) = update_tail(&rope[i-1], &rope[i]) {
            rope[i] = new_rope_pos;
        }
    }
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

fn follow_n_rope(input: &[Input], length: usize) -> usize {
    let mut visited = HashSet::new();
    let mut rope: Vec<(isize, isize)> = vec![(0, 0); length];
    visited.insert(rope[length - 1]);
    for command in input {
        let ops: (fn(isize, isize) -> isize, fn(isize, isize) -> isize)  = match command {
            Command::Right(_) => (add, mul),
            Command::Left(_) => (sub, mul),
            Command::Down(_) => (mul, sub),
            Command::Up(_) => (mul, add),
        };
        let step = command.get_inner();
        for _ in 0..step {
            pull_rope(&mut rope, ops);
            visited.insert(rope[length - 1]);
        }
    }
    visited.len()
}

fn part1(input: &[Input]) -> Output {
    follow_n_rope(input, 2)
}

fn part2(input: &[Input]) -> Output {
    follow_n_rope(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(6314, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(2504, part2(&input));
    }
}
