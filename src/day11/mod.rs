use aoc_downloader::download_day;

const DAY: u32 = 11;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Monkey;
type Output = u64;

fn parse_input(_input: Vec<String>) -> Vec<Input> {
    let common = 7 * 19 * 5 * 11 * 17 * 13 * 2 * 3;
    vec![
        Monkey::new(vec![57, 58], 7, m0, (2, 3), common),
        Monkey::new(vec![66, 52, 59, 79, 94, 73], 19, m1, (4, 6), common),
        Monkey::new(vec![80], 5, m2, (7, 5), common),
        Monkey::new(vec![82, 81, 68, 66, 71, 83, 75, 97], 11, m3, (5, 2), common),
        Monkey::new(vec![55, 52, 67, 70, 69, 94, 90], 17, m4, (0, 3), common),
        Monkey::new(vec![69, 85, 89, 91], 13, m5, (1, 7), common),
        Monkey::new(vec![75, 53, 73, 52, 75], 2, m6, (0, 4), common),
        Monkey::new(vec![94, 60, 79], 3, m7, (1, 6), common),
    ]
    /*
    let common = 23 * 19 * 13 * 17;
    vec![
        Monkey::new(vec![79, 98], 23, t0, (2, 3), common),
        Monkey::new(vec![54, 65, 75, 74], 19, t1, (2, 0), common),
        Monkey::new(vec![79, 60, 97], 13, t2, (1, 3), common),
        Monkey::new(vec![74], 17, t3, (0, 1), common),
    ]
    */
}

type Op = fn(Item) -> Item;

fn m0(lhs: Item) -> Item {
    lhs * 19
}

fn m1(lhs: Item) -> Item {
    lhs + 1
}

fn m2(lhs: Item) -> Item {
    lhs + 6
}

fn m3(lhs: Item) -> Item {
    lhs + 5
}

fn m4(lhs: Item) -> Item {
    lhs * lhs
}

fn m5(lhs: Item) -> Item {
    lhs + 7
}

fn m6(lhs: Item) -> Item {
    lhs * 7
}

fn m7(lhs: Item) -> Item {
    lhs + 2
}

#[allow(dead_code)]
fn t0(lhs: Item) -> Item {
    lhs * 19
}

#[allow(dead_code)]
fn t1(lhs: Item) -> Item {
    lhs + 6
}

#[allow(dead_code)]
fn t2(lhs: Item) -> Item {
    lhs * lhs
}

#[allow(dead_code)]
fn t3(lhs: Item) -> Item {
    lhs + 3
}

type Item = u64;
type MonkeyId = usize;
type Throw = (MonkeyId, Item);

#[derive(Clone)]
struct Monkey {
    items: Vec<Item>,
    test_val: Item,
    inspect_it: Op,
    targets: (MonkeyId, MonkeyId),
    pub inspections: u64,
    common: Item,
}

impl Monkey {
    pub fn new(starting_items: Vec<Item>, test_val: Item, inspect: Op, targets: (MonkeyId, MonkeyId), common: Item) -> Self {
        Monkey {
            items: starting_items,
            test_val,
            inspect_it: inspect,
            targets,
            inspections: 0,
            common
        }
    }

    fn checkout(&self, item: Item) -> Item {
        (self.inspect_it)(item)
    }

    fn unworry(&self, item: Item, worry: Item) -> Item {
        let item = item / worry;
        item % self.common
    }

    fn test(&self, item: Item) -> bool {
        item % self.test_val == 0
    }

    pub fn turn(&mut self, worry_factor: Item) -> Vec<Throw> {
        let mut throws = vec![];
        while let Some(mut item) = self.items.pop() {
            item = self.checkout(item);
            item = self.unworry(item, worry_factor);
            let target = if self.test(item) {
                self.targets.0
            } else {
                self.targets.1
            };
            throws.push((target, item));
            self.inspections += 1;
        }
        throws
    }

    pub fn catch(&mut self, new_item: Item) {
        self.items.push(new_item)
    }
}


pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn follow_the_monkey(input: &[Input], round: usize, worry_factor: Item) -> Vec<Input> {
    let mut input = input.to_owned();
    for _ in 0..round {
        for i in 0..input.len() {
            let air = input[i].turn(worry_factor);
            for throw in air {
                input[throw.0].catch(throw.1);
            }
        }
    }

    input.sort_by(|a, b| a.inspections.cmp(&b.inspections));
    input
}

fn score(input: &[Input]) -> Output {
    input.iter()
        .rev()
        .take(2)
        .map(|m| m.inspections)
        .product()
}

fn part1(input: &[Input]) -> Output {
    score(&follow_the_monkey(input, 20, 3))
}

fn part2(input: &[Input]) -> Output {
    score(&follow_the_monkey(input, 10_000, 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(50830, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(14399640002, part2(&input));
    }
}
