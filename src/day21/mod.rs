use std::{str::FromStr, num::ParseIntError};
use std::collections::HashMap;

use aoc_downloader::download_day;

const DAY: u32 = 21;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = HashMap<String, Monkey>;
type Output = i64;

#[derive(Clone, Debug)]
enum Monkey {
    Job(String, String, char),
    Num(i64)
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_mon_re = regex!(r"(\d+)");
        let job_mon_re = regex!(r"(\w+) ([\+\-*/=]) (\w+)");
        if let Some(monkey) = num_mon_re.captures(s).map(|captured| {
            Monkey::Num(captured[1].parse::<i64>().unwrap())
        }) {
            return Ok(monkey);
        }
        if let Some(monkey) = job_mon_re.captures(s).map(|captured| {
            Monkey::Job(
                String::from(&captured[1]),
                String::from(&captured[3]),
                captured[2].chars().nth(0).unwrap(),
            )
        }) {
            return Ok(monkey);
        }
        unreachable!();
    }
}

fn parse_input(input: Vec<String>) -> Input {
    input
        .iter()
        .map(|line| {
            if let Some((name, operation)) = line.split_once(": ") {
                (String::from(name), Monkey::from_str(operation).unwrap())
            } else {
                unreachable!()
            }
        })
        .collect::<HashMap<_, _>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn tree_walk(monkey_tree: &HashMap<String, Monkey>, node: &str) -> i64 {
    if let Some(Monkey::Num(n)) = monkey_tree.get(node) {
        return *n;
    }

    if let Some(Monkey::Job(left, right, op)) = monkey_tree.get(node) {
        return match op {
            '+' => tree_walk(monkey_tree, left) + tree_walk(monkey_tree, right),
            '-' => tree_walk(monkey_tree, left) - tree_walk(monkey_tree, right),
            '*' => tree_walk(monkey_tree, left) * tree_walk(monkey_tree, right),
            '/' => tree_walk(monkey_tree, left) / tree_walk(monkey_tree, right),
            '=' => {
                if tree_walk(monkey_tree, left) == tree_walk(monkey_tree, right) {
                    0
                } else {
                    1
                }
            },
            _ => unreachable!("Unkown symbol {op}"),
        }
    }

    unreachable!();
}

fn equation_generator(monkey_tree: &HashMap<String, Monkey>, node: &str) -> String {
    if node == "humn" {
        return String::from("x");
    }
    if let Some(Monkey::Num(n)) = monkey_tree.get(node) {
        return n.to_string();
    }

    if let Some(Monkey::Job(left, right, op)) = monkey_tree.get(node) {
        return vec!["(".to_string(), equation_generator(monkey_tree, left), op.to_string(), equation_generator(monkey_tree, right), ")".to_string()].connect(" ");
    }

    unreachable!();
}

fn part1(input: &HashMap<String, Monkey>) -> Output {
    let start = String::from("root");
    tree_walk(&input, &start)
}

fn part2(input: &HashMap<String, Monkey>) -> i64{
    let start = String::from("root");
    let mut input = input.clone();
    input.entry("root".to_string()).and_modify(|job| {
        if let Monkey::Job(m1, m2, op) = job {
            *job = Monkey::Job(m1.clone(), m2.clone(), '=');
        }
    });


    for i in 3_876_907_167_495..3_876_907_167_496{
        input.entry("humn".to_string()).and_modify(|job| {
            if let Monkey::Num(n) = job {
                *job = Monkey::Num(i);
            }
        });
        if tree_walk(&input, &start) == 0 {
            return i
        }
    }
    -1
    /*
    let mut input = input.clone();
    input.entry("root".to_string()).and_modify(|job| {
        if let Monkey::Job(m1, m2, op) = job {
            *job = Monkey::Job(m1.clone(), m2.clone(), '=');
        }
    });
    let start = String::from("root");
    println!("{}", equation_generator(&input, &start));
    0
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(24947355373338, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(3876907167495, part2(&input));
    }
}
