use aoc_downloader::download_day;

const DAY: u32 = 25;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = String;
type Output = String;

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut num = 0;
    for (exp, c) in snafu.chars().rev().enumerate() {
        let digit = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            e => panic!("Unexpected digid: {e}"),
        };
        num += 5_i64.pow(exp as u32) * digit;
    }
    num
}

fn decimal_to_snafu(mut decimal: i64) -> String {
    let mut snafu = String::new();
    loop {
        let base = decimal % 5;
        match base {
            0 => {
                snafu.push('0');
                decimal /= 5;
            },
            1 => {
                snafu.push('1');
                decimal -= 1;
                decimal /= 5;
            },
            2 => {
                snafu.push('2');
                decimal -= 2;
                decimal /= 5;
            },
            3 => {
                snafu.push('=');
                decimal += 2;
                decimal /= 5;
            },
            4 => {
                snafu.push('-');
                decimal += 1;
                decimal /= 5;
            }
            e => panic!("Unexpected value: {e}"),
        }
        if decimal == 0 {
            break;
        }
    }
    snafu.chars().rev().collect::<String>()
}

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|l| String::from(l))
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &[Input]) -> Output {
    let sum = input.iter()
        .map(|n| snafu_to_decimal(n))
        .sum();
    decimal_to_snafu(sum)
}

fn part2(input: &[Input]) -> Output {
    String::from("Done")
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
