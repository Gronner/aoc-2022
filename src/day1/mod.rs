use aoc_downloader::download_day;

const DAY: u32 = 1;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

fn parse_input(input: Vec<String>) -> Vec<u32> {
    let mut findings = vec![];
    let mut tmp = 0;
    for line in input {
        if line != "" {
            tmp +=  line.parse::<u32>().unwrap();
        } else {
            findings.push(tmp);
            tmp = 0;
        }
    }
    findings
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &Vec<u32>) -> u32 {
    *input.iter().max().unwrap()
}

fn part2(input: &Vec<u32>) -> u32 {
    let mut input = input.clone();
    input.sort();
    input[input.len()-1] + input[input.len()-2] + input[input.len()-3] 
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
