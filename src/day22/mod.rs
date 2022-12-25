use aoc_downloader::download_day;
use std::collections::{HashMap, HashSet};

const DAY: u32 = 22;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

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
        if c.is_ascii_digit() {
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
                    e => panic!("Unkown heading: {e}"),
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
                            e => panic!("Unkown heading: {e}"),
                        }
                    }
                }
            },
            Command::Direction(turn) => {
                if *turn == 'R' {
                    me.head = (me.head + 1) % 4;
                } else if *turn == 'L' {
                    me.head = if (me.head - 1) == -1 { 3 } else { me.head - 1};
                }
            },
        }
    }

    me
}

fn part1(input: &(HashMap<Coords, char>, Vec<Command>)) -> Output {
    let map = input.0.clone();
    let comms = &input.1;
    let upper_left_x = map
        .keys()
        .filter(|(_, y)| *y == 1)
        .map(|(x, _)| x)
        .min()
        .unwrap();
    walk_map((*upper_left_x, 1), &map, comms).get_score()
}

fn walk_cube(start: Coords, map: &HashMap<Coords, char>, comms: &Vec<Command>) -> Me {
    let mut me = Me {
        pos: start,
        head: 0,
    };

    let af = (51..=100)
        .map(|x| (x, 0))
        .collect::<HashSet<Coords>>();
    let fa = (151..=200)
        .map(|y| (0, y))
        .collect::<HashSet<Coords>>();

    let ae = (1..=50)
        .map(|y| (50, y))
        .collect::<HashSet<Coords>>();
    let ea = (101..=150)
        .map(|y| (0, y))
        .collect::<HashSet<Coords>>();

    let bc = (101..=150)
        .map(|x| (x, 51))
        .collect::<HashSet<Coords>>();
    let cb = (51..=100)
        .map(|y| (101, y))
        .collect::<HashSet<Coords>>();

    let bd = (1..=50)
        .map(|y| (151, y))
        .collect::<HashSet<Coords>>();
    let db = (101..=150)
        .map(|y| (101, y))
        .collect::<HashSet<Coords>>();

    let bf = (101..=150)
        .map(|x| (x, 0))
        .collect::<HashSet<Coords>>();
    let fb= (1..=50)
        .map(|x| (x, 201))
        .collect::<HashSet<Coords>>();

    let ce = (51..=100)
        .map(|y| (50, y))
        .collect::<HashSet<Coords>>();
    let ec = (1..=50)
        .map(|x| (x, 100))
        .collect::<HashSet<Coords>>();

    let df = (51..=100)
        .map(|x| (x, 151))
        .collect::<HashSet<Coords>>();
    let fd = (151..=200)
        .map(|y| (51, y))
        .collect::<HashSet<Coords>>();


    for comm in comms {
        match comm {
            Command::Movement(length) => {
                for _ in 1..=*length {
                    let offset = match me.head {
                        0 => (1, 0),
                        1 => (0, 1),
                        2 => (-1, 0),
                        3 => (0, -1),
                        e => panic!("Unkown heading: {e}"),
                    };
                    if let Some(field) = map.get(&(me.pos.0 + offset.0, me.pos.1 + offset.1)) {
                        if *field == '.' {
                            me.pos = (me.pos.0 + offset.0, me.pos.1 + offset.1);
                        } else {
                            break;
                        }
                    } else {
                        match me.head {
                            0 => {
                                let mut next_pos = (me.pos.0 + offset.0, me.pos.1 + offset.1);
                                let heading;
                                if bd.contains(&next_pos) {
                                    let x = 100;
                                    let y = 101 + (50 - me.pos.1);
                                    heading = 2;
                                    next_pos = (x, y);
                                } else if cb.contains(&next_pos) {
                                    let x = me.pos.1 + 50;
                                    let y = 50;
                                    heading = 3;
                                    next_pos = (x, y);
                                } else if db.contains(&next_pos) {
                                    let x = 150;
                                    let y = 50 + (101 - me.pos.1);
                                    heading = 2;
                                    next_pos = (x, y);
                                } else if fd.contains(&next_pos) {
                                    let x = me.pos.1 - 100;
                                    let y = 150;
                                    heading = 3;
                                    next_pos = (x, y);
                                }  else {
                                    panic!("Unexpected Transition from {me:?} to {next_pos:?}");
                                }
                                if let Some(field) = map.get(&next_pos) {
                                    if *field == '.' {
                                        me.pos = next_pos;
                                        me.head = heading;
                                    } else {
                                        break;
                                    }
                                }
                            },
                            1 => {
                                let mut next_pos = (me.pos.0 + offset.0, me.pos.1 + offset.1);
                                let heading;
                                if bc.contains(&next_pos) {
                                    let x = 100;
                                    let y = me.pos.0 - 50;
                                    heading = 2;
                                    next_pos = (x, y);
                                } else if df.contains(&next_pos) {
                                    let x = 50;
                                    let y = me.pos.0 + 100;
                                    heading = 2;
                                    next_pos = (x, y);
                                } else if fb.contains(&next_pos) {
                                    let x = 100 + me.pos.0;
                                    let y = 1;
                                    heading = 1;
                                    next_pos = (x, y);
                                }  else {
                                    panic!("Unexpected Transition from {me:?} to {next_pos:?}");
                                }
                                if let Some(field) = map.get(&next_pos) {
                                    if *field == '.' {
                                        me.pos = next_pos;
                                        me.head = heading;
                                    } else {
                                        break;
                                    }
                                }
                            },
                            2 => {
                                let mut next_pos = (me.pos.0 + offset.0, me.pos.1 + offset.1);
                                let heading;
                                if ae.contains(&next_pos) {
                                    let x = 1;
                                    let y = 101 + (50 - me.pos.1);
                                    heading = 0;
                                    next_pos = (x, y);
                                } else if ce.contains(&next_pos) {
                                    let x = me.pos.1 - 50;
                                    let y = 101;
                                    heading = 1;
                                    next_pos = (x, y);
                                } else if ea.contains(&next_pos) {
                                    let x = 51;
                                    let y = 50 + (101 - me.pos.1);
                                    heading = 0;
                                    next_pos = (x, y);
                                } else if fa.contains(&next_pos) {
                                    let x = me.pos.1 - 100;
                                    let y = 1;
                                    heading = 1;
                                    next_pos = (x, y);
                                }  else {
                                    panic!("Unexpected Transition from {me:?} to {next_pos:?}");
                                }
                                if let Some(field) = map.get(&next_pos) {
                                    if *field == '.' {
                                        me.pos = next_pos;
                                        me.head = heading;
                                    } else {
                                        break;
                                    }
                                }
                            },
                            3 => {
                                let mut next_pos = (me.pos.0 + offset.0, me.pos.1 + offset.1);
                                let heading;
                                if af.contains(&next_pos) {
                                    let x = 1;
                                    let y = me.pos.0 + 100;
                                    heading = 0;
                                    next_pos = (x, y);
                                } else if bf.contains(&next_pos) {
                                    let x = me.pos.0 - 100;
                                    let y = 200;
                                    heading = 3;
                                    next_pos = (x, y);
                                } else if ec.contains(&next_pos) {
                                    let x = 51;
                                    let y = me.pos.0 + 50;
                                    heading = 0;
                                    next_pos = (x, y);
                                }  else {
                                    panic!("Unexpected Transition from {me:?} to {next_pos:?}");
                                }
                                if let Some(field) = map.get(&next_pos) {
                                    if *field == '.' {
                                        me.pos = next_pos;
                                        me.head = heading;
                                    } else {
                                        break;
                                    }
                                }
                            },
                            e => panic!("Unkown heading: {e}"),
                        }
                    }
                }
            },
            Command::Direction(turn) => {
                if *turn == 'R' {
                    me.head = (me.head + 1) % 4;
                } else if *turn == 'L' {
                    me.head = if (me.head - 1) == -1 { 3 } else { me.head - 1};
                }
            },
        }
    }

    me
}
fn part2(input: &(HashMap<Coords, char>, Vec<Command>)) -> Output {
    let map = input.0.clone();
    let comms = &input.1;
    let upper_left_x = map
        .keys()
        .filter(|(_, y)| *y == 1)
        .map(|(x, _)| x)
        .min()
        .unwrap();
    walk_cube((*upper_left_x, 1), &map, comms).get_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(13566, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(11451, part2(&input));
    }
}
