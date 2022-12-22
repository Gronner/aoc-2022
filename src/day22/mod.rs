use aoc_downloader::download_day;
use std::{collections::HashMap, str::FromStr, num::ParseIntError};

const DAY: u32 = 22;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = u64;
type Output = i64;

type Coords = (i64, i64);

#[derive(Debug)]
enum Command {
    Direction(char),
    Movement(i64),
}

fn from_str(line: &str) -> Vec<Command> {
    let mut buf = String::new();
    let mut comms = vec![];

    for c in line.chars() {
        if c.is_digit(10) {
            buf.push(c);
            continue;
        } else {
            comms.push(Command::Movement(buf.parse::<i64>().unwrap()));
            buf.clear();
            comms.push(Command::Direction(c));
        }
    }
    comms.push(Command::Movement(buf.parse::<i64>().unwrap()));

    comms
}

fn parse_input(input: Vec<String>) -> (HashMap<Coords, char>, Vec<Command>) {
    let mut map = HashMap::new();
    let mut commands = vec![];
    let mut path = false;
    let mut y = 1;
    for line in input {
        if line.is_empty() {
            path = true;
            continue;
        }
        if path {
            commands = from_str(&line);
            continue;
        }
        let mut x = 1;
        println!("{line}");
        for c in line.chars() {
            if c == '.' || c == '#' {
                map.insert((x, y), c);
            }
            x += 1;
        }
        y += 1;
    }

    (map, commands)
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

#[derive(Debug)]
struct Me {
    pub pos: Coords,
    pub head: i64,
}

impl Me {
    pub fn get_score(&self) -> i64 {
        1000 * self.pos.1 + 4 * self.pos.0 + self.head
    }
}

fn walk_map(start: Coords, map: &HashMap<Coords, char>, comms: &Vec<Command>) -> Me {
    let mut me = Me {
        pos: start,
        head: 0,
    };

    for comm in comms {
        match comm {
            Command::Movement(length) => {
                let offset = match me.head {
                    0 => (1, 0),
                    1 => (0, 1),
                    2 => (-1, 0),
                    3 => (0, -1),
                    e => panic!("Unkown heading: {}", e),
                };
                for _ in 1..=*length {
                    if let Some(field) = map.get(&(me.pos.0 + offset.0, me.pos.1 + offset.1)) {
                        if *field == '.' {
                            me.pos = (me.pos.0 + offset.0, me.pos.1 + offset.1);
                        } else {
                            break;
                        }
                    } else {
                        match me.head {
                            0 => {
                                let row_min = map
                                    .keys()
                                    .filter(|(_, y)| *y == me.pos.1)
                                    .map(|(x, _)| *x)
                                    .min()
                                    .unwrap();
                                let next_pos = (row_min, me.pos.1);
                                if let Some(field) = map.get(&next_pos) {
                                    if *field == '.' {
                                        me.pos = next_pos;
                                    } else {
                                        break;
                                    }
                                }
                            },
                            1 => {
                                let col_min = map
                                    .keys()
                                    .filter(|(x, _)| *x == me.pos.0)
                                    .map(|(_, y)| *y)
                                    .min()
                                    .unwrap();
                                let next_pos = (me.pos.0, col_min);
                                if let Some(field) = map.get(&next_pos) {
                                    if *field == '.' {
                                        me.pos = next_pos;
                                    } else {
                                        break;
                                    }
                                }
                            },
                            2 => {
                                let row_max = map
                                    .keys()
                                    .filter(|(_, y)| *y == me.pos.1)
                                    .map(|(x, _)| *x)
                                    .max()
                                    .unwrap();
                                let next_pos = (row_max, me.pos.1);
                                if let Some(field) = map.get(&next_pos) {
                                    if *field == '.' {
                                        me.pos = next_pos;
                                    } else {
                                        break;
                                    }
                                }
                            },
                            3 => {
                                let col_max = map
                                    .keys()
                                    .filter(|(x, _)| *x == me.pos.0)
                                    .map(|(_, y)| *y)
                                    .max()
                                    .unwrap();
                                let next_pos = (me.pos.0, col_max);
                                if let Some(field) = map.get(&next_pos) {
                                    if *field == '.' {
                                        me.pos = next_pos;
                                    } else {
                                        break;
                                    }
                                }
                            },
                            e => panic!("Unkown heading: {}", e),
                        }
                    }
                    println!("{:?}", me);
                }
            },
            Command::Direction(turn) => {
                if *turn == 'R' {
                    me.head = (me.head + 1) % 4;
                } if *turn == 'L' {
                    me.head = if (me.head - 1) == -1 { 3 } else { me.head - 1};
                }
                println!("{:?}", me);
            },
        }
    }

    me
}

fn part1(input: &(HashMap<Coords, char>, Vec<Command>)) -> Output {
    let map = input.0.clone();
    println!("{:?}", map);
    let comms = &input.1;
    let upper_left_x = map
        .keys()
        .filter(|(_, y)| *y == 1)
        .map(|(x, _)| x)
        .min()
        .unwrap();
    walk_map((*upper_left_x, 1), &map, &comms).get_score()
}

fn part2(input: &(HashMap<Coords, char>, Vec<Command>)) -> Output {
    0
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
