use itertools::Itertools;
use std::collections::HashMap;

use aoc_downloader::download_day;

const DAY: u32 = 6;

type Input = char;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input[0].chars().collect::<Vec<_>>()
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

fn marker_start(size: usize, input: &[Input]) -> u32 {
    input
        .windows(size)
        .take_while(|window| (*window).iter().unique().count() != size)
        .count() as u32
        + size as u32
}

// Worse, more complicated but I was challenged to do it without sets!
#[allow(dead_code)]
#[allow(clippy::all)]
fn alt_marker(size: usize, input: &Vec<Input>) -> u32 {
    let mut state = HashMap::new();
    for i in 0..size {
        state
            .entry(input[i])
            .and_modify(|val| *val += 1)
            .or_insert(1);
    }
    if state.iter().filter(|(_, count)| **count == 1).count() == size {
        return size as u32;
    }
    for i in size..input.len() {
        let backmost_char = input[i - size];
        state.entry(backmost_char).and_modify(|val| *val -= 1);
        if *state.get(&backmost_char).unwrap() == 0 {
            state.remove(&backmost_char).unwrap();
        }
        state
            .entry(input[i])
            .and_modify(|val| *val += 1)
            .or_insert(1);
        if state.iter().filter(|(_, count)| **count == 1).count() == size {
            return i as u32 + 1;
        }
    }
    0
}

fn part1(input: &[Input]) -> u32 {
    marker_start(4, input)
}

fn part2(input: &[Input]) -> u32 {
    marker_start(14, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(1987, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(3059, part2(&input));
    }
}
