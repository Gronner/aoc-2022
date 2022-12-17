use aoc_downloader::download_day;
use once_cell::unsync::Lazy;
use std::collections::HashSet;
use std::pin::Pin;
use std::ops::{Generator, GeneratorState};

const DAY: u32 = 17;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Jet;
type Output = i128;

#[derive(Clone, Copy, Debug)]
enum Jet {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum Rock {
    Minus,
    Plus,
    L,
    I,
    Block,
}

impl Rock {
    fn get_shape(&self, max_height: i128) -> Vec<(i128, i128)> {
        let shape = match self {
            // (x, y)
            Self::Minus => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::Plus => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            Self::L => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Self::I => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Self::Block => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        };
        shape
            .iter()
            .map(|(x, y)| (x + 2, y + max_height + 4))
            .collect()
    }
}

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input[0]
        .chars()
        .map(|v| if v == '<' { Jet::Left } else { Jet::Right })
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn print_tetris(tetris: &HashSet<(i128, i128)>) {
    let y_max = (tetris
        .iter()
        .map(|(_, y)| *y)
        .max()
        .unwrap() + 2) as usize;

    let mut screen = vec![vec!['.'; 7]; y_max];

    for pos in tetris {
        screen[pos.1 as usize][pos.0 as usize] = '#';
    }

    for row in screen.iter().rev() {
        for pixel in row {
            print!("{pixel}");
        }
        println!();
    }
    println!();
    println!();
}


fn drop_rocks(rounds: usize, mut rock_round: usize, mut jet_round: usize, input: &[Input]) -> i128 {
    let rock_cycle: Lazy<Vec<Rock>> = Lazy::new(|| {
        use Rock::*;
        vec![Minus, Plus, L, I, Block]
        });

    let mut tetris: HashSet<(i128, i128)> = HashSet::new();
    for i in 0..7 {
        tetris.insert((i, 0));
    }

    let mut block_generator = move || {
        loop {
            rock_round = (rock_round) % (rock_cycle.len()) + 1;
            yield rock_cycle[rock_round - 1];
        }
    };

    let mut jet_generator = move || {
        loop {
            jet_round = (jet_round) % (input.len()) + 1;
            yield input[jet_round - 1];
        }
    };

    for _ in 0..rounds {
        if let GeneratorState::Yielded(block) = Pin::new(&mut block_generator).resume(()){
            let max_height = *tetris.iter()
                .map(|(_, y)| y)
                .max()
                .unwrap();
            let mut shape = block.get_shape(max_height);
            loop {
                if let GeneratorState::Yielded(jet) = Pin::new(&mut jet_generator).resume(()) {
                    let movement = match jet {
                        Jet::Right => 1,
                        Jet::Left => -1,
                    };

                    let mut shift_shape = shape
                        .iter()
                        .map(|(x, y)| (x + movement, *y))
                        .collect::<Vec<(i128, i128)>>();

                    if shift_shape.iter().any(|(x, _)| *x < 0 || *x > 6) {
                        shift_shape = shape;
                    } else if shift_shape.iter().any(|pos| tetris.contains(pos)) {
                        shift_shape = shape;
                    } else {
                    }
                    let drop_shape = shift_shape
                        .iter()
                        .map(|(x, y)| (*x, y - 1))
                        .collect::<Vec<(i128, i128)>>();

                    if drop_shape.iter().any(|pos| tetris.contains(pos)) {
                        shift_shape.iter().for_each(|pos| { tetris.insert(*pos); });
                        break;
                    }
                    shape = drop_shape;
                }
            }
        }
    }

    *tetris.iter()
        .map(|(_, y)| y)
        .max()
        .unwrap()
}

fn part1(input: &[Input]) -> Output {
    let rounds = 2022;
    drop_rocks(rounds, 0, 0, input)
}

fn create_pattern(rounds: usize, input: &[Input]) -> Vec<i128> {
    let rock_cycle: Lazy<Vec<Rock>> = Lazy::new(|| {
        use Rock::*;
        vec![Minus, Plus, L, I, Block]
        });

    let mut tetris: HashSet<(i128, i128)> = HashSet::new();
    for i in 0..7 {
        tetris.insert((i, 0));
    }

    let mut block_generator = move || {
        let mut rock_round = 0;
        loop {
            rock_round = (rock_round) % (rock_cycle.len()) + 1;
            yield rock_cycle[rock_round - 1];
        }
    };

    let mut jet_generator = move || {
        let mut jet_round = 0;
        loop {
            jet_round = (jet_round) % (input.len()) + 1;
            yield input[jet_round - 1];
        }
    };

    let mut deltas = vec![];
    let mut last = 0;
    for _ in 0..rounds {
        if let GeneratorState::Yielded(block) = Pin::new(&mut block_generator).resume(()){
            let max_height = *tetris.iter()
                .map(|(_, y)| y)
                .max()
                .unwrap();
            deltas.push(max_height - last);
            last = max_height;
            let mut shape = block.get_shape(max_height);
            loop {
                if let GeneratorState::Yielded(jet) = Pin::new(&mut jet_generator).resume(()) {
                    let movement = match jet {
                        Jet::Right => 1,
                        Jet::Left => -1,
                    };

                    let mut shift_shape = shape
                        .iter()
                        .map(|(x, y)| (x + movement, *y))
                        .collect::<Vec<(i128, i128)>>();

                    if shift_shape.iter().any(|(x, _)| *x < 0 || *x > 6) {
                        shift_shape = shape;
                    } else if shift_shape.iter().any(|pos| tetris.contains(pos)) {
                        shift_shape = shape;
                    } else {
                    }
                    let drop_shape = shift_shape
                        .iter()
                        .map(|(x, y)| (*x, y - 1))
                        .collect::<Vec<(i128, i128)>>();

                    if drop_shape.iter().any(|pos| tetris.contains(pos)) {
                        shift_shape.iter().for_each(|pos| { tetris.insert(*pos); });
                        break;
                    }
                    shape = drop_shape;
                }
            }
        }
    }

    let max_height = *tetris.iter()
        .map(|(_, y)| y)
        .max()
        .unwrap();
    deltas.push(max_height - last);
    deltas
}

fn part2(input: &[Input]) -> Output {
    println!("{}", input.len());
    let sample = 10000;
    let deltas = create_pattern(sample, input);
    let mut found_offset = None;
    let mut found_span = None;
    'outer: for offset in 0..2500 {
        for span in 2..(sample/2) {
            let mut found = true;
            let pattern = &deltas[offset..(offset + span)];
            let mut found_chunk = None;
            'iteration: for chunk in deltas[(offset + span)..].chunks(span) {
                if chunk.len() != span {
                    found = false;
                    break 'iteration;
                }
                for i in 0..pattern.len() {
                    if pattern[i] != chunk[i] {
                        found = false;
                        break 'iteration;
                    }
                }
                found_chunk = Some(chunk);
            }
            if found {
                println!("p: {pattern:?}");
                println!("c: {:?}", found_chunk.unwrap());
                assert_eq!(pattern.len(), found_chunk.unwrap().len());
                found_offset = Some(offset);
                found_span = Some(span);
                break 'outer;
            }
        }
    }

    let offset = found_offset.unwrap();
    let span = found_span.unwrap();
    println!("OS: {offset}, Span: {span}");

    let rounds = 1_000_000_000_000_usize;
    let missing_rounds = (rounds - offset) % span;
    let repeats = (rounds - offset) / span;

    let start = deltas[0..offset].iter().sum::<i128>();
    let pattern_result = deltas[offset..(offset + span)].iter().sum::<i128>();
    println!("{pattern_result}");
    let end = deltas[offset..=(offset + missing_rounds)].iter().sum::<i128>();


    start + pattern_result * repeats as i128 + end 
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
