use aoc_downloader::download_day;
use std::collections::VecDeque;

const DAY: u32 = 20;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = i64;
type Output = i64;

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &[Input]) -> Output {
    let length = input.len();
    let mut idxs = (0..length).collect::<VecDeque<_>>();

    for (idx, &num) in input.iter().enumerate() {
        let pos = idxs.iter().position(|&i| i == idx).unwrap();
        idxs.remove(pos);
        let new_pos = (pos as isize + num as isize).rem_euclid(idxs.len() as isize) as usize;
        idxs.insert(new_pos, idx);
    }
    let zero_old_pos = input.iter().position(|&n| n == 0).unwrap();
    let zero_idx = idxs.iter().position(|&i| i == zero_old_pos).unwrap();
    let mut sum = 0;
    for i in [1000, 2000, 3000] {
        sum += input[idxs[(zero_idx + i) % idxs.len()]];
    }
    sum
}

fn part2(input: &[Input]) -> Output {
    let length = input.len();
    let input = input.iter().map(|&n| n * 811589153).collect::<Vec<_>>();
    let mut idxs = (0..length).collect::<VecDeque<_>>();

    for _ in 0..10 {
        for (idx, &num) in input.iter().enumerate() {
            let pos = idxs.iter().position(|&i| i == idx).unwrap();
            idxs.remove(pos);
            let new_pos = (pos as isize + num as isize).rem_euclid(idxs.len() as isize) as usize;
            idxs.insert(new_pos, idx);
        }
    }
    let zero_old_pos = input.iter().position(|&n| n == 0).unwrap();
    let zero_idx = idxs.iter().position(|&i| i == zero_old_pos).unwrap();
    let mut sum = 0;
    for i in [1000, 2000, 3000] {
        sum += input[idxs[(zero_idx + i) % idxs.len()]];
    }
    sum
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
