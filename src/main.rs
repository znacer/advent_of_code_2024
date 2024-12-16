pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
use std::{collections::HashMap, sync::Arc};

use clap::{Parser, Subcommand};

#[derive(Debug, PartialEq, Eq)]
pub enum Puzzle {
    Example,
    Puzzle,
}
pub trait DaySolutions {
    fn part1(&self, puzzle: Puzzle) -> String;
    fn part2(&self, puzzle: Puzzle) -> String;
}
#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}
#[derive(Debug, Subcommand, Clone)]
enum Commands {
    Day { day: usize },
}
pub fn get_day(day: usize) -> Option<Arc<dyn DaySolutions>> {
    let mut days: HashMap<usize, Arc<dyn DaySolutions>> = HashMap::new();
    days.insert(1, Arc::new(day1::Problem));
    days.insert(2, Arc::new(day2::Problem));
    days.insert(3, Arc::new(day3::Problem));
    days.insert(4, Arc::new(day4::Problem));
    days.insert(5, Arc::new(day5::Problem));
    days.insert(6, Arc::new(day6::Problem));
    days.insert(7, Arc::new(day7::Problem));
    days.insert(8, Arc::new(day8::Problem));
    days.insert(9, Arc::new(day9::Problem));
    days.insert(10, Arc::new(day10::Problem));
    days.insert(11, Arc::new(day11::Problem));
    days.insert(12, Arc::new(day12::Problem));
    days.insert(13, Arc::new(day13::Problem));
    days.insert(14, Arc::new(day14::Problem));
    days.insert(15, Arc::new(day15::Problem));
    days.insert(16, Arc::new(day16::Problem));

    days.get(&day).cloned()
}
fn main() {
    let args = Args::parse();
    let day_value = match args.cmd {
        Commands::Day { day } => day,
    };

    if let Some(day) = get_day(day_value) {
        println!("Day {}:", day_value);
        println!("----PART 1----");
        println!("Example: {}", day.part1(Puzzle::Example));
        println!("Solution: {}", day.part1(Puzzle::Puzzle));
        println!("----PART 2----");
        println!("Example: {}", day.part2(Puzzle::Example));
        println!("Solution: {}", day.part2(Puzzle::Puzzle));
    } else {
        println!("Day {} is not implemented yet.", day_value);
    }
}
