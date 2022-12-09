use aoc_downloader::download_day;

const DAY: u32 = 0;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = u32;
type Output = u32;

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &[Input]) -> Output {
    for val1 in input {
        for val2 in input {
            if 2020 == (val1 + val2) {
                return val1 * val2;
            }
        }
    }
    0
}

fn part2(input: &[Input]) -> Output {
    for val1 in input {
        for val2 in input {
            for val3 in input {
                if 2020 == (val1 + val2 + val3) {
                    return val1 * val2 * val3;
                }
            }
        }
    }
    0
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
