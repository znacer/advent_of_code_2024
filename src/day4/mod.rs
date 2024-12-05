use std::fs;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Day4;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day4/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day4/puzzle.txt").unwrap(),
    };
    content
}
fn parse_problem(pb: &str) -> Vec<Vec<char>> {
    let mut pb_mat: Vec<Vec<char>> = vec![];
    for elt in pb.chars() {
        if elt == '\n' {
            pb_mat.push(vec![]);
        } else {
            if let Some(v) = pb_mat.last_mut() {
                v.push(elt);
            } else {
                pb_mat.push(vec![elt]);
            }
        }
    }
    pb_mat[..(pb_mat.len() - 1)].to_vec()
}

enum Direction {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}
fn next_step(
    position: (usize, usize),
    direction: &Direction,
    pb: &Vec<Vec<char>>,
) -> Option<(usize, usize)> {
    let (i, j) = position;
    match direction {
        Direction::N => {
            if i > 0 {
                Some((i - 1, j))
            } else {
                None
            }
        }
        Direction::S => {
            if i < pb.len() - 1 {
                Some((i + 1, j))
            } else {
                None
            }
        }
        Direction::E => {
            if j < pb.last().unwrap().len() - 1 {
                Some((i, j + 1))
            } else {
                None
            }
        }
        Direction::W => {
            if j > 0 {
                Some((i, j - 1))
            } else {
                None
            }
        }
        Direction::NE => {
            if i > 0 && j < pb.last().unwrap().len() - 1 {
                Some((i - 1, j + 1))
            } else {
                None
            }
        }
        Direction::NW => {
            if i > 0 && j > 0 {
                Some((i - 1, j - 1))
            } else {
                None
            }
        }
        Direction::SE => {
            if i < pb.len() - 1 && j < pb.last().unwrap().len() - 1 {
                Some((i + 1, j + 1))
            } else {
                None
            }
        }
        Direction::SW => {
            if i < pb.len() - 1 && j > 0 {
                Some((i + 1, j - 1))
            } else {
                None
            }
        }
    }
}
fn check_xmas(position: (usize, usize), direction: &Direction, pb: &Vec<Vec<char>>) -> bool {
    let xmas = "XMAS";
    let (mut i, mut j) = position;
    for c in xmas.chars() {
        if pb[i][j] != c {
            return false;
        }
        match next_step((i, j), direction, pb) {
            Some((u, v)) => {
                (i, j) = (u, v);
            }
            None => {
                if c == 'S' {
                    return true;
                } else {
                    return false;
                }
            }
        }
    }
    true
}

fn look_for_xmas(pb: &Vec<Vec<char>>, direction: &Direction) -> i32 {
    let i_max = pb.len();
    let j_max = pb.last().unwrap().len();
    let mut counter = 0;
    for i in 0..i_max {
        for j in 0..j_max {
            if pb[i][j] == 'X' {
                if check_xmas((i, j), direction, &pb) {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn check_x_mas(position: (usize, usize), pb: &Vec<Vec<char>>) -> bool {
    let (i, j) = position;
    let top_left = pb[i - 1][j - 1];
    let top_right = pb[i - 1][j + 1];
    let bottom_left = pb[i + 1][j - 1];
    let bottom_right = pb[i + 1][j + 1];
    let corners: Vec<char> = vec![top_left, top_right, bottom_right, bottom_left];
    let nb_m = corners.iter().filter(|&c| *c == 'M').count();
    let nb_s = corners.iter().filter(|&c| *c == 'S').count();

    if nb_m == 2 && nb_s == 2 && (top_right != bottom_left) && (bottom_right != top_left) {
        return true;
    } else {
        return false;
    }
}
impl DaySolutions for Day4 {
    fn part1(&self, puzzle: Puzzle) -> String {
        let pb = load_problem(puzzle);
        let pb = parse_problem(&pb);
        let n_count = look_for_xmas(&pb, &Direction::N);
        let s_count = look_for_xmas(&pb, &Direction::S);
        let e_count = look_for_xmas(&pb, &Direction::E);
        let w_count = look_for_xmas(&pb, &Direction::W);
        let nw_count = look_for_xmas(&pb, &Direction::NW);
        let ne_count = look_for_xmas(&pb, &Direction::NE);
        let sw_count = look_for_xmas(&pb, &Direction::SW);
        let se_count = look_for_xmas(&pb, &Direction::SE);
        format!(
            "{:?}",
            n_count + s_count + e_count + w_count + nw_count + ne_count + se_count + sw_count
        )
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let pb = load_problem(puzzle);
        let pb = parse_problem(&pb);
        let mut result = 0;
        for i in 1..pb.len() - 1 {
            for j in 1..pb.last().unwrap().len() - 1 {
                if pb[i][j] == 'A' {
                    if check_x_mas((i, j), &pb) {
                        result += 1;
                    }
                }
            }
        }
        format!("{:?}", result)
    }
}
