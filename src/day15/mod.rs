use std::{str::FromStr, num::ParseIntError};

use aoc_downloader::download_day;

const DAY: u32 = 15;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Sensor;
type Output = u32;

struct Sensor {
    pub pos: (isize, isize),
    pub closest_beacon: (isize, isize),
}

fn manhatten_distance(pos_a: (isize, isize), pos_b: (isize, isize)) -> isize {
        (pos_a.0 - pos_b.0).abs() + (pos_a.1 - pos_b.1).abs()
}

impl Sensor {
    pub fn get_exlusive_range(&self) -> isize {
        manhatten_distance(self.pos, self.closest_beacon)
    }

    pub fn in_ex_range(&self, position: (isize, isize)) -> bool {
        manhatten_distance(self.pos, position) <= self.get_exlusive_range()
    }

    pub fn is_beacon(&self, position: (isize, isize)) -> bool {
        self.closest_beacon == position
    }

    pub fn get_outer_ring(&self) -> Vec<(isize, isize)> {
        let mut outer_ring = Vec::new();
        for direction_x in vec![-1, 1] {
            for direction_y in vec![-1, 1] {
                for delta_x in 0..=(self.get_exlusive_range() + 1) {
                    let delta_y = self.get_exlusive_range() + 1 - delta_x;
                    let pos_x = self.pos.0 + delta_x * direction_x;
                    let pos_y = self.pos.1 + delta_y * direction_y;
                    outer_ring.push((pos_x, pos_y));
                }
            }
        }
        outer_ring
    }
}

impl FromStr for Sensor {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)");

        Ok(re.captures(s).map(|captured| {
            Sensor {
                pos: (
                         captured[1].parse::<isize>().unwrap(),
                         captured[2].parse::<isize>().unwrap(),
                     ),
             closest_beacon: (
                         captured[3].parse::<isize>().unwrap(),
                         captured[4].parse::<isize>().unwrap(),
             ),
            }
        }).unwrap())

    }
}


fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|line| Sensor::from_str(line).unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &[Input]) -> Output {
    let min_x = input
        .iter()
        .map(|sensor| {
            sensor.pos.0 - sensor.get_exlusive_range()
        })
        .min()
        .unwrap();
    let max_x = input
        .iter()
        .map(|sensor| {
            sensor.pos.0 + sensor.get_exlusive_range()
        })
        .max()
        .unwrap();
    let mut empty_and_exlusive = 0;
    for x in min_x..=max_x {
        let pos = (x, 2000000);
        let mut exclusive = false;
        for sensor in input {
            if sensor.in_ex_range(pos) {
                exclusive = true;
                break;
            }
        }
        if !exclusive {
            continue;
        }
        let mut empty = true;
        for sensor in input {
            if sensor.is_beacon(pos) {
                empty = false;
                break;
            }
        }
        if exclusive && empty {
            empty_and_exlusive += 1;
        }
    }
    empty_and_exlusive
}

fn part2(input: &[Input]) -> Output {
    let size = 4000000;
    for sensor in input {
        for pos in sensor.get_outer_ring() {
            if pos.0 < 0 || pos.1 < 0 || pos.0 > size || pos.1 > size {
                continue;
            }
            let mut exclusive = false;
            for sensor in input {
                if sensor.in_ex_range(pos) {
                    exclusive = true;
                    break;
                }
            }
            if !exclusive {
                print!("{} - {}", pos.0, pos.1);
                return (pos.0 * 4000000 + pos.1) as u32;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(5838453, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(70276940, part2(&input));
    }
}
