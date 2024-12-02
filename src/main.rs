pub mod day1;
pub mod day2;
use std::{collections::HashMap, sync::Arc};

use clap::{Parser, Subcommand};
use day1::Day1;
use day2::Day2;

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
