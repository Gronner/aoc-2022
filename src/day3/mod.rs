use aoc_downloader::download_day;
use std::collections::HashSet;
use std::iter::FromIterator;

const DAY: u32 = 3;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn parse_input(input: Vec<String>) -> Vec<Vec<char>> {
    input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
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

fn score_item(item: char) -> u32 {
    if item.is_lowercase() {
        item as u32 - ('a' as u32) + 1
    } else {
        item as u32 - ('A' as u32) + 27
    }
}

fn part1(input: &Vec<Vec<char>>) -> u32 {
    let mut priority = 0;
    for backpack in input {
        let bagsize = backpack.len();
        let left_compartment: HashSet<&char> = HashSet::from_iter(&backpack[..bagsize / 2]);
        let right_compartment = HashSet::from_iter(&backpack[bagsize / 2..]);
        let in_common = left_compartment.intersection(&right_compartment);
        priority += in_common
            .into_iter()
            .map(|&&item| score_item(item))
            .sum::<u32>();
    }
    priority
}

fn part2(input: &[Vec<char>]) -> u32 {
    let mut priority = 0;
    for backpack in input.chunks(3) {
        let elf1: HashSet<&char> = HashSet::from_iter(&backpack[0]);
        let elf2 = HashSet::from_iter(&backpack[1]);
        let elf3 = HashSet::from_iter(&backpack[2]);
        let in_common = elf1.intersection(&elf2).cloned().collect::<HashSet<_>>();
        let in_common = in_common.intersection(&elf3);
        priority += in_common
            .into_iter()
            .map(|&&item| score_item(item))
            .sum::<u32>();
    }
    priority
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(7821, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(2752, part2(&input));
    }
}
