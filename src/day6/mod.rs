use core::panic;
use std::{collections::HashSet, fmt::Display, fs, ops};

use itertools::Itertools;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day6/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day6/puzzle.txt").unwrap(),
    };
    content
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}
#[derive(Debug, Clone, Copy)]
enum Cell {
    Guard(Direction),
    Empty,
    Visited,
    Obstacle,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}
impl ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Guard {
    position: Pos,
    direction: Direction,
}
impl Guard {
    fn new(position: Pos, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<Cell>>,
    max_x: isize,
    max_y: isize,
    guard: Guard,
    visited: HashSet<Guard>,
    counter: i32,
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .map
            .iter()
            .map(|line| {
                line.iter()
                    .map(|v| match v {
                        Cell::Guard(direction) => match direction {
                            Direction::N => "^",
                            Direction::S => "v",
                            Direction::E => ">",
                            Direction::W => "<",
                        },
                        Cell::Empty => ".",
                        Cell::Visited => "X",
                        Cell::Obstacle => "#",
                    })
                    .join("")
            })
            .join("\n");
        write!(f, "{}", &out)
    }
}
impl Map {
    fn parse_problem(input: &str) -> Self {
        let mut map = vec![];
        let mut guard_position = Pos { x: 0, y: 0 };
        let mut guard_direction = None;
        for (x, line) in input.lines().enumerate() {
            map.push(
                line.chars()
                    .enumerate()
                    .map(|(y, c)| match c {
                        '.' => Cell::Empty,
                        '#' => Cell::Obstacle,
                        'X' => Cell::Visited,
                        _ => {
                            guard_position = Pos {
                                x: x.try_into().unwrap(),
                                y: y.try_into().unwrap(),
                            };
                            if c == '^' {
                                guard_direction = Some(Direction::N);
                            } else if c == 'v' {
                                guard_direction = Some(Direction::S);
                            } else if c == '>' {
                                guard_direction = Some(Direction::E);
                            } else if c == '<' {
                                guard_direction = Some(Direction::W);
                            } else {
                                panic!("wrong charcter parsed: {:?}", c)
                            }
                            Cell::Guard(guard_direction.unwrap())
                        }
                    })
                    .collect_vec(),
            )
        }
        let max_x = (map.len() - 1).try_into().unwrap();
        let max_y = (map.last().unwrap().len() - 1).try_into().unwrap();
        Map {
            map,
            max_x,
            max_y,
            visited: HashSet::new(),
            counter: 0,
            guard: Guard::new(guard_position, guard_direction.unwrap()),
        }
    }
    fn get(&self, pos: Pos) -> Cell {
        self.map[pos.x as usize][pos.y as usize]
    }
    fn get_guard_direction(&self) -> Direction {
        match self.get(self.guard.position) {
            Cell::Guard(dir) => dir,
            _ => panic!("there is no guard at this position"),
        }
    }
    fn map_edit(&mut self, pos: Pos, new_value: Cell) {
        self.map[pos.x as usize][pos.y as usize] = new_value;
    }
    fn step(&mut self) -> Option<bool> {
        let direction = self.get_guard_direction();

        let next_position = match direction {
            Direction::N => self.guard.position + Pos::new(-1, 0),
            Direction::S => self.guard.position + Pos::new(1, 0),
            Direction::E => self.guard.position + Pos::new(0, 1),
            Direction::W => self.guard.position + Pos::new(0, -1),
        };

        if next_position.x < 0
            || next_position.x > self.max_x
            || next_position.y < 0
            || next_position.y > self.max_y
        {
            self.counter += 1;
            return Some(false);
        }
        match self.get(next_position) {
            Cell::Empty => {
                self.map_edit(next_position, Cell::Guard(direction));
                self.map_edit(self.guard.position, Cell::Visited);
                self.guard.position = next_position;
                self.counter += 1;
                self.visited.insert(self.guard.clone());
                Some(true)
            }
            Cell::Visited => {
                self.map_edit(next_position, Cell::Guard(direction));
                self.map_edit(self.guard.position, Cell::Visited);
                self.guard.position = next_position;
                if self.visited.contains(&self.guard) {
                    return None;
                }
                self.visited.insert(self.guard.clone());
                Some(true)
            }
            Cell::Obstacle => {
                self.guard.direction = match direction {
                    Direction::N => Direction::E,
                    Direction::E => Direction::S,
                    Direction::S => Direction::W,
                    Direction::W => Direction::N,
                };
                self.map_edit(self.guard.position, Cell::Guard(self.guard.direction));
                Some(true)
            }
            a => panic!("{:?}", a),
        }
    }
    fn play(&mut self) -> i32 {
        loop {
            match self.step() {
                Some(true) => (),
                Some(false) => return self.counter,
                None => return -1,
            };
        }
    }
}
impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let mut pb = Map::parse_problem(&input);
        format!("\n{}", pb.play())
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let mut result = 0;
        let pb = Map::parse_problem(&input);
        // Could highly be improved by adding the obsutruction only on the path of the guard...
        // Any way it happened to work on my tiny computer in not so much time
        for i in 0..(pb.max_x + 1) {
            for j in 0..(pb.max_y + 1) {
                match pb.get(Pos::new(i, j)) {
                    Cell::Empty => {
                        let mut n_pb = pb.clone();
                        n_pb.map_edit(Pos::new(i, j), Cell::Obstacle);
                        let play = n_pb.play();
                        if play < 0 {
                            result += 1;
                        }
                    }
                    _ => (),
                }
            }
        }
        format!("{}", result)
    }
}
