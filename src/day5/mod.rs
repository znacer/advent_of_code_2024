use std::{collections::{HashMap, VecDeque}, fs};

use crate::{DaySolutions, Puzzle};

#[derive(Clone)]
pub struct Problem;


fn load_problem(puzzle: Puzzle) -> String {
    let content = match puzzle {
        Puzzle::Example => fs::read_to_string("src/day5/example.txt").unwrap(),
        Puzzle::Puzzle => fs::read_to_string("src/day5/puzzle.txt").unwrap(),
    };
    content
}

fn parse_problem(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut sections = input.split("\n\n");
    let rules_section = sections.next().unwrap();
    let updates_section = sections.next().unwrap();

    let rules = rules_section
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split('|').map(|x| x.parse::<i32>().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let updates = updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    (rules, updates)
}

fn is_update_valid(update: &[i32], rules: &[(i32, i32)]) -> bool {
    let position_map: HashMap<i32, usize> = update.iter().enumerate().map(|(i, &page)| (page, i)).collect();
    for &(before, after) in rules {
        if let (Some(&pos_before), Some(&pos_after)) = (position_map.get(&before), position_map.get(&after)) {
            if pos_before > pos_after {
                return false;
            }
        }
    }
    true
}

fn reorder_update(update: &[i32], rules: &[(i32, i32)]) -> Vec<i32> {
    // create a graph representation
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();

    for &page in update {
        graph.entry(page).or_default();
        in_degree.entry(page).or_insert(0);
    }

    // add edges
    for &(before, after) in rules {
        if update.contains(&before) && update.contains(&after) {
            graph.get_mut(&before).unwrap().push(after);
            *in_degree.entry(after).or_default() += 1;
        }
    }

    // topological sort
    let mut sorted: Vec<i32> = Vec::new();
    let mut queue: VecDeque<i32> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&page, _)| page)
        .collect();

    while let Some(page) = queue.pop_front() {
        sorted.push(page);
        if let Some(neighbors) = graph.get(&page) {
            for &neighbor in neighbors {
                let degree = in_degree.get_mut(&neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    sorted
}

impl DaySolutions for Problem {
    fn part1(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let (rules, updates) = parse_problem(&input);

        let valid_updates: Vec<Vec<i32>> = updates
            .iter()
            .filter(|update| is_update_valid(update, &rules))
            .cloned()
            .collect();

        let middle_sum: i32 = valid_updates
            .iter()
            .map(|update| update[update.len() / 2])
            .sum();

        format!("{}", middle_sum)
    }

    fn part2(&self, puzzle: Puzzle) -> String {
        let input = load_problem(puzzle);
        let (rules, updates) = parse_problem(&input);

        let invalid_updates: Vec<Vec<i32>> = updates
            .iter()
            .filter(|update| !is_update_valid(update, &rules))
            .map(|update| reorder_update(update, &rules))
            .collect();

        let middle_sum: i32 = invalid_updates
            .iter()
            .map(|update| update[update.len() / 2]) // Get the middle element
            .sum();

        format!("{}", middle_sum)
    }
}

