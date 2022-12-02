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
        (1, false) => Some(days::day01::part1),
        (1, true) => Some(days::day01::part2),
        (2, false) => Some(days::day02::part1),
        (2, true) => Some(days::day02::part2),
        (3, false) => Some(days::day03::part1),
        (3, true) => Some(days::day03::part2),
        (4, false) => Some(days::day04::part1),
        (4, true) => Some(days::day04::part2),
        (5, false) => Some(days::day05::part1),
        (5, true) => Some(days::day05::part2),
        (6, false) => Some(days::day06::part1),
        (6, true) => Some(days::day06::part2),
        (7, false) => Some(days::day07::part1),
        (7, true) => Some(days::day07::part2),
        (8, false) => Some(days::day08::part1),
        (8, true) => Some(days::day08::part2),
        (9, false) => Some(days::day09::part1),
        (9, true) => Some(days::day09::part2),
        (10, false) => Some(days::day10::part1),
        (10, true) => Some(days::day10::part2),
        (11, false) => Some(days::day11::part1),
        (11, true) => Some(days::day11::part2),
        (12, false) => Some(days::day12::part1),
        (12, true) => Some(days::day12::part2),
        (13, false) => Some(days::day13::part1),
        (13, true) => Some(days::day13::part2),
        (14, false) => Some(days::day14::part1),
        (14, true) => Some(days::day14::part2),
        (15, false) => Some(days::day15::part1),
        (15, true) => Some(days::day15::part2),
        (16, false) => Some(days::day16::part1),
        (16, true) => Some(days::day16::part2),
        (17, false) => Some(days::day17::part1),
        (17, true) => Some(days::day17::part2),
        (18, false) => Some(days::day18::part1),
        (18, true) => Some(days::day18::part2),
        (19, false) => Some(days::day19::part1),
        (19, true) => Some(days::day19::part2),
        (20, false) => Some(days::day20::part1),
        (20, true) => Some(days::day20::part2),
        (21, false) => Some(days::day21::part1),
        (21, true) => Some(days::day21::part2),
        (22, false) => Some(days::day22::part1),
        (22, true) => Some(days::day22::part2),
        (23, false) => Some(days::day23::part1),
        (23, true) => Some(days::day23::part2),
        (24, false) => Some(days::day24::part1),
        (24, true) => Some(days::day24::part2),
        _ => None,
    }
}
