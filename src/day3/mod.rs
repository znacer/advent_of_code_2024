use regex::Regex;
use std::fs;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day3/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day3/puzzle.txt").unwrap(),
    };
    content
}

fn read_line(line: &str) -> i32 {
    let re = Regex::new(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))").unwrap();
    let mut process = true;
    let mut result = 0;
    for (e, [_]) in re.captures_iter(line).map(|c| c.extract()) {
        if e.starts_with("do()") {
            process = true;
        } else if e.starts_with("don't") {
            process = false;
        }
        if process && e.starts_with("m") {
            result += read_mul(e);
        }
    }
    result
}
fn read_mul(elt: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result: i32 = 0;
    for (_, [nb1, nb2]) in re.captures_iter(elt).map(|c| c.extract()) {
        let a: i32 = nb1.parse().unwrap();
        let b: i32 = nb2.parse().unwrap();
        result += a * b;
    }
    result
}
impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let pb = load_problem(puzzle);
        let result = read_mul(&pb);
        format!("{:?}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let pb = match puzzle {
            Puzzle::Example => fs::read_to_string("src/day3/example2.txt").unwrap(),
            Puzzle::Puzzle => fs::read_to_string("src/day3/puzzle.txt").unwrap(),
        };
        let result = read_line(&pb);
        format!("{:?}", result)
    }
}
