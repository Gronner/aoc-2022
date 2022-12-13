use std::{cmp::Ordering, num::ParseIntError};


use aoc_downloader::download_day;

const DAY: u32 = 13;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Side;
type Output = usize;

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .filter(|&line| !line.is_empty())
        .map(|line| {
            let (side, _) = Side::from_str_fixed(line).unwrap();
            side
        })
        .collect::<Vec<_>>()
}

#[allow(clippy::derive_ord_xor_partial_ord)]
#[derive(Debug, Ord, Eq, Clone)]
enum Side {
    Integer(u64),
    List(Vec<Side>),
}

impl Side {
    fn from_str_fixed(s: &str) -> Result<(Self, usize), ParseIntError> {
        let mut stack = String::new();
        let mut list = vec![];
        let mut advance = 0;
        for i in 0..s.len() {
            if advance != 0 {
                advance -= 1;
                continue;
            }
            let c = s.chars().nth(i).unwrap();
            if c == '[' {
                let (new_side, new_i) = Side::from_str_fixed(&s[i+1..]).unwrap();
                list.push(new_side);
                advance = new_i + 1;
                continue;
            } else if c.is_ascii_digit() {
                stack.push(c);
                continue;
            } else if c == ',' && !stack.is_empty() {
                list.push(Side::Integer(stack.parse::<u64>().unwrap()));
                stack.clear();
            } else if c == ',' {
                continue;
            } else if c == ']' {
                if !stack.is_empty() {
                    list.push(Side::Integer(stack.parse::<u64>().unwrap()));
                }
                return Ok((Side::List(list), i));
            } else {
                panic!("Unknown char: {c}");
            }
        }
        Ok((Side::List(list), 0))
    }
}

impl std::cmp::PartialEq for Side {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl std::cmp::PartialOrd for Side {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match(self, other) {
            (Side::Integer(left), Side::Integer(right)) => left.partial_cmp(right),
            (Side::List(_), Side::Integer(_)) => self.partial_cmp(&Side::List(vec![other.clone()])),
            (Side::Integer(_), Side::List(_)) => Side::List(vec![self.clone()]).partial_cmp(other),
            (Side::List(left), Side::List(right)) => {
                for (list_left, list_right) in left.iter().zip(right) {
                    if let Some(result) = list_left.partial_cmp(list_right) {
                        if result != Ordering::Equal {
                            return Some(result);
                        }
                    }
                }
                left.len().partial_cmp(&right.len())
            },
        }
    }
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &[Input]) -> Output {
    let mut index_sum = 0;
    for (i, pair) in input.chunks(2).enumerate() {
        if pair[0] < pair[1] {
            index_sum += i + 1;
        }
    }
    index_sum
}

fn part2(input: &[Input]) -> Output {
    let mut input = input.to_owned();
    let div_package_1 = Side::List(vec![Side::List(vec![Side::Integer(2)])]);
    let div_package_2 = Side::List(vec![Side::List(vec![Side::Integer(6)])]);
    input.push(div_package_1.clone());
    input.push(div_package_2.clone());
    input.sort();
    let div1_idx = input.iter().position(|side| side == &div_package_1).unwrap() + 1;
    let div2_idx = input.iter().position(|side| side == &div_package_2).unwrap() + 1;
    div1_idx * div2_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(5852, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(24190, part2(&input));
    }
}
