pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
use std::{collections::HashMap, sync::Arc};

use clap::{Parser, Subcommand};
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;

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
    days.insert(1, Arc::new(Day1));
    days.insert(2, Arc::new(Day2));
    days.insert(3, Arc::new(Day3));
    days.insert(4, Arc::new(Day4));
    days.insert(5, Arc::new(Day5));
    days.insert(6, Arc::new(Day6));

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
