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

fn marker_start(size: usize, input: &Vec<Input>) -> u32 {
    let mut processed = size;
    for block in input.windows(size) {
        let set: HashSet<char> = HashSet::from_iter(block.iter().cloned());
        if set.len() == size {
            break;
        }
        processed += 1;
    }
    processed as u32
}

fn part1(input: &Vec<Input>) -> u32 {
    marker_start(4, input)
}

fn part2(input: &Vec<Input>) -> u32 {
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
