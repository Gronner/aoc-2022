use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
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
type Output = u64;

struct Sensor {
    pub pos: (isize, isize),
    pub closest_beacon: (isize, isize),
    pub ex_range: isize,
}

fn manhatten_distance(pos_a: (isize, isize), pos_b: (isize, isize)) -> isize {
        (pos_a.0 - pos_b.0).abs() + (pos_a.1 - pos_b.1).abs()
}

impl Sensor {
    pub fn in_ex_range(&self, position: (isize, isize)) -> bool {
        manhatten_distance(self.pos, position) <= self.ex_range
    }

    pub fn is_beacon(&self, position: (isize, isize)) -> bool {
        self.closest_beacon == position
    }
}



impl FromStr for Sensor {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)");


        Ok(re.captures(s).map(|captured| {
            let sensor_x = captured[1].parse::<isize>().unwrap();
            let sensor_y = captured[2].parse::<isize>().unwrap();
            let beacon_x = captured[3].parse::<isize>().unwrap();
            let beacon_y = captured[4].parse::<isize>().unwrap();
            let manhatten = manhatten_distance((sensor_x, sensor_y), (beacon_x, beacon_y));

            Sensor {
                pos: (sensor_x, sensor_y),
                closest_beacon: (beacon_x, beacon_y),
                ex_range: manhatten,
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
            sensor.pos.0 - sensor.ex_range
        })
        .min()
        .unwrap();
    let max_x = input
        .iter()
        .map(|sensor| {
            sensor.pos.0 + sensor.ex_range
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

fn not_in_range(pos: (isize, isize), max_range: isize) -> bool {
    pos.0 < 0 || pos.0 > max_range || pos.1 < 0 || pos.1 > max_range 
}

fn part2(input: &[Input]) -> Output {
    let size = 4000000;
    for sensor in input {
        let mut generator = move || {
            for direction_x in &[-1, 1] {
                for direction_y in &[-1, 1] {
                    for delta_x in 0..=(sensor.ex_range + 1) {
                        let delta_y = sensor.ex_range + 1 - delta_x;
                        let pos_x = sensor.pos.0 + delta_x * direction_x;
                        let pos_y = sensor.pos.1 + delta_y * direction_y;
                        yield (pos_x, pos_y);
                    }
                }
            }
            return (-1, -1);
        };
        while let GeneratorState::Yielded(pos) = Pin::new(&mut generator).resume(()) {
            if not_in_range(pos, size) {
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
                return (pos.0 * 4000000 + pos.1) as u64;
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
        assert_eq!(12413999391794, part2(&input));
    }
}
