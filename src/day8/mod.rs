use std::collections::{HashSet, HashMap};

use aoc_downloader::download_day;

const DAY: u32 = 8;

fn get_input() -> Vec<String> {
    use std::io::BufRead;
    download_day((DAY) as u32, "input").unwrap();

    let file = std::fs::File::open(format!("input/input{}.txt", DAY)).unwrap();
    let reader = std::io::BufReader::new(&file);
    reader.lines().collect::<Result<_, _>>().unwrap()
}

type Input = Vec<i32>;
type Output = u32;

fn parse_input(input: Vec<String>) -> Vec<Input> {
    input
        .iter()
        .map(|line| line.chars().map(|v| v.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
        .collect::<Vec<_>>()
}

pub fn run_day() {
    let input = get_input();
    let input = parse_input(input);
    println!("Running day {}:\n\tPart1 {}\n\tPart2 {}", DAY, part1(&input), part2(&input));
}

fn part1(input: &[Input]) -> Output {
    let mut visible_trees = 0;

    let mut already_seen = HashSet::new();

    for (r, tree_row_we) in input.iter().enumerate() {
        let mut max_tree = -1;
        for (c, tree) in tree_row_we.iter().enumerate() {
            if *tree > max_tree {
                max_tree = *tree;
                if !already_seen.contains(&(r, c)) {
                    visible_trees += 1;
                    already_seen.insert((r, c));
                }
            }
        }
    }

    for (r, tree_row_ew) in input.iter().enumerate() {
        let mut max_tree = -1;
        for (c, tree) in tree_row_ew.iter().enumerate().rev() {
            if *tree > max_tree {
                max_tree = *tree;
                if !already_seen.contains(&(r, c)) {
                    visible_trees += 1;
                    already_seen.insert((r, c));
                }
            }
        }
    }

    for tree_column_ns in 0..input[0].len() {
        let mut max_tree = -1;
        for tree in 0..input.len() {
            if input[tree][tree_column_ns] > max_tree {
                max_tree = input[tree][tree_column_ns];
                if !already_seen.contains(&(tree, tree_column_ns)) {
                    visible_trees += 1;
                    already_seen.insert((tree, tree_column_ns));
                }
            }
        }
    }

    for tree_column_sn in (0..input[0].len()) {
        let mut max_tree = -1;
        for tree in (0..input.len()).rev() {
            if input[tree][tree_column_sn] > max_tree {
                max_tree = input[tree][tree_column_sn];
                if !already_seen.contains(&(tree, tree_column_sn)) {
                    visible_trees += 1;
                    already_seen.insert((tree, tree_column_sn));
                }
            }
        }
    }

    visible_trees
}

fn part2(input: &[Input]) -> Output {
    let mut visibility =  HashMap::new();
    let max_r = input.len();
    //let max_c = input[0].len();

    for (r, row) in input.iter().enumerate() {
        for (c, tree) in row.iter().enumerate() {
            // Left
            let mut left_vis = 0;
            for other_tree in row[0..c].iter().rev() {
                left_vis += 1;
                if other_tree < tree {
                } else {
                    break;
                }
            }
            // Right
            let mut righ_vis = 0;
            for other_tree in row[c+1..].iter() {
                righ_vis += 1;
                if other_tree < tree {
                } else {
                    break;
                }
            }
            // Up
            let mut top_vis = 0;
            for other_tree_r in (0..r).rev() {
                top_vis += 1;
                if input[other_tree_r][c] < *tree {
                } else {
                    break;
                }
            }
            // Down
            let mut bot_vis = 0;
            for other_tree_r in r+1..max_r {
                bot_vis += 1;
                if input[other_tree_r][c] < *tree {
                } else {
                    break;
                }
            }

            visibility.insert((r, c), left_vis * righ_vis * top_vis * bot_vis);
        }
    }

    *visibility.values()
        .max()
        .unwrap()
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
