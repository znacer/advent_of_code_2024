use core::panic;
use std::fs;

use itertools::Itertools;
use radix_fmt::radix;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day7/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day7/puzzle.txt").unwrap(),
    };
    content
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum Op {
    #[default]
    Sum,
    Prod,
    Concat,
}
#[derive(Debug, Clone)]
struct Equation {
    result: i64,
    terms: Vec<i64>,
    operators: Vec<Op>,
}
fn parse_pb(input: &str) -> Vec<Equation> {
    let mut pb: Vec<Equation> = vec![];
    for line in input.lines() {
        let (result, terms) = line.split_once(": ").unwrap();
        let result = result.parse().unwrap();
        let terms = terms
            .split(" ")
            .map(|elt| elt.parse().unwrap())
            .collect_vec();
        let operators = (0..(terms.len() - 1)).map(|_| Op::default()).collect_vec();
        pb.push(Equation {
            result,
            terms,
            operators,
        });
    }
    pb
}

impl Equation {
    fn compute(&self) -> i64 {
        let mut r = self.terms[0];
        for i in 0..self.operators.len() {
            r = match self.operators[i] {
                Op::Sum => r + self.terms[i + 1],
                Op::Prod => r * self.terms[i + 1],
                Op::Concat => format!("{}{}", r, self.terms[i + 1]).parse().unwrap(),
            };
        }
        r
    }

    fn next_operator(&self) -> Option<Vec<Op>> {
        // compute the next operation to do.
        // The idea is to associate + => 0 and * => 1,
        // our operators vector is then binary number and we add 1
        if self.operators.iter().all(|e| *e == Op::Prod) {
            return None;
        }

        let op_encode: String = self
            .operators
            .iter()
            .map(|op| match *op {
                Op::Sum => '0',
                Op::Prod => '1',
                _ => panic!("no concat expected here"),
            })
            .collect();
        let op_encode = 1 + u32::from_str_radix(&op_encode, 2).unwrap();
        let pad_size = self.operators.len();

        let op_decode = format!("{op_encode:0width$b}", width = pad_size);
        Some(
            op_decode
                .chars()
                .map(|c| match c {
                    '0' => Op::Sum,
                    '1' => Op::Prod,
                    something_else => panic!("expected 0 or 1, got {:?}", something_else),
                })
                .collect(),
        )
    }
    fn next_operator2(&self) -> Option<Vec<Op>> {
        // compute the next operation to do.
        // The idea is to associate + => 0 and * => 1 and || => 2,
        // our operators vector is then binary number and we add 1
        if self.operators.iter().all(|e| *e == Op::Concat) {
            return None;
        }

        let op_encode: String = self
            .operators
            .iter()
            .map(|op| match *op {
                Op::Sum => '0',
                Op::Prod => '1',
                Op::Concat => '2',
            })
            .collect();
        let op_encode = 1 + u32::from_str_radix(&op_encode, 3).unwrap();
        let pad_size = self.operators.len();

        let op_decode: i64 = format!("{}", radix(op_encode, 3)).parse().unwrap();
        let op_decode = format!("{op_decode:0width$}", width = pad_size);
        Some(
            op_decode
                .chars()
                .map(|c| match c {
                    '0' => Op::Sum,
                    '1' => Op::Prod,
                    '2' => Op::Concat,
                    something_else => panic!("expected 0 or 1 or 2, got {:?}", something_else),
                })
                .collect(),
        )
    }

    fn check_equation(&mut self) -> Option<Vec<Op>> {
        loop {
            let result = self.compute();
            if result == self.result {
                return Some(self.operators.clone());
            }
            if let Some(next_op) = self.next_operator() {
                self.operators = next_op;
            } else {
                return None;
            }
        }
    }
    fn check_equation2(&mut self) -> Option<Vec<Op>> {
        loop {
            let result = self.compute();
            if result == self.result {
                return Some(self.operators.clone());
            }
            if let Some(next_op) = self.next_operator2() {
                self.operators = next_op;
            } else {
                return None;
            }
        }
    }
}

impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let pb = parse_pb(&input);
        let mut result = 0;
        for eq in pb {
            match eq.clone().check_equation() {
                Some(_) => result += eq.result,
                None => (),
            }
        }

        format!("{:?}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let pb = parse_pb(&input);
        let mut result = 0;
        for eq in pb {
            match eq.clone().check_equation2() {
                Some(_) => result += eq.result,
                None => (),
            }
        }

        format!("{:?}", result)
    }
}
