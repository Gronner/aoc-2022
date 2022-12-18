use std::collections::{HashSet, HashMap};

use aoc_downloader::download_day;

const DAY: u32 = 18;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Vec<i64>;
type Output = u64;

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|line| line.split(",").map(|v| v.parse::<i64>().unwrap()).collect())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn get_neighbours(pos: (i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    let offsets = vec![
        (1, 0, 0), (-1, 0, 0),
        (0, 1, 0), (0, -1, 0),
        (0, 0, 1), (0, 0, -1),
    ];
    let mut neighbours = vec![];
    for offset in offsets {
        neighbours.push((pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2));
    }
    neighbours
}

fn count_neighbours(grid: &HashMap<(i64, i64, i64), u64>, pos: (i64, i64, i64)) -> u64 {
    let mut uncovered = *grid.get(&pos).unwrap();

    for neighbour in get_neighbours(pos) {
        if grid.contains_key(&neighbour) {
            uncovered -= 1;
        }

    }

    uncovered
}

fn part1(input: &[Input]) -> Output {
    let mut exposed = HashMap::new();
    for cube in input {
        exposed.insert((cube[0], cube[1], cube[2]), 6);
    }
    let mut sum = 0;
    for cube in exposed.keys() {
        sum += count_neighbours(&exposed, *cube)
    }

    sum
}

fn is_external(external: &mut HashSet<(i64, i64, i64)>, grid: &HashSet<(i64, i64, i64)>, pos: (i64, i64, i64), boundries: (i64, i64, i64, i64, i64, i64)) -> bool {
    if grid.contains(&pos) {
        return false;
    }

    let mut visited: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut stack = vec![pos];

    while !stack.is_empty() {
        let next = stack.pop().unwrap();
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next);

        if external.contains(&next) 
            || !(boundries.0..=boundries.1).any(|v| v == next.0)
            || !(boundries.2..=boundries.3).any(|v| v == next.1)
            || !(boundries.4..=boundries.5).any(|v| v == next.2) {
            let diff: HashSet<(i64, i64, i64)> = visited.difference(&grid).map(|&pos| pos).collect();
            external.extend(&diff);
            return true;
        }

        if !grid.contains(&next) {
            stack.append(&mut get_neighbours(next));
        }
    }

    false
}

fn part2(input: &[Input]) -> Output {
    let mut exposed = HashSet::new();
    for cube in input {
        exposed.insert((cube[0], cube[1], cube[2]));
    }

    let min_x = exposed.iter()
        .map(|(x, _, _)| x)
        .min()
        .unwrap();

    let max_x = exposed.iter()
        .map(|(x, _, _)| x)
        .max()
        .unwrap();

    let min_y = exposed.iter()
        .map(|(_, y, _)| y)
        .min()
        .unwrap();

    let max_y = exposed.iter()
        .map(|(_, y, _)| y)
        .max()
        .unwrap();

    let min_z = exposed.iter()
        .map(|(_, _, z)| z)
        .min()
        .unwrap();

    let max_z = exposed.iter()
        .map(|(_, _, z)| z)
        .max()
        .unwrap();

    let boundries = (*min_x, *max_x, *min_y, *max_y, *min_z, *max_z);

    let mut external = HashSet::new();

    let mut sum = 0;

    for cube in &exposed {
        for neighbour in get_neighbours(*cube) {
            if is_external(&mut external, &exposed, neighbour, boundries) {
                sum += 1;
            }
        }
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
