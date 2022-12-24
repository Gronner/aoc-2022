use std::collections::{HashSet, HashMap};
use once_cell::unsync::Lazy;

use aoc_downloader::download_day;

const DAY: u32 = 24;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = u64;
type Output = u64;
type Coords = (i64, i64);

fn parse_input(input: Vec<String>) -> (Vec<(Blizzard, Coords)>, i64, i64) {
    use Blizzard::*;

    let mut map = Vec::new();

    let max_x = input[0].len() as i64 - 1;
    let max_y = input.len() as i64 - 1;

    for (y, row) in input.iter().enumerate() {
        for (x, grid) in row.chars().enumerate() {
            match grid {
                '>' => map.push((Right, (x as i64, y as i64))),
                '<' => map.push((Left, (x as i64, y as i64))),
                'v' => map.push((Down, (x as i64, y as i64))),
                '^' => map.push((Up,  (x as i64, y as i64))),
                '.' | '#' => continue,
                e => panic!("Unkown symbol: {}", e),
            };
        }
    }
    (map, max_x, max_y)
}

#[derive(Clone, Copy, Debug, Hash)]
enum Blizzard {
    Right,
    Left,
    Up,
    Down,
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn blizz_next_pos(blizz: Blizzard, pos: Coords, max: Coords) -> Coords {
    use Blizzard::*;
    let offset = match blizz {
        Right => (1, 0),
        Left => (-1, 0),
        Up => (0, -1),
        Down => (0, 1),
    };
    (
        if pos.0 + offset.0 == 0 { max.0 - 1 } else if pos.0 + offset.0 == max.0 { 1 } else { pos.0 + offset.0 },
        if pos.1 + offset.1 == 0 { max.1 - 1 } else if pos.1 + offset.1 == max.1 { 1 } else { pos.1 + offset.1 },
    )
}

fn move_blizzards(map: &Vec<(Blizzard, Coords)>, max: Coords) -> Vec<(Blizzard, Coords)> {
    let mut future = Vec::new();
    for blizz in map {
        future.push((blizz.0, blizz_next_pos(blizz.0, blizz.1, max)));
    }
    future
}

fn own_moves(pos: Coords, max: Coords) -> Vec<Coords> {
    let offsets = Lazy::new(|| {vec![
        (0, 1), (1, 0), (-1, 0), (0, -1), (0, 0)
    ]});

    let mut options = Vec::new();
    for offset in offsets.iter() {
        if pos.0 + offset.0 == 1 && pos.1 + offset.1 == 0 {
            options.push((pos.0 + offset.0 , pos.1 + offset.1));
        } else if pos.0 + offset.0 == (max.0 - 1) && pos.1 + offset.1 == max.1 {
            options.push((pos.0 + offset.0 , pos.1 + offset.1));
        } else if pos.0 + offset.0 == 0 || pos.0 + offset.0 == max.0
            || pos.1 + offset.1 <= 0 || pos.1 + offset.1 == max.1 
        {
                continue;
        } 
        options.push((pos.0 + offset.0 , pos.1 + offset.1));
    }

    options
}

fn part1(input: &(Vec<(Blizzard, Coords)>, i64, i64)) -> Output {
    let start = (1, 0);

    let mut map = input.0.clone();
    let max_x = input.1;
    let max_y = input.2;
    let end = (max_x - 1, max_y);

    let mut minutes = 0;
    let mut prev = HashSet::new();
    prev.insert(start);
    let blocked = map
        .iter()
        .map(|(_, pos)| *pos)
        .collect::<HashSet<Coords>>();
    loop {
        map = move_blizzards(&map, (max_x, max_y));
        let blocked = map
            .iter()
            .map(|(_, pos)| *pos)
            .collect::<HashSet<Coords>>();


        let mut next_options = HashSet::new();
        for pos in &prev {
            for mo in own_moves(*pos, (max_x, max_y)) {
                if !blocked.contains(&mo) {
                    next_options.insert(mo);
                }
            }
        }
        minutes += 1;
        if next_options.contains(&end) {
            break;
        }
        prev = next_options.clone();
    }
    minutes
}

fn part2(input: &(Vec<(Blizzard, Coords)>, i64, i64)) -> Output {
    let start = (1, 0);

    let mut map = input.0.clone();
    let max_x = input.1;
    let max_y = input.2;
    let end = (max_x - 1, max_y);

    let mut minutes = 0;
    let mut prev = HashSet::new();
    prev.insert(start);
    let blocked = map
        .iter()
        .map(|(_, pos)| *pos)
        .collect::<HashSet<Coords>>();
    let mut run = 1;
    loop {
        map = move_blizzards(&map, (max_x, max_y));
        let blocked = map
            .iter()
            .map(|(_, pos)| *pos)
            .collect::<HashSet<Coords>>();


        let mut next_options = HashSet::new();
        for pos in &prev {
            for mo in own_moves(*pos, (max_x, max_y)) {
                if !blocked.contains(&mo) {
                    next_options.insert(mo);
                }
            }
        }
        minutes += 1;
        if run == 1 {
            if next_options.contains(&end) {
                run = 2;
                next_options.clear();
                next_options.insert(end);
            }
        }
        if run == 2 {
            if next_options.contains(&start) {
                run = 3;
                next_options.clear();
                next_options.insert(start);
            }
        }
        if run == 3 {
            if next_options.contains(&end) {
                break;
            }

        }
        prev = next_options.clone();
    }
    minutes
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
