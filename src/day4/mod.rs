use std::{str::FromStr, string::ParseError};

use aoc_downloader::download_day;

const DAY: u32 = 4;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

struct Section {
    start: u32,
    end: u32,
}

impl Section {
    pub fn contained(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end ||
            other.start <= self.start && other.end >= self.end 
    }

    pub fn not_overlapping(&self, other: &Self) -> bool {
        self.end < other.start && self.end < other.end ||
            other.end < self.start && other.end < self.end
    }
}

struct ElfPair {
    first: Section,
    second: Section,
}

impl ElfPair {
    pub fn one_bored(&self) -> bool {
        self.first.contained(&self.second)
    }

    pub fn full_work(&self) -> bool {
        self.first.not_overlapping(&self.second)
    }
}

impl FromStr for ElfPair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"(\d+)-(\d+),(\d+)-(\d+)");
        
        Ok(re.captures(s).and_then(|captured| {
            Some(ElfPair {
                first: Section {
                    start: captured[1].parse::<u32>().ok()?,
                    end: captured[2].parse::<u32>().ok()?,
                },
                second: Section {
                    start: captured[3].parse::<u32>().ok()?,
                    end: captured[4].parse::<u32>().ok()?,
                },
            })
        }).unwrap())
    }
}

fn parse_input(input: Vec<String>) -> Vec<ElfPair> {
    input
        .iter()
        .map(|elfpair| ElfPair::from_str(elfpair).unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<ElfPair>) -> u32 {
    input.iter()
        .filter(|ep| ep.one_bored())
        .count() as u32
}

fn part2(input: &Vec<ElfPair>) -> u32 {
    let all_pairs = input.len() as u32;
    all_pairs - input.iter()
        .filter(|ep| ep.full_work())
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(595, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(952, part2(&input));
    }
}
