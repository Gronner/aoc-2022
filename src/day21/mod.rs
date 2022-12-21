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
type Output = f64;

#[derive(Clone, Debug)]
enum Monkey {
    Job(String, String, char),
    Num(f64)
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_mon_re = regex!(r"(\d+)");
        let job_mon_re = regex!(r"(\w+) ([\+\-*/=]) (\w+)");
        if let Some(monkey) = num_mon_re.captures(s).map(|captured| {
            Monkey::Num(captured[1].parse::<f64>().unwrap())
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

fn tree_walk(monkey_tree: &HashMap<String, Monkey>, node: &str) -> f64 {
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
                    0.0
                } else {
                    1.0
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
        return vec!["(".to_string(), equation_generator(monkey_tree, left), op.to_string(), equation_generator(monkey_tree, right), ")".to_string()].join(" ");
    }

    unreachable!();
}

fn part1(input: &HashMap<String, Monkey>) -> Output {
    let start = String::from("root");
    tree_walk(&input, &start)
}

fn part2(input: &HashMap<String, Monkey>) -> f64{
    let mut input = input.clone();
    let (left, right) = &input.get(&"root".to_string()).and_then(|job| {
        if let Monkey::Job(m1, m2, _) = job {
            return Some((m1.clone(), m2.clone()));
        } else {
            return None;
        };
    }).unwrap();

    let result = tree_walk(&input, &right);

    let mut low = 0.0;
    let mut high = 10_000_000_000_000.0;

    while low < high {
        let pivot = ((low + high) / 2.0_f64).round();
        input.entry("humn".to_string()).and_modify(|job| {
            if let Monkey::Num(_) = job {
                *job = Monkey::Num(pivot);
            }
        });
        match result.total_cmp(&tree_walk(&input, &left)) {
            std::cmp::Ordering::Less => low = pivot,
            std::cmp::Ordering::Equal => return pivot,
            std::cmp::Ordering::Greater => high = pivot,
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(24947355373338.0, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(3876907167495.0, part2(&input));
    }
}
