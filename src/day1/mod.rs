use itertools::Itertools;
use std::{collections::HashMap, fs};

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day1/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day1/puzzle.txt").unwrap(),
    };
    content
}

fn read_lists(pb: String) -> (Vec<i32>, Vec<i32>) {
    let mut l_list: Vec<i32> = vec![];
    let mut r_list: Vec<i32> = vec![];
    for line in pb.lines() {
        let [l_ele, r_ele] = &line.split_whitespace().collect_vec()[..2] else {
            return (vec![], vec![]);
        };

        l_list.push(l_ele.parse().unwrap());
        r_list.push(r_ele.parse().unwrap());
    }
    (l_list, r_list)
}

impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let pb_string = load_problem(puzzle);
        let (mut l_list, mut r_list) = read_lists(pb_string);

        r_list.sort();
        l_list.sort();
        let mut delta_list: Vec<i32> = vec![];
        for i in 0..r_list.len() {
            delta_list.push((r_list[i] - l_list[i]).abs());
        }
        let result: i32 = delta_list.iter().sum();
        format!("{result}")
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let pb_string = load_problem(puzzle);
        let (l_list, r_list) = read_lists(pb_string);
        let mut counter: HashMap<i32, i32> = l_list.iter().map(|e| (*e, 0i32)).collect();
        for e in r_list.iter() {
            counter.entry(*e).and_modify(|k| *k += 1);
        }
        let mut result: i32 = 0;
        for k in l_list.iter() {
            if let Some(v) = counter.get(k) {
                result += k * v
            }
        }
        format!("{result}")
    }
}
