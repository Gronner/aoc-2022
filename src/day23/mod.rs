use aoc_downloader::download_day;
use std::collections::{HashSet, HashMap, VecDeque};
use once_cell::unsync::Lazy;

const DAY: u32 = 23;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Coords = (i64, i64);
type Input = HashSet<Coords>;
type Output = i64;

fn parse_input(input: Vec<String>) -> Input {
    let mut map = HashSet::new();
    for (y, row) in input.iter().enumerate() {
        for (x, spot) in row.chars().enumerate() {
            if spot == '#' {
                map.insert((x as i64, y as i64));
            }
        }
    }
    map
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn get_offsets() -> Vec<Coords> {
    let offsets = Lazy::new(|| {vec![
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ]});
    offsets.clone()
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Directions {
    North,
    East,
    South,
    West,
}

fn draw_input(map: &Input) {
    let min_x = *map.iter()
        .map(|(x, _)| x)
        .min()
        .unwrap();

    let max_x = *map.iter()
        .map(|(x, _)| x)
        .max()
        .unwrap();

    let min_y = *map.iter()
        .map(|(_, y)| y)
        .min()
        .unwrap();

    let max_y = *map.iter()
        .map(|(_, y)| y)
        .max()
        .unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

}

fn get_movement_options(round: usize) -> Vec<Directions>{
    use Directions::*;
    let mut options = VecDeque::from(vec![North, South, West, East]);
    options.rotate_left((round - 1) % 4);
    Vec::from(options)
}

fn part1(input: &Input) -> Output {
    use Directions::*;

    let mut no_movement = true;
    let mut map = input.clone();

    let mut i = 1;
    loop {
        let mut proposed_movement: HashMap<Coords, (i64, Coords)> = HashMap::new();
        for elf in &map {
            let mut movement_options = get_movement_options(i);
            for offset in get_offsets() {
                let neighbour = (elf.0 + offset.0, elf.1 + offset.1);
                if map.contains(&neighbour) {
                    match offset {
                        (-1, -1) => {
                            movement_options.retain(|m| *m != North);
                            movement_options.retain(|m| *m != West);
                        },
                        (0, -1) => {
                            movement_options.retain(|m| *m != North);
                        },
                        (1, -1) => {
                            movement_options.retain(|m| *m != North);
                            movement_options.retain(|m| *m != East);
                        },
                        (1,  0) => {
                            movement_options.retain(|m| *m != East);
                        },
                        (-1,  1) => {
                            movement_options.retain(|m| *m != South);
                            movement_options.retain(|m| *m != West);
                        },
                        (0,  1) => {
                            movement_options.retain(|m| *m != South);
                        },
                        (1,  1) => {
                            movement_options.retain(|m| *m != South);
                            movement_options.retain(|m| *m != East);
                        },
                        (-1,  0) => {
                            movement_options.retain(|m| *m != West);
                        },
                        _ => unreachable!(),
                    }
                }
            }
            if movement_options.len() == 4 {
                no_movement = true;
                continue;
            }
            if movement_options.is_empty() {
                continue;
            }
            no_movement = false;
            let proposed_move = match movement_options[0] {
                North => (elf.0, elf.1 - 1),
                East => (elf.0 + 1, elf.1),
                South => (elf.0, elf.1 + 1),
                West => (elf.0 - 1, elf.1),
            };
            proposed_movement.entry(proposed_move).and_modify(|mut e| {
                e.0 += 1;
            })
            .or_insert(
                (
                    1,
                    *elf
                )
            );
        }

        for movement in proposed_movement {
            if movement.1.0 > 1 {
                continue;
            }
            assert!(map.remove(&movement.1.1));
            map.insert(movement.0);
        }
        assert!(map.len() == input.len());
        
        i += 1;
        if i > 10 {
            break;
        }
    }

    let min_x = map.iter()
        .map(|(x, _)| x)
        .min()
        .unwrap();

    let max_x  = map.iter()
        .map(|(x, _)| x)
        .max()
        .unwrap();

    let min_y = map.iter()
        .map(|(_, y)| y)
        .min()
        .unwrap();

    let max_y  = map.iter()
        .map(|(_, y)| y)
        .max()
        .unwrap();

    (max_x - min_x + 1) * (max_y - min_y + 1) - input.len() as i64
}

fn part2(input: &Input) -> Output {
    use Directions::*;

    let mut no_movement = true;
    let mut map = input.clone();

    let mut i = 1;
    loop {
        let mut proposed_movement: HashMap<Coords, (i64, Coords)> = HashMap::new();
        for elf in &map {
            let mut movement_options = get_movement_options(i);
            for offset in get_offsets() {
                let neighbour = (elf.0 + offset.0, elf.1 + offset.1);
                if map.contains(&neighbour) {
                    match offset {
                        (-1, -1) => {
                            movement_options.retain(|m| *m != North);
                            movement_options.retain(|m| *m != West);
                        },
                        (0, -1) => {
                            movement_options.retain(|m| *m != North);
                        },
                        (1, -1) => {
                            movement_options.retain(|m| *m != North);
                            movement_options.retain(|m| *m != East);
                        },
                        (1,  0) => {
                            movement_options.retain(|m| *m != East);
                        },
                        (-1,  1) => {
                            movement_options.retain(|m| *m != South);
                            movement_options.retain(|m| *m != West);
                        },
                        (0,  1) => {
                            movement_options.retain(|m| *m != South);
                        },
                        (1,  1) => {
                            movement_options.retain(|m| *m != South);
                            movement_options.retain(|m| *m != East);
                        },
                        (-1,  0) => {
                            movement_options.retain(|m| *m != West);
                        },
                        _ => unreachable!(),
                    }
                }
            }
            if movement_options.len() == 4 || movement_options.is_empty() {
                continue;
            }
            no_movement = false;
            let proposed_move = match movement_options[0] {
                North => (elf.0, elf.1 - 1),
                East => (elf.0 + 1, elf.1),
                South => (elf.0, elf.1 + 1),
                West => (elf.0 - 1, elf.1),
            };
            proposed_movement.entry(proposed_move).and_modify(|mut e| {
                e.0 += 1;
            })
            .or_insert(
                (
                    1,
                    *elf
                )
            );
        }

        if proposed_movement.is_empty() {
            break;
        }

        for movement in proposed_movement {
            if movement.1.0 > 1 {
                continue;
            }
            assert!(map.remove(&movement.1.1));
            map.insert(movement.0);
        }
        assert!(map.len() == input.len());
        
        i += 1;
    }
    i as i64
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
