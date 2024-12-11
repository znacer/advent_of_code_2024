use memoize::memoize;
use std::fs;

use itertools::Itertools;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day11/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day11/puzzle.txt").unwrap(),
    };
    content
}

fn parse_pb(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|nb| nb.parse().unwrap())
        .collect_vec()
}

fn apply_rule(stone: &i64) -> Vec<i64> {
    if *stone == 0 {
        return vec![1];
    } else if (((*stone + 1) as f64).log10().ceil() as i64) % 2 == 0 {
        let stone_string = stone.to_string();
        let (lhs, rhs) = stone_string.split_at(stone_string.len() / 2);
        let lhs = lhs.parse().unwrap();
        let rhs = rhs.parse().unwrap();
        return vec![lhs, rhs];
    } else {
        return vec![2024 * stone];
    }
}

fn apply_rule_n_times(stone: &i64, n: usize) -> Vec<i64> {
    let mut stones = vec![*stone];
    (0..n).for_each(|_| {
        stones = stones
            .iter()
            .flat_map(|stone| apply_rule(stone))
            .collect_vec();
    });
    stones
}

#[memoize]
fn reccursive_rules(stone: i64, blinks: u8) -> usize {
    if blinks == 0 {
        1
    } else if stone == 0 {
        reccursive_rules(1, blinks - 1)
    } else {
        let digits = ((stone + 1) as f64).log10().ceil() as u32;
        if digits % 2 == 0 {
            let half_div = 10i64.pow(digits / 2);
            reccursive_rules(stone / half_div, blinks - 1)
                + reccursive_rules(stone % half_div, blinks - 1)
        } else {
            reccursive_rules(stone * 2024, blinks - 1)
        }
    }
}

impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let mut stones = parse_pb(&input);
        stones = stones
            .iter()
            .flat_map(|stone| apply_rule_n_times(stone, 25))
            .collect_vec();
        let result = stones.len();
        format!("{:?}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        if puzzle == Puzzle::Example {
            return format!("nothing for today");
        }
        let input = load_problem(puzzle);
        let stones = parse_pb(&input);
        let result: usize = stones
            .iter()
            .map(|stone| reccursive_rules(*stone, 75))
            .sum();
        format!("{:?}", result)
    }
}
