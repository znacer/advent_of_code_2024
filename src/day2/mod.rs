use std::fs;

use itertools::Itertools;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Day2;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day2/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day2/puzzle.txt").unwrap(),
    };
    content
}

fn read_lists(pb: String) -> Vec<Vec<i32>> {
    let mut line_lists: Vec<Vec<i32>> = vec![];
    for line in pb.lines() {
        let sub_list: Vec<i32> = line
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        line_lists.push(sub_list);
    }
    line_lists
}

fn is_safe(line: &Vec<i32>) -> bool {
    let deltas = line
        .windows(2)
        .map(|chunck| chunck[1] - chunck[0])
        .collect_vec();
    for i in 0..deltas.len() {
        if i != 0 {
            if deltas[i] * deltas[i - 1] < 0 {
                return false;
            }
        }
        if (deltas[i]).abs() > 3 || deltas[i] == 0 {
            return false;
        }
    }
    true
}

fn is_almost_safe(line: &Vec<i32>) -> bool {
    if is_safe(line) {
        return true;
    }
    for i in 0..line.len() {
        let sub_line = line
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(_, v)| *v)
            .collect_vec();
        if is_safe(&sub_line) {
            return true;
        }
    }
    false
}

impl DaySolutions for Day2 {
    fn part1(&self, puzzle: Puzzle) -> String {
        let pb = load_problem(puzzle);
        let line_lists = read_lists(pb);
        let mut result = 0;
        for line in line_lists.iter() {
            if is_safe(&line) {
                result += 1;
            }
        }
        format!("{:?}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let pb = load_problem(puzzle);
        let line_lists = read_lists(pb);
        let mut result = 0;
        for line in line_lists.iter() {
            if is_almost_safe(&line) {
                result += 1;
            }
        }
        format!("{:?}", result)
    }
}
