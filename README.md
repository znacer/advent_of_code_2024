# Advent of Code 2024 Solutions

## Overview

This repository contains my solutions for the Advent of Code 2024 challenge, implemented in Rust. The project is structured to provide a flexible and extensible approach to solving daily puzzles.

## Project Structure

- `src/main.rs`: The main entry point of the application
```bash
dayX # solution for day X challenge
 ├── example.txt # exemple provided
 ├── mod.rs # code solution
 └── puzzle.txt # input provided
```

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Running the Project

### Run a Specific Day's Solutions

```bash
cargo run day <day-number>
```

For example, to run Day 1 solutions:

```bash
cargo run -- day 1
```

This will output:
- Part 1 example solution
- Part 1 puzzle solution
- Part 2 example solution
- Part 2 puzzle solution

## Project Design

The project uses a generic `DaySolutions` trait that each day's module must implement:

```rust
pub trait DaySolutions {
    fn part1(&self, puzzle: Puzzle) -> String;
    fn part2(&self, puzzle: Puzzle) -> String;
}
```

The `Puzzle` enum allows differentiation between example and actual puzzle inputs.

## Adding New Days

To add solutions for a new day:
1. Create a new module in `src/` (e.g., `day3/mod.rs`)
2. Implement the `DaySolutions` trait for the new day
3. Add the new day to the `get_day()` function in `main.rs`

## Disclaimer

These solutions are part of my personal Advent of Code 2024 journey. They may not be the most optimized or elegant solutions, but represent my approach to solving the daily challenges.
