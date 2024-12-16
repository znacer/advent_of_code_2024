use std::fs;

use regex::Regex;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day13/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day13/puzzle.txt").unwrap(),
    };
    content
}

#[derive(Debug, Clone, Default)]
struct SubProblem {
    a: (isize, isize),
    b: (isize, isize),
    p: (isize, isize),
    n: isize,
    m: isize,
}
impl SubProblem {
    fn compute_n_m(&mut self) {
        let n = ((self.p.0 * self.b.1) - (self.p.1 * self.b.0))
            / ((self.b.1 * self.a.0) - (self.a.1 * self.b.0));
        let m = ((self.p.0 * self.a.1) - (self.p.1 * self.a.0))
            / ((self.a.1 * self.b.0) - (self.b.1 * self.a.0));
        if n * self.a.0 + m * self.b.0 == self.p.0 && n * self.a.1 + m * self.b.1 == self.p.1 {
            self.n = n;
            self.m = m;
        }
    }
}
fn parse_pb_2(input: &str) -> Vec<SubProblem> {
    let mut pb = vec![];
    let re = Regex::new(r"[\+\=](\d+),.*[\+\=](\d+)").unwrap();
    input.split("\n\n").for_each(|spb| {
        let mut new_spb = SubProblem::default();
        for (i, (_, [nb1, nb2])) in re.captures_iter(spb).map(|c| c.extract()).enumerate() {
            let a: isize = nb1.parse().unwrap();
            let b: isize = nb2.parse().unwrap();
            if i == 0 {
                new_spb.a = (a, b)
            } else if i == 1 {
                new_spb.b = (a, b)
            } else if i == 2 {
                new_spb.p = (10000000000000 + a, 10000000000000 + b)
            }
        }
        pb.push(new_spb);
    });
    pb
}
fn parse_pb(input: &str) -> Vec<SubProblem> {
    let mut pb = vec![];
    let re = Regex::new(r"[\+\=](\d+),.*[\+\=](\d+)").unwrap();
    input.split("\n\n").for_each(|spb| {
        let mut new_spb = SubProblem::default();
        for (i, (_, [nb1, nb2])) in re.captures_iter(spb).map(|c| c.extract()).enumerate() {
            let a: isize = nb1.parse().unwrap();
            let b: isize = nb2.parse().unwrap();
            if i == 0 {
                new_spb.a = (a, b)
            } else if i == 1 {
                new_spb.b = (a, b)
            } else if i == 2 {
                new_spb.p = (a, b)
            }
        }
        pb.push(new_spb);
    });
    pb
}

impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let mut pb = parse_pb(&input);
        pb.iter_mut().for_each(|spb| (*spb).compute_n_m());
        let result: isize = pb
            .iter()
            .map(|spb| {
                let tokens = spb.n * 3 + spb.m;
                tokens
            })
            .sum();
        format!("{}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let mut pb = parse_pb_2(&input);
        pb.iter_mut().for_each(|spb| (*spb).compute_n_m());
        let result: isize = pb
            .iter()
            .map(|spb| {
                let tokens = spb.n * 3 + spb.m;
                tokens
            })
            .sum();
        format!("{}", result)
    }
}
