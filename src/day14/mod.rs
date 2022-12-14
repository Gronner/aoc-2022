use std::collections::HashMap;
use std::cmp::Ordering::*;

use aoc_downloader::download_day;

const DAY: u32 = 14;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

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
            coords.push(re.captures(coord_pair).map(|captured| {
                    (captured[1].parse::<i32>().unwrap(),
                    captured[2].parse::<i32>().unwrap())
            }).unwrap());
        }
        for (from, to) in coords.windows(2).map(|pair| {
            match pair[..] {
                [from, to] => (from, to),
                _ => unreachable!(),
            }
        }) {
            match (from.0.cmp(&to.0), from.1.cmp(&to.1)) {
                (Less, Equal) => {
                    for x in from.0..=to.0 {
                        cave_map.insert((x, from.1), Cave::Rock);
                    }
                },
                (Greater, Equal) => {
                    for x in to.0..=from.0 {
                        cave_map.insert((x, from.1), Cave::Rock);
                    }
                },
                (Equal, Less) => {
                    for y in from.1..=to.1 {
                        cave_map.insert((from.0, y), Cave::Rock);
                    }
                },
                (Equal, Greater) => {
                    for y in to.1..=from.1 {
                        cave_map.insert((from.0, y), Cave::Rock);
                    }
                },
                (a, b) => panic!("Diagonal detected!: {a:?}, {b:?}"),
            }
        }
    }
    cave_map
}

#[allow(dead_code)]
pub fn draw_sample(input: &HashMap<(i32, i32), Cave>, min_x: i32, max_x: i32, max_y: i32) {
    let mut canvas = vec![vec!['.'; (max_x - min_x + 1) as usize]; max_y as usize];


    for entry in input {
        println!("{:?}: {:?}", entry.0.0, entry.0.0 - min_x);
        if *entry.1 == Cave::Rock {
            canvas[(entry.0.1 - 1) as usize ][(entry.0.0 - min_x) as usize] = '#';
        } else if *entry.1 == Cave::Sand {
            canvas[(entry.0.1 - 1) as usize][(entry.0.0 - min_x) as usize] = 'o';
        }
    }
    for line in canvas {
        for pixel in line {
            print!("{pixel}");
        }
        println!();
    }
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn rain_sand(cave_map: &mut HashMap<(i32, i32), Cave>, max_y: i32) -> bool {
    let mut sand_x = 500;
    let mut sand_y = 0;
    while !cave_map.contains_key(&(sand_x, sand_y)) {
        if sand_y > max_y {
            return false;
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
    true
}

fn part1(input: &HashMap<(i32, i32), Cave>) -> Output {
    let mut sand_volume = 0;
    let max_y = *input.keys()
        .map(|(_, y)| y)
        .max()
        .unwrap();

    let mut cave_map = input.clone();
    while rain_sand(&mut cave_map, max_y) {
        sand_volume += 1;
    }
    sand_volume
}

fn part2(input: &HashMap<(i32, i32), Cave>) -> Output {
    const FLOOR_OFFSET: i32  = 2;
    let mut sand_volume = 0;
    let max_y = *input.keys()
        .map(|(_, y)| y)
        .max()
        .unwrap() + FLOOR_OFFSET;

    let mut cave_map = input.clone();
    for floor_x in (500 - max_y - 2)..(500 + max_y + 3) {
        cave_map.insert((floor_x, max_y), Cave::Rock);
    }

    while !cave_map.contains_key(&(500, 0)) {
        rain_sand(&mut cave_map, max_y);
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
