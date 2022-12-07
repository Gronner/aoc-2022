use std::{str::FromStr, string::ParseError};

use aoc_downloader::download_day;

const DAY: u32 = 5;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

pub struct Command {
    pub amount: usize,
    pub source: usize,
    pub target: usize,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"move (\d+) from (\d+) to (\d+)");
        Ok(re
            .captures(s)
            .and_then(|captured| {
                Some(Command {
                    amount: captured[1].parse::<usize>().ok()?,
                    source: captured[2].parse::<usize>().ok()?,
                    target: captured[3].parse::<usize>().ok()?,
                })
            })
            .unwrap())
    }
}

fn parse_input(input: Vec<String>) -> (Vec<Vec<char>>, Vec<Command>) {
    let mut switch = true;
    let mut commands = Vec::new();
    let mut stack_line = 0;
    for (idx, line) in input.iter().enumerate() {
        if switch {
            if line.is_empty() {
                stack_line = idx - 1;
                switch = false;
                continue;
            }
        } else {
            commands.push(Command::from_str(line).unwrap());
        }
    }

    let stack_count = input[stack_line]
        .chars()
        .filter(|c| !c.is_whitespace())
        .last()
        .map(|value| value.to_digit(10).unwrap() as usize)
        .unwrap();

    let mut stacks = vec![Vec::new(); stack_count];
    const OFFSET: usize = 1;

    for line in input[0..stack_line].iter().rev() {
        for (stack_id, stack) in stacks.iter_mut().enumerate().take(stack_count) {
            if let Some(chest) = line.chars().nth(OFFSET + stack_id * 4) {
                if chest == ' ' {
                    continue;
                }
                stack.push(chest);
            }
        }
    }

    (stacks, commands)
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!(
        "Running day {}:\n\tPart1 {}\n\tPart2 {}",
        DAY,
        part1(&input),
        part2(&input)
    );
}

fn move_box(com: &Command, stacks: &mut [Vec<char>]) {
    for _ in 0..com.amount {
        let tmp = stacks[com.source - 1].pop().unwrap();
        stacks[com.target - 1].push(tmp);
    }
}

fn part1(input: &(Vec<Vec<char>>, Vec<Command>)) -> String {
    let (stacks, commands) = input;
    let mut stacks = stacks.clone();

    for com in commands {
        move_box(com, &mut stacks);
    }

    let mut output = String::new();
    for stack in stacks.iter_mut() {
        output.push(stack.pop().unwrap());
    }
    output
}

fn move_all_boxes(com: &Command, stacks: &mut [Vec<char>]) {
    let mut tmp = vec![];
    for _ in 0..com.amount {
        tmp.push(stacks[com.source - 1].pop().unwrap());
    }
    tmp.reverse();
    stacks[com.target - 1].append(&mut tmp);
}

fn part2(input: &(Vec<Vec<char>>, Vec<Command>)) -> String {
    let (stacks, commands) = input;
    let mut stacks = stacks.clone();

    for com in commands {
        move_all_boxes(com, &mut stacks);
    }

    let mut output = String::new();
    for stack in stacks.iter_mut() {
        output.push(stack.pop().unwrap());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!("FJSRQCFTN", part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!("CJVLJQPHS", part2(&input));
    }
}
