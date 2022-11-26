use aoc_2022;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if 2 == args.len() {
        aoc_2022::get_days()[args[1].parse::<usize>().unwrap()]();
    } else {
        for call in aoc_2022::get_days() {
            call();
        }
    }
}
