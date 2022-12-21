use std::str::FromStr;
use std::num::ParseIntError;
use rayon::prelude::*;


use aoc_downloader::download_day;

const DAY: u32 = 19;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Blueprint;
type Output = u64;

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|v| Blueprint::from_str(v).unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

#[derive(Debug)]
struct Blueprint {
    id: u64,
    ore: Resources,
    clay: Resources,
    obsidian: Resources,
    geode: Resources,
}

impl FromStr for Blueprint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"Blueprint (\d+): Each ore robot costs (\d) ore. Each clay robot costs (\d) ore. Each obsidian robot costs (\d) ore and (\d+) clay. Each geode robot costs (\d) ore and (\d+) obsidian.");
        Ok(re.captures(s).map(|captured|
                Blueprint {
                    id: captured[1].parse::<u64>().unwrap(),
                    ore: Resources {
                        ore: captured[2].parse::<u64>().unwrap(),
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                    clay: Resources {
                        ore: captured[3].parse::<u64>().unwrap(),
                        clay: 0,
                        obsidian: 0,
                        geode: 0,
                    },
                    obsidian: Resources {
                        ore: captured[4].parse::<u64>().unwrap(),
                        clay: captured[5].parse::<u64>().unwrap(),
                        obsidian: 0,
                        geode: 0,
                    },
                    geode: Resources {
                        ore: captured[6].parse::<u64>().unwrap(),
                        clay: 0,
                        obsidian: captured[7].parse::<u64>().unwrap(),
                        geode: 0,
                    },
                }
        ).unwrap())
        
    }
}

#[derive(Clone, Copy, Debug)]
struct Resources {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
}

#[derive(Clone, Copy, Debug)]
struct Robots {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geode: u64,
}

fn mine(blueprint: &Blueprint, mut time: u64, mut res: Resources, rob: Robots, best_so_far: u64) -> u64 {
    if time == 0 {
        return res.geode;
    }

    // Check wether with our current robots and by creating a robot per minute we could beat our
    // best. This could be improved, as we probably can not create a robot each minute!
    if res.geode + time * rob.geode + time * (time - 1) / 2 <= best_so_far {
        return 0;
    }

    // Have enough ore to build and we have enough robots to get all the ore for the most expensive
    // robot in one cycle
    let new_ore = res.ore >= blueprint.ore.ore 
        && rob.ore < blueprint.clay.ore.max(blueprint.obsidian.ore).max(blueprint.geode.ore);
    let new_clay = res.ore >= blueprint.clay.ore;
    let new_obsidian = res.ore >= blueprint.obsidian.ore && res.clay >= blueprint.obsidian.clay;
    let new_geode = res.ore >= blueprint.geode.ore && res.obsidian >= blueprint.geode.obsidian;

    time -= 1;
    res.ore += rob.ore;
    res.clay += rob.clay;
    res.obsidian += rob.obsidian;
    res.geode += rob.geode;

    let mut max = best_so_far;
    if new_geode {
        let mut new_res = res;
        new_res.ore -= blueprint.geode.ore;
        new_res.obsidian -= blueprint.geode.obsidian;
        let mut new_rob = rob;
        new_rob.geode += 1;
        max = max.max(mine(blueprint, time, new_res, new_rob, best_so_far));
    } 
    if new_obsidian {
        let mut new_res = res;
        new_res.ore -= blueprint.obsidian.ore;
        new_res.clay -= blueprint.obsidian.clay;
        let mut new_rob = rob;
        new_rob.obsidian += 1;
        max = max.max(mine(blueprint, time, new_res, new_rob, best_so_far));
    } 
    if new_clay {
        let mut new_res = res;
        new_res.ore -= blueprint.clay.ore;
        let mut new_rob = rob;
        new_rob.clay += 1;
        max = max.max(mine(blueprint, time, new_res, new_rob, best_so_far));
    } 
    if new_ore {
        let mut new_res = res;
        new_res.ore -= blueprint.ore.ore;
        let mut new_rob = rob;
        new_rob.ore += 1;
        max = max.max(mine(blueprint, time, new_res, new_rob, best_so_far));
    } 
    max = max.max(mine(blueprint, time, res, rob, max));
    max 
}

fn part1(input: &[Input]) -> Output {
    input.par_iter()
        .map(|blueprint| {
            let res = Resources {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            };
            let robs = Robots {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            };
            mine(blueprint, 24, res, robs, 0) * blueprint.id
        })
        .sum()
}

fn part2(input: &[Input]) -> Output {
    input[0..3].par_iter().map(|blueprint| {
        let res = Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        let robs = Robots {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geode: 0,
        };
        mine(blueprint, 32, res, robs, 0)
    })
    .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(1147, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(3080, part2(&input));
    }
}
