use aoc_downloader::download_day;
use std::collections::HashMap;

const DAY: u32 = 2;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn parse_input(input: Vec<String>) -> Vec<Vec<char>> {
    input
        .iter()
        .map(|v| {
            v.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<Vec<char>>) -> u32 {
    let mut points = HashMap::new();
    points.insert( 'X', 1);
    points.insert( 'Y', 2);
    points.insert( 'Z', 3);
    let mut score = 0;
    for round in input {
        score += match (round[0], round[2]) {
            ('A', 'X') => 3,
            ('B', 'Y') => 3,
            ('C', 'Z') => 3,
            ('A', 'Y') => 6,
            ('B', 'Z') => 6,
            ('C', 'X') => 6,
            ('A', 'Z') => 0,
            ('B', 'X') => 0,
            ('C', 'Y') => 0,
            (_, _) => panic!(),
        };
        score += points.get(&round[2]).unwrap();
    }
    score
}

fn part2(input: &Vec<Vec<char>>) -> u32 {
    let mut points = HashMap::new();
    points.insert( 'X', 1);
    points.insert( 'Y', 2);
    points.insert( 'Z', 3);

    let mut outcome_A = HashMap::new();
    outcome_A.insert('Y', 'X');
    outcome_A.insert('X', 'Z');
    outcome_A.insert('Z', 'Y');

    let mut outcome_B = HashMap::new();
    outcome_B.insert('Y', 'Y');
    outcome_B.insert('X', 'X');
    outcome_B.insert('Z', 'Z');

    let mut outcome_C = HashMap::new();
    outcome_C.insert('Y', 'Z');
    outcome_C.insert('X', 'Y');
    outcome_C.insert('Z', 'X');

    let mut score = 0;
    for round in input {
        score += match (round[0], round[2]) {
            (_, 'X') => 0,
            (_, 'Y') => 3,
            (_, 'Z') => 6,
            (_, _) => panic!(),
        };
        let outcome = match round[0] {
            'A' => outcome_A.get(&round[2]).unwrap(),
            'B' => outcome_B.get(&round[2]).unwrap(),
            'C' => outcome_C.get(&round[2]).unwrap(),
            _ => panic!(),
        };
        score += points.get(&outcome).unwrap();
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(13809, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(12316, part2(&input));
    }
}
