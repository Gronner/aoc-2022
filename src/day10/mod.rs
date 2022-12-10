use std::{str::FromStr, num::ParseIntError};

use aoc_downloader::download_day;

const DAY: u32 = 10;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Instruction;
type Output = i64;

enum Instruction {
    Addx(i64),
    Noop,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let noop_re = regex!(r"noop");
        let add_re = regex!(r"addx (-?\d+)");
        
        if noop_re.is_match(s) {
            return Ok(Instruction::Noop);
        } else {
            Ok(add_re.captures(s).map(|captured| {
                Instruction::Addx(captured[1].parse::<i64>().unwrap())
            })
            .unwrap())
        }
    }
}

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|v| Instruction::from_str(v).unwrap())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

struct Cpu {
    pub register: i64,
    pub clock_count: i64,
    pub checkpoint: i64,
    pub checkpoints: Vec<(i64, i64)>,
    pub sprite_position: isize,
}

impl Cpu {
    pub fn execute_instruction(&mut self, op: &Instruction) {
        match op {
            Instruction::Noop => {
                self.clock_count += 1;
                if self.clock_count == self.checkpoint {
                    self.checkpoints.push((self.clock_count, self.register));
                    self.checkpoint += 40;
                }
            },
            Instruction::Addx(val) => {
                for _ in 0..2 {
                    self.clock_count += 1;
                    if self.clock_count == self.checkpoint {
                        self.checkpoints.push((self.clock_count, self.register));
                        self.checkpoint += 40;
                    }
                }
                self.register += val;
            }
        }
    }

    pub fn draw_instruction(&mut self, op: &Instruction) {
        match op {
            Instruction::Noop => {
                self.clock_count += 1;
                let pixel = if self.sprite_position - 1 <= ((self.clock_count % 40) - 1) as isize 
                    && ((self.clock_count % 40) - 1) as isize <= self.sprite_position + 1 {
                    '#'
                } else {
                    '.'
                };
                if self.clock_count == self.checkpoint {
                    println!("{}", pixel);
                    self.checkpoint += 40;
                } else {
                    print!("{}", pixel);
                }
                // println!("c: {}, r: {}", self.clock_count, self.sprite_position);
            },
            Instruction::Addx(val) => {
                for _ in 0..2 {
                    self.clock_count += 1;
                    // println!("c: {}, r: {}", self.clock_count, self.sprite_position);
                    let pixel = if self.sprite_position - 1 <= ((self.clock_count % 40) - 1) as isize 
                        && ((self.clock_count % 40) - 1) as isize <= self.sprite_position + 1 {
                        '#'
                    } else {
                        '.'
                    };
                    if self.clock_count == self.checkpoint {
                        println!("{}", pixel);
                        self.checkpoint += 40;
                    } else {
                        print!("{}", pixel);
                    }
                }
                self.register += val;
                self.sprite_position = self.register as isize;
            }
        }
    }
}

fn part1(input: &[Input]) -> Output {
    let mut cpu = Cpu { register: 1, clock_count: 0, checkpoint: 20, checkpoints: vec![], sprite_position: 0 };

    for op in input {
        cpu.execute_instruction(op);
    }
    println!("{:?}", cpu.checkpoints);
    cpu.checkpoints.iter().map(|(c, r)| c * r).sum()
}

fn part2(input: &[Input]) -> Output {
    let mut cpu = Cpu { register: 1, clock_count: 0, checkpoint: 40, checkpoints: vec![], sprite_position: 1};

    println!("\n");
    for op in input {
        cpu.draw_instruction(op);
    }
    println!("\n");
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
