use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day12/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day12/puzzle.txt").unwrap(),
    };
    content
}

fn parse_pb(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

#[derive(Debug, Clone)]
struct Cluster {
    plant_type: char,
    area: usize,
    perimeter: usize,
    sides: usize,
    sides_u: HashSet<(usize, usize)>,
    sides_d: HashSet<(usize, usize)>,
    sides_l: HashSet<(usize, usize)>,
    sides_r: HashSet<(usize, usize)>,
    pos: HashSet<(usize, usize)>,
}

impl Cluster {
    fn new(plant_type: char) -> Self {
        Self {
            plant_type,
            area: 0,
            perimeter: 0,
            sides: 0,
            sides_u: HashSet::new(),
            sides_d: HashSet::new(),
            sides_l: HashSet::new(),
            sides_r: HashSet::new(),
            pos: HashSet::new(),
        }
    }

    fn build_cluster(&mut self, pb: &Vec<Vec<char>>) {
        let mut revisited: HashSet<(usize, usize)> = HashSet::new();

        'outer: for _ in 0..(pb.len() * pb.first().unwrap().len()) {
            // instead to just loop
            let mut to_add = HashSet::new();
            'inner: for (i, j) in self.pos.iter() {
                if revisited.contains(&(*i, *j)) {
                    continue 'inner;
                }
                revisited.insert((*i, *j));
                for (ii, jj) in neighbors(pb, *i, *j).iter() {
                    if pb[*ii][*jj] == pb[*i][*j] {
                        to_add.insert((*ii, *jj));
                    }
                }
            }
            to_add.iter().for_each(|node| {
                self.pos.insert(*node);
            });
            if revisited.len() == self.pos.len() {
                break 'outer;
            }
        }
    }
    fn compute_perimeter(&mut self, pb: &Vec<Vec<char>>) -> usize {
        self.pos
            .iter()
            .map(|(i, j)| {
                let mut sub_count = 0;
                if *i > 0 {
                    if !self.pos.contains(&(*i - 1, *j)) {
                        sub_count += 1;
                        self.sides_u.insert((*i, *j));
                    }
                } else {
                    sub_count += 1;
                    self.sides_u.insert((*i, *j));
                }
                if *j > 0 {
                    if !self.pos.contains(&(*i, *j - 1)) {
                        sub_count += 1;
                        self.sides_l.insert((*i, *j));
                    }
                } else {
                    sub_count += 1;
                    self.sides_l.insert((*i, *j));
                }
                if *i < pb.len() - 1 {
                    if !self.pos.contains(&(*i + 1, *j)) {
                        sub_count += 1;
                        self.sides_d.insert((*i, *j));
                    }
                } else {
                    sub_count += 1;
                    self.sides_d.insert((*i, *j));
                }
                if *j < pb.first().unwrap().len() - 1 {
                    if !self.pos.contains(&(*i, *j + 1)) {
                        sub_count += 1;
                        self.sides_r.insert((*i, *j));
                    }
                } else {
                    sub_count += 1;
                    self.sides_r.insert((*i, *j));
                }
                sub_count
            })
            .sum()
    }
    fn compute_sides(&mut self, pb: &Vec<Vec<char>>) {
        let mut nb_u = 0;
        let mut nb_d = 0;
        let mut nb_l = 0;
        let mut nb_r = 0;
        //
        //u d
        for i in 0..pb.len() {
            for j in 0..pb.first().unwrap().len() {
                if self.sides_u.contains(&(i, j)) {
                    if j == 0 {
                        nb_u += 1;
                    } else {
                        if !self.sides_u.contains(&(i, j - 1)) {
                            nb_u += 1
                        }
                    }
                }
                if self.sides_d.contains(&(i, j)) {
                    if j == pb.first().unwrap().len() - 1 {
                        nb_d += 1;
                    } else {
                        if !self.sides_d.contains(&(i, j + 1)) {
                            nb_d += 1
                        }
                    }
                }
            }
        }
        for j in 0..pb.first().unwrap().len() {
            for i in 0..pb.len() {
                if self.sides_l.contains(&(i, j)) {
                    if i == 0 {
                        nb_l += 1;
                    } else {
                        if !self.sides_l.contains(&(i - 1, j)) {
                            nb_l += 1
                        }
                    }
                }
                if self.sides_r.contains(&(i, j)) {
                    if i == pb.len() - 1 {
                        nb_r += 1;
                    } else {
                        if !self.sides_r.contains(&(i + 1, j)) {
                            nb_r += 1
                        }
                    }
                }
            }
        }
        self.sides += nb_u;
        self.sides += nb_d;
        self.sides += nb_l;
        self.sides += nb_r;
    }
}

#[inline]
fn neighbors(pb: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut out = vec![];
    if i > 0 {
        out.push((i - 1, j));
    }
    if j > 0 {
        out.push((i, j - 1));
    }
    if i < pb.len() - 1 {
        out.push((i + 1, j));
    }
    if j < pb.first().unwrap().len() - 1 {
        out.push((i, j + 1));
    }
    out
}

#[inline]
fn clusters_has(clusters: &HashMap<usize, Cluster>, (i, j): (usize, usize)) -> Option<usize> {
    let cluster = clusters
        .iter()
        .map(|(k, cluster)| (k, cluster.pos.contains(&(i, j))))
        .filter(|(_, b)| *b)
        .next();
    match cluster {
        Some((k, true)) => Some(*k),
        None => None,
        _ => panic!("unexpected false with input ({i}, {j})"),
    }
}

fn compute_fences(pb: &Vec<Vec<char>>) -> HashMap<usize, Cluster> {
    let mut clusters: HashMap<usize, Cluster> = HashMap::new();

    for i in 0..pb.len() {
        'node_iter: for j in 0..pb.first().unwrap().len() {
            let plant_type = pb[i][j];
            // check if already in a cluser
            match clusters_has(&clusters, (i, j)) {
                Some(_) => {
                    continue 'node_iter;
                }
                None => (),
            };
            let kn = neighbors(pb, i, j);
            for (ii, jj) in kn.iter() {
                if pb[*ii][*jj] == plant_type {
                    match clusters_has(&clusters, (*ii, *jj)) {
                        Some(k) => {
                            clusters.entry(k).and_modify(|cluster| {
                                cluster.pos.insert((i, j));
                                cluster.area += 1;
                            });
                            continue 'node_iter;
                        }
                        None => (),
                    };
                }
            }

            clusters.insert(clusters.len(), Cluster::new(plant_type));
            clusters.entry(clusters.len() - 1).and_modify(|cluster| {
                cluster.pos.insert((i, j));
                cluster.build_cluster(pb);
            });
        }
    }

    clusters
}

impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let pb = parse_pb(&input);
        let mut plants = compute_fences(&pb);
        for (_, v) in plants.iter_mut() {
            (*v).area = v.pos.len();
            (*v).perimeter = v.compute_perimeter(&pb);
        }
        let result: usize = plants.iter().map(|(_, v)| v.perimeter * v.area).sum();
        format!("{:?}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let pb = parse_pb(&input);
        let mut plants = compute_fences(&pb);
        for (_, v) in plants.iter_mut() {
            (*v).area = v.pos.len();
            (*v).perimeter = v.compute_perimeter(&pb);
            (*v).compute_sides(&pb);
        }
        let result: usize = plants.iter().map(|(_, v)| v.area * v.sides).sum();
        format!("{:?}", result)
    }
}
