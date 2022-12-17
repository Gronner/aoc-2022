use std::collections::HashMap;
use std::cmp::max;

use aoc_downloader::download_day;

const DAY: u32 = 16;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day(DAY, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{DAY}.txt")).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Cache;
type Output = usize;

#[derive(Clone)]
struct Cache {
    valve_to_idx: HashMap<String, usize>,
    flow_rates: HashMap<usize, usize>,
    successors: HashMap<usize, Vec<usize>>,
    cache: HashMap<(usize, u64, usize, bool), usize>
}

fn parse_input(input: Vec<String>) -> Input {
    let re = regex!(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)");
    let mut valve_to_idx = HashMap::new();
    let mut flow_rates = HashMap::new();
    let mut successors = HashMap::new();

    let mut idx = 0;
    input
        .iter()
        .for_each(|line| {
            re.captures(line).map(|captured| {
                let name = String::from(&captured[1]);
                valve_to_idx.insert(name, idx);
                flow_rates.insert(idx, captured[2].parse::<usize>().unwrap());
                idx +=1;
            });
        });
    input
        .iter()
        .for_each(|line| {
            re.captures(line).map(|captured| {
                let name = String::from(&captured[1]);
                successors.insert(valve_to_idx[&name], captured[3]
                    .split(", ")
                    .map(|v| valve_to_idx[v])
                    .collect::<Vec<_>>()
                    );
            });
        });
    Cache {
        valve_to_idx,
        flow_rates,
        successors,
        cache: HashMap::new(),
    }

}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

impl Cache {
    fn compute_max_flow(&mut self, position: usize, visited: u64, time: usize, part2: bool) -> Output {
        if time == 0 {
            if part2 {
                return self.compute_max_flow(self.valve_to_idx["AA"], visited, 26, false);
            } else {
                return 0;
            }
        }

        let cache_key = (position, visited, time, part2);
        if let Some(result) = self.cache.get(&cache_key) {
            return *result;
        }

        let mut result = 0;

        let visited_mask = 1 << position;
        let flow_rate = self.flow_rates[&position];

        let not_visited = 0 == (visited & visited_mask);
        if not_visited && flow_rate > 0 {
            let new_visited = visited | visited_mask;
            result = max(result, (time - 1) * flow_rate + self.compute_max_flow(position, new_visited, time - 1, part2));
        }

        for successor in self.successors[&position].clone() {
            result = max(result, self.compute_max_flow(successor, visited, time - 1, part2));
        }

        self.cache.insert(cache_key, result);

        result
    }
}

fn part1(input: &Input) -> Output {
    let mut cache = (*input).clone();
    let idx_aa = cache.valve_to_idx["AA"];
    cache.compute_max_flow(idx_aa, 0, 30, false)
}

fn part2(input: &Input) -> Output {
    let mut cache = (*input).clone();
    let idx_aa = cache.valve_to_idx["AA"];
    cache.compute_max_flow(idx_aa, 0, 26, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_output() {
        let input = parse_input(get_input());
        assert_eq!(1820, part1(&input));
    }

    #[test]
    fn day0_part2_output() {
        let input = parse_input(get_input());
        assert_eq!(2602, part2(&input));
    }
}
