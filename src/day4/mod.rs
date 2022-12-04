use aoc_downloader::download_day;
use regex::Regex;
use once_cell::sync::OnceCell;

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

const DAY: u32 = 4;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn parse_input(input: Vec<String>) -> Vec<Vec<(u32, u32)>> {
    let re = regex!(r"(\d+)-(\d+),(\d+)-(\d+)");
    input
        .iter()
        .map(|v| {
            re.captures(v).and_then(|captured| {
                Some(vec![
                    (captured[1].parse::<u32>().unwrap(),
                    captured[2].parse::<u32>().unwrap()),
                    (captured[3].parse::<u32>().unwrap(),
                    captured[4].parse::<u32>().unwrap())],
                )
            }).unwrap()
        })
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<Vec<(u32, u32)>>) -> u32 {
    let mut contained = 0;
    for elves in input {
        if elves[0].0 <= elves[1].0 && elves[0].1 >= elves[1].1 {
            contained += 1;
            continue;
        }
        if elves[1].0 <= elves[0].0 && elves[1].1 >= elves[0].1 {
            contained += 1;
        }
    }
    contained
}

fn part2(input: &Vec<Vec<(u32, u32)>>) -> u32 {
    let mut contained = 0;
    for elves in input {
        if elves[0].0 <= elves[1].0 && elves[0].1 >= elves[1].1 {
            contained += 1;
            continue;
        }
        if elves[1].0 <= elves[0].0 && elves[1].1 >= elves[0].1 {
            contained += 1;
            continue;
        }
        if elves[0].0 >= elves[1].0 && elves[0].0 <= elves[1].1 {
            contained += 1;
            continue;
        }
        if elves[1].0 >= elves[0].0 && elves[1].0 <= elves[0].1 {
            contained += 1;
            continue;
        }
    }
    contained
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
