use std::{collections::HashSet, fs};

use itertools::Itertools;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        //TODO: change the directory name
        Puzzle::Example => fs::read_to_string("src/day10/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day10/puzzle.txt").unwrap(),
    };
    content
}

fn parse_pb(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn trailheads(pb: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    pb.iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == '0')
                .map(|(j, _)| (i, j))
                .collect_vec()
        })
        .flatten()
        .collect_vec()
}

fn next_step(pb: &Vec<Vec<char>>, pos: &(usize, usize)) -> HashSet<(usize, usize)> {
    let (i, j) = *pos;
    let u_min = if i == 0 { i } else { i - 1 };
    let v_min = if j == 0 { j } else { j - 1 };
    let u_max = if i == pb.len() - 1 { i } else { i + 1 };
    let v_max = if j == pb.first().unwrap().len() - 1 {
        j
    } else {
        j + 1
    };
    let next_height = pb[i][j].to_digit(10).unwrap() + 1;
    let mut next_nodes: HashSet<(usize, usize)> = HashSet::new();
    for u in u_min..=u_max {
        for v in v_min..=v_max {
            if u == i || v == j {
                if pb[u][v].to_digit(10).unwrap() == next_height {
                    next_nodes.insert((u, v));
                }
            }
        }
    }

    next_nodes
}
fn score_trailhead(pb: &Vec<Vec<char>>, zero: &(usize, usize)) -> usize {
    let mut current_nodes = next_step(pb, zero);
    for _ in 1..9 {
        current_nodes = current_nodes
            .iter()
            .map(|node| next_step(pb, node))
            .flatten()
            .collect();
    }
    current_nodes.len()
}

fn next_step_2(pb: &Vec<Vec<char>>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let (i, j) = *pos;
    let u_min = if i == 0 { i } else { i - 1 };
    let v_min = if j == 0 { j } else { j - 1 };
    let u_max = if i == pb.len() - 1 { i } else { i + 1 };
    let v_max = if j == pb.first().unwrap().len() - 1 {
        j
    } else {
        j + 1
    };
    let next_height = pb[i][j].to_digit(10).unwrap() + 1;
    let mut next_nodes = vec![];
    for u in u_min..=u_max {
        for v in v_min..=v_max {
            if u == i || v == j {
                if pb[u][v].to_digit(10).unwrap() == next_height {
                    next_nodes.push((u, v));
                }
            }
        }
    }

    next_nodes
}
fn score_trailhead_2(pb: &Vec<Vec<char>>, zero: &(usize, usize)) -> usize {
    let mut current_nodes = next_step_2(pb, zero);
    for _ in 1..9 {
        current_nodes = current_nodes
            .iter()
            .map(|node| next_step_2(pb, node))
            .flatten()
            .collect();
    }
    current_nodes.len()
}
impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let pb = parse_pb(&input);
        let zeros = trailheads(&pb);
        let result: usize = zeros.iter().map(|z| score_trailhead(&pb, z)).sum();
        format!("{:?}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let pb = parse_pb(&input);
        let zeros = trailheads(&pb);
        let result: usize = zeros.iter().map(|z| score_trailhead_2(&pb, z)).sum();
        format!("{:?}", result)
    }
}
