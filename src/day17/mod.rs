use aoc_downloader::download_day;
use std::collections::HashSet;

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

#[allow(dead_code)]
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


fn part1(input: &[Input]) -> Output {
    create_pattern(2022, input).iter().sum()
}

fn create_pattern(rounds: usize, input: &[Input]) -> Vec<i128> {
    use Rock::*;
    let mut deltas = vec![];
    let rocks = vec![Minus, Plus, L, I, Block];
    let mut rock_gen = rocks.iter().cycle();
    let mut jet_gen = input.iter().cycle();

    let mut tetris: HashSet<(i128, i128)> = HashSet::new();
    for i in 0..7 {
        tetris.insert((i, 0));
    }

    let mut last = 0;
    let mut max_height = 0;
    for _ in 0..rounds {
        let block = rock_gen.next().unwrap();
        deltas.push(max_height - last);
        last = max_height;
        let mut shape = block.get_shape(max_height);
        loop {
            let jet = jet_gen.next().unwrap();
            let movement = match jet {
                Jet::Right => 1,
                Jet::Left => -1,
            };

            let mut shift_shape = shape
                .iter()
                .map(|(x, y)| (x + movement, *y))
                .collect::<Vec<(i128, i128)>>();

            if shift_shape.iter().any(|(x, _)| *x < 0 || *x > 6)
                || shift_shape.iter().any(|pos| tetris.contains(pos)) {
                shift_shape = shape.clone();
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
        max_height = std::cmp::max(max_height, *shape.iter().map(|(_, y)| y).max().unwrap());
    }
    

    deltas.push(max_height - last);
    deltas
}

fn part2(input: &[Input]) -> Output {
    let sample = 5000;
    let deltas = create_pattern(sample, input);
    let mut found_offset = None;
    let mut found_span = None;
    'outer: for offset in 1426..2500{
        for span in 1715..(sample/2) {
            let mut found = true;
            let pattern = &deltas[offset..(offset + span)];
            'iteration: for chunk in deltas[(offset + span)..].chunks(span) {
                if pattern != chunk {
                    found = false;
                    break 'iteration;
                }
            }
            if found {
                found_offset = Some(offset);
                found_span = Some(span);
                break 'outer;
            }
        }
    }

    let offset = found_offset.unwrap();
    let span = found_span.unwrap();
    // println!("OS: {offset}, Span: {span}");

    let rounds = 1_000_000_000_000_usize;
    let missing_rounds = (rounds - offset) % span;
    let repeats = (rounds - offset) / span;

    let start = deltas[0..offset].iter().sum::<i128>();
    let pattern_result = deltas[offset..(offset + span)].iter().sum::<i128>();
    let end = deltas[offset..=(offset + missing_rounds)].iter().sum::<i128>();


    start + pattern_result * repeats as i128 + end 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(3098, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(1525364431487, part2(&input));
    }
}
