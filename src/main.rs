mod days;

use std::fs;

use chrono::Datelike;
use clap::Parser;

/// Runs a single Advent of Code puzzle
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Which day's puzzle to run, defaults to the current day
    #[arg()]
    day: Option<u32>,

    /// Run the second part of the puzzle
    #[arg(short = '2', long)]
    part2: bool,

    /// Use the small input file
    #[arg(short, long)]
    small: bool,
}

fn main() {
    let args = Args::parse();
    let day = args.day.unwrap_or_else(|| chrono::Utc::now().day());

    let input = if args.small {
        format!("input/{day}/input_small.txt")
    } else {
        format!("input/{day}/input.txt")
    };

    let Ok(input) = fs::read_to_string(input) else {
        if args.small {
            eprintln!("Day {day} does not have a small input file!");
        } else {
            eprintln!("Day {day} does not have an input file!");
        }
        return;
    };

    let Some(solver) = get_solver(day, args.part2) else {
        eprintln!("Day {day} does not have a solver.");
        return;
    };

    let solution = solver(input);

    println!("Solution:\n{solution}");
}

type Solver = fn(String) -> u32;

fn get_solver(puzzle: u32, part2: bool) -> Option<Solver> {
    match (puzzle, part2) {
        (1, false) => Some(days::day1::part1),
        (1, true) => Some(days::day1::part2),
        _ => None,
    }
}
