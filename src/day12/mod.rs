use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

use aoc_downloader::download_day;

const DAY: u32 = 12;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Vec<char>;
type Output = u64;

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|l| l.chars().collect())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

type Coord = (usize, usize);

fn find_start_end(input: &[Input]) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let mut plan = (None, None);
    'outer: for (y, row) in input.iter().enumerate() {
        for (x, grid) in row.iter().enumerate() {
            if *grid == 'S' {
                plan.0 = Some((x, y));
            } else if *grid == 'E' {
                plan.1 = Some((x, y));
            }
            if plan.0.is_some() && plan.1.is_some() {
                break 'outer;
            }
        }
    }
    plan
}

fn traverse_bf(input: &[Input], start: Coord, end: Coord) -> Option<u64> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_front((start, 0));

    loop {
        let ((x_pos, y_pos), len) = queue.pop_front()?;
        if (x_pos, y_pos) == end {
            return Some(len)
        }
        for (x_offset, y_offset) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (new_x, new_y) = (
                (x_offset as isize + x_pos as isize) as usize,
                (y_offset as isize + y_pos as isize) as usize,
            );
            let Some(grid) = input.get(new_y).and_then(|row| row.get(new_x)) else { continue };

            if (input[y_pos][x_pos] as u32 + 1) >= (*grid as u32)
                && !visited.contains(&(new_x, new_y)) {
                visited.insert((new_x, new_y));
                queue.push_back(((new_x, new_y), len + 1));
            }
        }
    }
}

fn part1(input: &[Input]) -> Output {
    let (start, end) = find_start_end(input);
    let mut input = input.to_owned();
    input[start.unwrap().1][start.unwrap().0] = 'a';
    input[end.unwrap().1][end.unwrap().0] = 'z';
    traverse_bf(&input, start.unwrap(), end.unwrap()).unwrap()
}

fn part2(input: &[Input]) -> Output {
    let (start, end) = find_start_end(input);
    let mut input = input.to_owned();
    input[start.unwrap().1][start.unwrap().0] = 'a';
    input[end.unwrap().1][end.unwrap().0] = 'z';

    (0..input.len()).cartesian_product(0..input[0].len())
        .filter(|&(y, x)| input[y][x] == 'a')
        .filter_map(|(y, x)| traverse_bf(&input, (x, y), end.unwrap()))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(425, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(418, part2(&input));
    }
}
