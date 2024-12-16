use core::panic;
use std::{
    fmt::{Debug, Display},
    fs,
};

use itertools::Itertools;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day15/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day15/puzzle.txt").unwrap(),
    };
    content
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Robot,
    Wall,
    Food,
    Empty,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
impl Element {
    fn to_str(&self) -> &str {
        match self {
            Element::Robot => "@",
            Element::Wall => "#",
            Element::Food => "O",
            Element::Empty => ".",
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    length: usize,
    map: Vec<Vec<Element>>,
    moves: Vec<char>,
    robot: (usize, usize),
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self
            .map
            .iter()
            .map(|line| line.iter().map(|elt| elt.to_str()).join(""))
            .join("\n");
        write!(
            f,
            "{}\n\nNext moves: {}",
            output,
            match self.moves.last() {
                Some(a) => {
                    a.to_string()
                }
                None => {
                    "None".to_string()
                }
            }
        )
    }
}

impl Grid {
    fn parse_pb(input: &str) -> Self {
        if let Some((map_str, moves_str)) = input.split_once("\n\n") {
            let mut robot = (0, 0);
            let map = map_str
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(j, c)| match c {
                            '#' => Element::Wall,
                            '@' => {
                                robot = (i, j);
                                Element::Robot
                            }
                            'O' => Element::Food,
                            '.' => Element::Empty,
                            _ => panic!(),
                        })
                        .collect_vec()
                })
                .collect_vec();
            let length = map_str.lines().count();
            let width = map_str.lines().next().unwrap().len();

            let moves = moves_str.chars().rev().filter(|c| *c != '\n').collect_vec();

            return Grid {
                length,
                width,
                map,
                moves,
                robot,
            };
        } else {
            panic!("error parsing")
        }
    }

    fn next_move(&mut self) {
        let to_move = match self.moves.pop() {
            Some(c) => c,
            None => return (),
        };

        let (i, j) = self.robot;
        let move_direction = match to_move {
            '<' => |u: usize, v: usize| (u, v - 1),
            '>' => |u: usize, v: usize| (u, v + 1),
            '^' => |u: usize, v: usize| (u - 1, v),
            'v' => |u: usize, v: usize| (u + 1, v),
            cc => panic!("expected one of < v > ^, got {cc}"),
        };
        let (mut ni, mut nj) = move_direction(i, j);
        // println!("{}", self);
        // dbg!(&self.map[i][j]);
        // dbg!(&self.map[ni][nj]);
        match self.map[ni][nj] {
            Element::Wall => {
                return;
            }
            Element::Robot => panic!("unexpected Element"),
            Element::Empty => {
                self.map[i][j] = Element::Empty;
                self.map[ni][nj] = Element::Robot;
                self.robot = (ni, nj);
                return;
            }
            Element::Food => (),
        };
        // now we have food
        // what's behind food ? if empty, everything must move, wall, nothing moves, if food again,
        // loop

        loop {
            (ni, nj) = move_direction(ni, nj);
            match self.map[ni][nj] {
                Element::Wall => {
                    //if wall nothing moves
                    return;
                }
                Element::Robot => panic!("unexpected Element"),
                Element::Empty => {
                    self.map[i][j] = Element::Empty;
                    self.map[ni][nj] = Element::Food;
                    let (u, v) = move_direction(i, j);
                    self.map[u][v] = Element::Robot;
                    self.robot = (u, v);
                    return;
                }
                Element::Food => (),
            };
        }
    }
}

impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let mut pb = Grid::parse_pb(&input);
        loop {
            if pb.moves.is_empty() {
                break;
            }
            pb.next_move();
        }
        let result: usize = pb
            .map
            .iter()
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, elt)| **elt == Element::Food)
                    .map(|(j, _)| 100 * i + j)
                    .sum::<usize>()
            })
            .sum();
        format!("{}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        // if puzzle == Puzzle::Puzzle {
        unimplemented!()
        // }
        // let input = load_problem(puzzle);
        // let pb = parse_pb(&input);
        // let mut result = 0;
        // format!("{:?}", result)
    }
}
