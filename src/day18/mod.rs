use std::collections::HashSet;
use once_cell::unsync::Lazy;
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
        .map(|line| line.split(',').map(|v| v.parse::<i64>().unwrap()).collect())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn get_offsets() -> Vec<(i64, i64, i64)> {
    let offsets = Lazy::new(|| {vec![
        (1, 0, 0), (-1, 0, 0),
        (0, 1, 0), (0, -1, 0),
        (0, 0, 1), (0, 0, -1),
    ]});
    offsets.clone()
}

fn count_neighbours(grid: &HashSet<(i64, i64, i64)>, pos: (i64, i64, i64)) -> u64 {
    6 - get_offsets()
        .iter()
        .map(|off| (pos.0 + off.0, pos.1 + off.1, pos.2 + off.2))
        .filter(|neigh| grid.contains(neigh))
        .count() as u64
}

fn part1(input: &[Input]) -> Output {
    let mut exposed = HashSet::new();
    for cube in input {
        exposed.insert((cube[0], cube[1], cube[2]));
    }
    let mut sum = 0;
    for cube in &exposed {
        sum += count_neighbours(&exposed, *cube)
    }

    sum
}

fn is_external(external: &mut HashSet<(i64, i64, i64)>, grid: &HashSet<(i64, i64, i64)>, pos: &(i64, i64, i64), boundries: (i64, i64, i64, i64, i64, i64)) -> bool {
    if grid.contains(pos) {
        return false;
    }

    let mut visited: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut stack = vec![*pos];

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
            external.extend(visited.difference(grid));
            return true;
        }

        if !grid.contains(&next) {
            stack.extend(
                get_offsets()
                .iter()
                .map(|off| (next.0 + off.0, next.1 + off.1, next.2 + off.2)));
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

    exposed
        .iter()
        .map(|cube| {
            get_offsets()
                .iter()
                .map(|off| (cube.0 + off.0, cube.1 + off.1, cube.2 + off.2))
                .filter(|neigh| is_external(&mut external, &exposed, neigh, boundries))
                .count()
        })
        .sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(4512, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(2554, part2(&input));
    }
}
