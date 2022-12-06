use std::collections::HashSet;

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
    input[0].chars()
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<Input>) -> u32 {
    let mut processed = 4;
    for block in input.windows(4) {
        let set: HashSet<char> = HashSet::from_iter(block.iter().cloned());
        if set.len() == 4 {
            break;
        }
        processed += 1;
    }
    processed
}

fn part2(input: &Vec<Input>) -> u32 {
    let mut processed = 14;
    for block in input.windows(14) {
        let set: HashSet<char> = HashSet::from_iter(block.iter().cloned());
        if set.len() == 14 {
            break;
        }
        processed += 1;
    }
    processed
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
