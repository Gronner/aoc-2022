use aoc_downloader::download_day;

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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Play {
    fn from(input: char) -> Self {
        match input { 
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            e => panic!("Unknown play: {}", e),
        }
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;

        match (self, other) {
            (Self::Rock, Self::Paper) => Some(Less),
            (Self::Rock, Self::Scissors) => Some(Greater),
            (Self::Paper, Self::Rock) => Some(Greater),
            (Self::Paper, Self::Scissors) => Some(Less),
            (Self::Scissors, Self::Rock) => Some(Less),
            (Self::Scissors, Self::Paper) => Some(Greater),
            (_, _) => Some(Equal),
        }
    }
}

impl std::fmt::Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Rock => "Rock",
            Self::Paper => "Paper",
            Self::Scissors => "Scissors",
        })
    }
}

impl Play {
    pub fn scoring(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn compare(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Self::Rock, Self::Scissors) => Outcome::Win,
            (Self::Rock, Self::Paper) => Outcome::Loss,
            (Self::Paper, Self::Rock) => Outcome::Win,
            (Self::Paper, Self::Scissors) => Outcome::Loss,
            (Self::Scissors, Self::Paper) => Outcome::Win,
            (Self::Scissors, Self::Rock) => Outcome::Loss,
            (_, _) => Outcome::Draw,
        }
    }
}

fn part1(input: &Vec<Vec<char>>) -> u32 {
    let strategy = input.iter()
        .map(|round| (Play::from(round[0]), Play::from(round[2])))
        .collect::<Vec<_>>();

    let mut score = 0;
    for round in strategy {
        score += round.1.compare(&round.0).scoring();
        score += round.1.scoring();
    }
    score
}

pub enum Outcome {
    Loss,
    Draw,
    Win,
}

impl From<char> for Outcome {
    fn from(input: char) -> Self {
        match input {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            e => panic!("Unkown result: {}", e),
        }
    }
}

impl Outcome {
    pub fn scoring(&self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

impl Play {
    pub fn outcome(&self, result: Outcome) -> Self {
        match (self, result) {
            (Play::Rock, Outcome::Win) => Play::Paper,
            (Play::Rock, Outcome::Loss) => Play::Scissors,
            (Play::Paper, Outcome::Win) => Play::Scissors,
            (Play::Paper, Outcome::Loss) => Play::Rock,
            (Play::Scissors, Outcome::Win) => Play::Rock,
            (Play::Scissors, Outcome::Loss) => Play::Paper,
            (_, Outcome::Draw) => *self,
        }
    }
}

fn part2(input: &Vec<Vec<char>>) -> u32 {
    let playbook = input.iter()
        .map(|round| (Play::from(round[0]), Outcome::from(round[2])))
        .collect::<Vec<_>>();
    let mut score = 0;
    for round in playbook {
        score += round.1.scoring();
        score += round.0.outcome(round.1).scoring();
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
