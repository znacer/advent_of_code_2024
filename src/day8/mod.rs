use std::fs;

use itertools::Itertools;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day8/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day8/puzzle.txt").unwrap(),
    };
    content
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    i: isize,
    j: isize,
}
impl std::ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}
impl std::ops::Sub<Pos> for Pos {
    type Output = Pos;
    fn sub(self, rhs: Pos) -> Self::Output {
        Pos {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
        }
    }
}
impl Pos {
    fn new(i: isize, j: isize) -> Self {
        Self { i, j }
    }
}
#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<char>>,
    length: isize,
    width: isize,
}
impl Map {
    fn parse_pb(input: &str) -> Self {
        let mut map: Vec<Vec<char>> = Vec::new();
        input
            .lines()
            .for_each(|line| map.push(line.chars().collect_vec()));

        let length = map.len().try_into().unwrap();
        let width = map.first().unwrap().len().try_into().unwrap();
        Self { map, length, width }
    }

    fn isin(&self, a: &Pos) -> Option<Pos> {
        if a.i < self.length && a.j < self.width && a.i >= 0 && a.j >= 0 {
            return Some(*a);
        }
        None
    }
    fn antinode(&self, a: &Pos, b: &Pos) -> (Option<Pos>, Option<Pos>) {
        let delta = *b - *a;
        let anti_a = *a - delta;
        let anti_a = self.isin(&anti_a);
        let anti_b = *b + delta;
        let anti_b = self.isin(&anti_b);
        (anti_a, anti_b)
    }
    fn antinode2(&self, a: &Pos, b: &Pos) -> Vec<Pos> {
        let delta = *b - *a;
        let mut antinodes = vec![];
        let mut anti_a = *a;
        loop {
            match self.isin(&anti_a) {
                Some(pos) => antinodes.push(pos),
                None => {
                    break;
                }
            };
            anti_a = anti_a - delta;
        }
        let mut anti_b = *b;
        loop {
            match self.isin(&anti_b) {
                Some(pos) => antinodes.push(pos),
                None => {
                    break;
                }
            };
            anti_b = anti_b + delta;
        }
        antinodes
    }
    fn antenas_pos(&self, c: char) -> Vec<Pos> {
        self.map
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_, val)| **val == c)
            .map(|(pos, val)| {
                let pos: isize = pos.try_into().unwrap();
                let i = pos / self.width;
                let j = pos % self.width;
                (Pos::new(i, j), val)
            })
            .map(|(pos, _)| pos)
            .collect()
    }
    fn wave_list(&self) -> Vec<char> {
        self.map
            .iter()
            .flatten()
            .filter(|val| **val != '.')
            .unique()
            .map(|&val| val)
            .collect()
    }
    fn antinodes_of_wave(&self, c: char) -> Vec<Pos> {
        let antenas = self.antenas_pos(c);
        let mut antinodes: Vec<Pos> = Vec::new();
        for a in antenas.iter() {
            for b in antenas.iter() {
                if *a == *b {
                    continue;
                }
                let (ant_a, ant_b) = self.antinode(a, b);
                for ant in [ant_a, ant_b].iter() {
                    match ant {
                        Some(pos) => {
                            if !antinodes.contains(pos) {
                                antinodes.push(*pos)
                            }
                        }
                        None => (),
                    };
                }
            }
        }
        antinodes
    }
    fn antinodes_of_wave2(&self, c: char) -> Vec<Pos> {
        let antenas = self.antenas_pos(c);
        let mut antinodes: Vec<Pos> = Vec::new();
        for a in antenas.iter() {
            for b in antenas.iter() {
                if *a == *b {
                    continue;
                }
                let ants = self.antinode2(a, b);
                for pos in ants.iter() {
                    if !antinodes.contains(pos) {
                        antinodes.push(*pos)
                    }
                }
            }
        }
        antinodes
    }
    fn all_antinodes(&self) -> Vec<Pos> {
        let mut antinodes: Vec<Pos> = Vec::new();
        let waves = self.wave_list();
        for wave in waves.iter() {
            let wave_antinodes = self.antinodes_of_wave(*wave);
            for antinode in wave_antinodes.iter() {
                if !antinodes.contains(antinode) {
                    antinodes.push(*antinode)
                }
            }
        }
        antinodes
    }
    fn all_antinodes2(&self) -> Vec<Pos> {
        let mut antinodes: Vec<Pos> = Vec::new();
        let waves = self.wave_list();
        for wave in waves.iter() {
            let wave_antinodes = self.antinodes_of_wave2(*wave);
            for antinode in wave_antinodes.iter() {
                if !antinodes.contains(antinode) {
                    antinodes.push(*antinode)
                }
            }
        }
        antinodes
    }
}

impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let pb = Map::parse_pb(&input);
        format!("{}", pb.all_antinodes().len())
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let pb = Map::parse_pb(&input);
        format!("{}", pb.all_antinodes2().len())
    }
}
