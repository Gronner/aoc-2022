use std::collections::{HashSet, HashMap};

use aoc_downloader::download_day;

const DAY: u32 = 14;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Cave;
type Output = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cave {
    Sand,
    Rock,
}

fn parse_input(input: Vec<String>) -> HashMap<(i32, i32), Cave> {
    let re = regex!(r"(\d+),(\d+)");
    let mut cave_map = HashMap::new();
    for line in input{
            let mut coords = vec![];
            for coord_pair in line.split(" -> ") {
                coords.push(re.captures(coord_pair).and_then(|captured| {
                    Some(
                        (captured[1].parse::<i32>().unwrap(),
                        captured[2].parse::<i32>().unwrap())
                    )
                }).unwrap());
            }
            for pair in coords.windows(2) {
                match (pair[0].0.cmp(&pair[1].0), pair[0].1.cmp(&pair[1].1)) {
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => {
                        for x in pair[0].0..=pair[1].0 {
                            cave_map.insert((x, pair[0].1), Cave::Rock);
                        }
                    },
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => {
                        for x in pair[1].0..=pair[0].0 {
                            cave_map.insert((x, pair[0].1), Cave::Rock);
                        }
                    },
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => {
                        for y in pair[0].1..=pair[1].1 {
                            cave_map.insert((pair[0].0, y), Cave::Rock);
                        }
                    },
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => {
                        for y in pair[1].1..=pair[0].1 {
                            cave_map.insert((pair[0].0, y), Cave::Rock);
                        }
                    },
                    (a, b) => panic!("Diagonal detected!: {:?}, {:?}", a, b),
                }

            }
        }

    cave_map
}

#[allow(dead_code)]
pub fn draw_sample(input: HashMap<(i32, i32), Cave>) {
    let mut canvas = vec![vec!['.'; 10]; 11];

    for entry in input {
        if entry.1 == Cave::Rock {
            canvas[entry.0.1 as usize][(entry.0.0 - 494) as usize] = '#';
        } else if entry.1 == Cave::Sand {
            canvas[entry.0.1 as usize][(entry.0.0 - 494) as usize] = 'o';
        }
    }
    for line in canvas {
        for pixel in line {
            print!("{}", pixel);
        }
        println!("");
    }
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &HashMap<(i32, i32), Cave>) -> Output {
    let mut sand_volume = 0;
    let max_y = *input.keys()
        .map(|(_, y)| y)
        .max()
        .unwrap();

    let mut cave_map = input.clone();
    'outer: loop {
        let mut sand_x = 500;
        let mut sand_y = 0;
        while !cave_map.contains_key(&(sand_x, sand_y)) {
            if sand_y > max_y {
                break 'outer;
            }
            if !cave_map.contains_key(&(sand_x, sand_y + 1)) {
                sand_y += 1;
                continue;
            }
            if !cave_map.contains_key(&(sand_x - 1, sand_y + 1)) {
                sand_y += 1;
                sand_x -= 1;
                continue;
            }
            if !cave_map.contains_key(&(sand_x + 1, sand_y + 1)) {
                sand_y += 1;
                sand_x += 1;
                continue;
            }
            cave_map.insert((sand_x, sand_y), Cave::Sand);
        }
        sand_volume += 1;
    }

    sand_volume
}

fn part2(input: &HashMap<(i32, i32), Cave>) -> Output {
    let mut sand_volume = 0;
    let max_y = *input.keys()
        .map(|(_, y)| y)
        .max()
        .unwrap() + 2;

    let mut cave_map = input.clone();

    for floor_x in (500 - max_y - 2)..(500 + max_y + 3) {
        cave_map.insert((floor_x, max_y), Cave::Rock);
    }
    while !cave_map.contains_key(&(500, 0)) {
        let mut sand_x = 500;
        let mut sand_y = 0;
        while !cave_map.contains_key(&(sand_x, sand_y)) {
            if !cave_map.contains_key(&(sand_x, sand_y + 1)) {
                sand_y += 1;
                continue;
            }
            if !cave_map.contains_key(&(sand_x - 1, sand_y + 1)) {
                sand_y += 1;
                sand_x -= 1;
                continue;
            }
            if !cave_map.contains_key(&(sand_x + 1, sand_y + 1)) {
                sand_y += 1;
                sand_x += 1;
                continue;
            }
            cave_map.insert((sand_x, sand_y), Cave::Sand);
        }
        sand_volume += 1;
    }

    sand_volume
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(719, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(23390, part2(&input));
    }
}
