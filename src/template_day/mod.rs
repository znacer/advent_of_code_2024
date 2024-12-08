use std::fs;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        //TODO: change the directory name
        Puzzle::Example => fs::read_to_string("src/dayX/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/dayX/puzzle.txt").unwrap(),
    };
    content
}

fn parse_pb(input: &str) -> Vec<&str> {
    todo!()
}


impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        if puzzle == Puzzle::Puzzle {
            unimplemented!()
        }
        let input = load_problem(puzzle);
        let mut result = 0;
        format!("{:?}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        if puzzle == Puzzle::Puzzle {
            unimplemented!()
        }
        let input = load_problem(puzzle);
        let pb = parse_pb(&input);
        let mut result = 0;
        format!("{:?}", result)
    }
}
