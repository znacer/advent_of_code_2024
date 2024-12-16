use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    fs, usize,
};

use itertools::Itertools;

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;

fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day16/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day16/puzzle.txt").unwrap(),
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

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
impl Direction {
    fn to_str(&self) -> &str {
        match self {
            Direction::N => "^",
            Direction::S => "v",
            Direction::E => ">",
            Direction::W => "<",
        }
    }
}

type Pos = (usize, usize, Direction);
#[derive(Debug, Clone)]
struct Maze {
    nodes: HashMap<Pos, HashSet<Pos>>,
    nodes_weight: HashMap<Pos, usize>,
    position: Pos,
    target: (usize, usize),
}
impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position: ({}, {}), Facing: {}",
            self.position.0, self.position.1, self.position.2
        )
    }
}

impl Maze {
    fn parse_pb(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let mut nodes: HashMap<Pos, HashSet<Pos>> = HashMap::new();
        let mut nodes_weight: HashMap<Pos, usize> = HashMap::new();
        let mut position = (0, 0, Direction::E);
        let mut target = (0, 0);
        for i in 1..(map.len() - 1) {
            for j in 1..(map.first().unwrap().len() - 1) {
                match map[i][j] {
                    'S' => position = (i, j, Direction::E),
                    'E' => target = (i, j),
                    '.' => (),
                    _ => continue,
                };
                // Check all directions
                for dir in [Direction::N, Direction::S, Direction::E, Direction::W] {
                    let mut neighbors: HashSet<Pos> = HashSet::new();
                    let (x, y) = match dir {
                        Direction::N => (i - 1, j),
                        Direction::S => (i + 1, j),
                        Direction::E => (i, j + 1),
                        Direction::W => (i, j - 1),
                    };
                    // Check if the neighboring cell is within bounds and is a '.'
                    if map[x][y] == '.' || map[x][y] == 'E' {
                        // Add the neighboring node to the set of neighbors
                        neighbors.insert((x, y, dir));
                    }
                    // Add the neighbors of the current node to the set of neighbors
                    for dir2 in [Direction::N, Direction::S, Direction::E, Direction::W] {
                        if dir2 != dir {
                            neighbors.insert((i, j, dir2));
                        }
                    }
                    nodes.insert((i, j, dir), neighbors);
                    nodes_weight.insert((i, j, dir), usize::MAX);
                }
            }
        }

        Maze {
            nodes,
            nodes_weight,
            target,
            position,
        }
    }
}
impl Maze {
    fn bfs(&mut self) -> HashSet<(usize, usize)> {
        let current_node = self.position;
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back(current_node);
        self.nodes_weight.insert(current_node, 0);
        while let Some(v) = q.pop_front() {
            // if (v.0, v.1) == self.target {
            // return *self.nodes_weight.get(&v).unwrap();
            // }
            visited.insert((v.0, v.1));
            for node in self.nodes[&v].clone() {
                let current_cost: usize = if node.2 != v.2 {
                    1000 + self.nodes_weight[&v]
                } else {
                    1 + self.nodes_weight[&v]
                };
                if current_cost < *self.nodes_weight.get(&node).unwrap_or(&usize::MAX) {
                    self.nodes_weight.insert(node, current_cost);
                    q.push_back(node);
                }
            }
        }
        visited
    }
    fn find_best_paths(&mut self) -> Vec<Vec<(usize, usize)>> {
        let _ = self.bfs();

        let min_cost = self
            .nodes_weight
            .iter()
            .filter(|((i, j, _), _)| (*i, *j) == self.target)
            .map(|(_, w)| *w)
            .min()
            .unwrap();

        fn backtrack(
            maze: &Maze,
            current: Pos,
            path: &mut Vec<(usize, usize)>,
            best_paths: &mut Vec<Vec<(usize, usize)>>,
            min_cost: usize,
            current_cost: usize,
        ) {
            if (current.0, current.1) == maze.target && current_cost == min_cost {
                best_paths.push(path.clone());
                return;
            }

            for neighbor in maze.nodes[&current].iter() {
                let move_cost = if neighbor.2 != current.2 {
                    1000 + current_cost
                } else {
                    1 + current_cost
                };

                if let Some(&neighbor_cost) = maze.nodes_weight.get(neighbor) {
                    if neighbor_cost == move_cost {
                        path.push((neighbor.0, neighbor.1));
                        backtrack(maze, *neighbor, path, best_paths, min_cost, move_cost);
                        path.pop();
                    }
                }
            }
        }

        let mut best_paths = Vec::new();
        let start_nodes: Vec<Pos> = self
            .nodes_weight
            .iter()
            .filter(|((i, j, _), _)| (*i, *j) == (self.position.0, self.position.1))
            .map(|(pos, _)| *pos)
            .collect();

        for start in start_nodes {
            let mut current_path = vec![(start.0, start.1)];
            backtrack(self, start, &mut current_path, &mut best_paths, min_cost, 0);
        }

        best_paths
    }
}
impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let mut pb = Maze::parse_pb(&input);
        let _ = pb.bfs();
        let result: usize = pb
            .nodes_weight
            .iter()
            .filter(|((i, j, _), _)| (*i, *j) == pb.target)
            .map(|(_, w)| *w)
            .min()
            .unwrap();
        format!("{}", result)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let mut pb = Maze::parse_pb(&input);
        let best_paths = pb.find_best_paths();

        let unique_positions: HashSet<(usize, usize)> = best_paths
            .iter()
            .flat_map(|path| path.iter().cloned())
            .collect();

        let result = unique_positions.len();

        format!("{}", result)
    }
}
