use std::fs;

use itertools::Itertools;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        //TODO: change the directory name
        Puzzle::Example => fs::read_to_string("src/day9/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day9/puzzle.txt").unwrap(),
    };
    content
}

fn parse_pb(input: &str) -> Vec<usize> {
    input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec()
}
fn expand(file: Vec<usize>) -> Vec<Option<usize>> {
    let mut expanded = vec![];
    for (i, size) in file.iter().enumerate() {
        let data = if i % 2 == 1 { None } else { Some(i / 2) };
        for _ in 0..*size {
            expanded.push(data);
        }
    }
    expanded
}
fn swap(expanded: &mut Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut i;
    let mut j;
    loop {
        j = expanded.len()
            - 1
            - expanded
                .iter()
                .rev()
                .find_position(|e| e.is_some())
                .unwrap()
                .0;

        i = expanded.iter().find_position(|e| e.is_none()).unwrap().0;
        if i > j {
            break;
        }
        expanded.swap(i, j);
    }
    expanded.to_vec()
}

fn swap2(expanded: &mut Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut file_positions: Vec<(usize, std::ops::Range<usize>)> = vec![];
    let mut current_id = None;
    let mut start = 0;

    for (i, block) in expanded.iter().enumerate() {
        if block != &current_id {
            if let Some(id) = current_id {
                file_positions.push((id, start..i));
            }
            current_id = *block;
            start = i;
        }
    }
    if let Some(id) = current_id {
        file_positions.push((id, start..expanded.len()));
    }

    file_positions.sort_by(|a, b| b.0.cmp(&a.0));

    for (file_id, range) in file_positions {
        let file_size = range.len();

        if let Some(start_idx) = expanded
            .windows(file_size)
            .take(range.start)
            .position(|window| window.iter().all(|block| block.is_none()))
        {
            for i in 0..file_size {
                expanded[start_idx + i] = Some(file_id);
                expanded[range.start + i] = None;
            }
        }
    }

    expanded.to_vec()
}

impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        if puzzle == Puzzle::Puzzle {
            return "".to_string();
        }
        let input = load_problem(puzzle);
        let pb = parse_pb(&input);
        let mut expanded = expand(pb);
        let result: usize = swap(&mut expanded)
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_some())
            .map(|(i, v)| i * v.unwrap())
            .sum();
        format!("{:?}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let pb = parse_pb(&input);
        let mut expanded = expand(pb);
        let result: usize = swap2(&mut expanded)
            .iter()
            .enumerate()
            .filter(|(_, v)| v.is_some())
            .map(|(i, v)| i * v.unwrap())
            .sum();
        format!("{:?}", result)
    }
}
