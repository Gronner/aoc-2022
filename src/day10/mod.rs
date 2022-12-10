use std::{str::FromStr, num::ParseIntError};

use aoc_downloader::download_day;

const DAY: u32 = 10;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

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
            Ok(Instruction::Noop)
        } else {
            Ok(add_re.captures(s).map(|captured| {
                Instruction::Addx(captured[1].parse::<i64>().unwrap())
            })
            .unwrap())
        }
    }
}

impl Instruction {
    pub fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
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
}

enum Symbol {
    Pixel(char),
    Eol(char),
}

impl Cpu {
    pub fn execute_instruction(&mut self, op: &Instruction) {
        for _ in 0..op.cycles() {
            self.clock_count += 1;
            if self.clock_count == self.checkpoint {
                self.checkpoints.push((self.clock_count, self.register));
                self.checkpoint += 40;
            }
        }
        if let Instruction::Addx(val) = op { self.register += val }
    }

    fn pointer_in_sprite(&self) -> bool {
        ((self.register - 1)..=(self.register + 1)).contains(&(self.clock_count - 1))
    }

    pub fn draw_instruction(&mut self, op: &Instruction) -> Vec<Symbol> {
        let mut ret = vec![];
        for _ in 0..op.cycles() {
            self.clock_count += 1;
            let pixel = if self.pointer_in_sprite() {
                '#'
            } else {
                '.'
            };
            if self.clock_count == self.checkpoint {
                self.clock_count = 0;
                ret.push(Symbol::Eol(pixel))
            } else {
                ret.push(Symbol::Pixel(pixel))
            };
        }
        if let Instruction::Addx(val) = op { self.register += val }
        ret
    }
}

fn part1(input: &[Input]) -> Output {
    let mut cpu = Cpu { register: 1, clock_count: 0, checkpoint: 20, checkpoints: vec![] };

    for op in input {
        cpu.execute_instruction(op);
    }
    cpu.checkpoints.iter().map(|(c, r)| c * r).sum()
}

fn part2(input: &[Input]) -> String {
    let mut cpu = Cpu { register: 1, clock_count: 0, checkpoint: 40, checkpoints: vec![] };

    let mut screen = String::from("\n");

    for op in input {
        for symbol in cpu.draw_instruction(op) {
            match symbol {
                Symbol::Pixel(px) => screen.push(px),
                Symbol::Eol(px) => {
                    screen.push(px);
                    screen.push('\n');
                }
            }
        }
    }
    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(12520, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        let expected = "
####.#..#.###..####.###....##..##..#....
#....#..#.#..#....#.#..#....#.#..#.#....
###..####.#..#...#..#..#....#.#....#....
#....#..#.###...#...###.....#.#.##.#....
#....#..#.#....#....#....#..#.#..#.#....
####.#..#.#....####.#.....##...###.####.
";

        assert_eq!(expected, part2(&input));
    }
}
