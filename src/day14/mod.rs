use std::{fs, isize, usize};

use itertools::Itertools;
use regex::Regex;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day14/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day14/puzzle.txt").unwrap(),
    };
    content
}

#[derive(Debug, Clone, Default)]
struct Robot {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl Robot {
    fn tick_n(&mut self, n: usize) {
        let n = n as isize;
        self.x += n * self.dx;
        self.y += n * self.dy;
    }
    fn on_grid(&mut self, length: usize, width: usize) {
        let length = length as isize;
        let width = width as isize;
        self.x = self.x % length;
        self.y = self.y % width;

        if self.x < 0 {
            self.x += length;
        }
        if self.y < 0 {
            self.y += width;
        }
    }
}

fn parse_pb(input: &str) -> Vec<Robot> {
    let mut pb = vec![];
    let re = Regex::new(r"\=(-*\d+)\,(-*\d+).+=(-*\d+)\,(-*\d+)").unwrap();
    for (_, [nb1, nb2, nb3, nb4]) in re.captures_iter(input).map(|c| c.extract()) {
        let robot = Robot {
            x: nb1.parse().unwrap(),
            y: nb2.parse().unwrap(),
            dx: nb3.parse().unwrap(),
            dy: nb4.parse().unwrap(),
        };
        pb.push(robot);
    }
    pb
}

fn draw(pb: Vec<Robot>, length: usize, width: usize) -> String {
    let mut grid: Vec<Vec<isize>> = (0..width)
        .map(|_| (0..length).map(|_| 0).collect_vec())
        .collect_vec();
    for robot in pb.iter() {
        let mut m_robot = robot.clone();
        m_robot.on_grid(length, width);
        grid[m_robot.y as usize][m_robot.x as usize] += 1;
    }
    grid.iter()
        .map(|line| {
            line.iter()
                .map(|v| match v {
                    0 => ".".to_string(),
                    k => k.to_string(),
                })
                .join("|")
        })
        .join("\n")
}

impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let mut length: usize = 11;
        let mut width: usize = 7;
        if puzzle == Puzzle::Puzzle {
            length = 101;
            width = 103;
        }
        let half_length = (length / 2) as isize;
        let half_width = (width / 2) as isize;
        let input = load_problem(puzzle);
        let mut pb = parse_pb(&input);
        pb.iter_mut().for_each(|robot| {
            (*robot).tick_n(100);
            (*robot).on_grid(length, width);
        });

        let q1 = pb
            .iter()
            .filter(|robot| robot.x < half_length && robot.y < half_width)
            .count();
        let q2 = pb
            .iter()
            .filter(|robot| robot.x < half_length && robot.y > half_width)
            .count();
        let q3 = pb
            .iter()
            .filter(|robot| robot.x > half_length && robot.y < half_width)
            .count();
        let q4 = pb
            .iter()
            .filter(|robot| robot.x > half_length && robot.y > half_width)
            .count();

        let result = q1 * q2 * q3 * q4;
        format!("{}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        if puzzle == Puzzle::Example {
            return format!("irrelevant");
        }
        let length = 101;
        let width = 103;
        let input = load_problem(puzzle);
        let mut possible_iter = vec![];
        'k: for k in 0..10000 {
            let mut pb = parse_pb(&input);
            pb.iter_mut().for_each(|robot| {
                (*robot).tick_n(k);
                (*robot).on_grid(length, width);
            });
            for i in 0..length {
                for j in 0..width {
                    if pb
                        .iter()
                        .filter(|robot| robot.x == i as isize && robot.y == j as isize)
                        .count()
                        > 1
                    {
                        continue 'k;
                    }
                }
            }

            let filename = format!("output/day14/{}.txt", k);
            fs::write(filename, draw(pb, length, width)).unwrap();
            possible_iter.push(k);
        }

        format!("{}", possible_iter.iter().join(" - "))
    }
}
