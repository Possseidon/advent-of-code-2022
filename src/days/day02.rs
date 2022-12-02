pub fn part1(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let (opponent, myself) = parse_hands(line);
            let outcome = outcome(opponent, myself);
            myself.score() + outcome.score()
        })
        .sum()
}

pub fn part2(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let (opponent, outcome) = parse_hand_and_outcome(line);
            let myself = opponent.hand_to_get(outcome);
            myself.score() + outcome.score()
        })
        .sum()
}
#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Hand {
    fn score(self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn hand_to_get(&self, outcome: Outcome) -> Hand {
        match self {
            Hand::Rock => match outcome {
                Outcome::Win => Hand::Paper,
                Outcome::Lose => Hand::Scissors,
                Outcome::Draw => Hand::Rock,
            },
            Hand::Paper => match outcome {
                Outcome::Win => Hand::Scissors,
                Outcome::Lose => Hand::Rock,
                Outcome::Draw => Hand::Paper,
            },
            Hand::Scissors => match outcome {
                Outcome::Win => Hand::Rock,
                Outcome::Lose => Hand::Paper,
                Outcome::Draw => Hand::Scissors,
            },
        }
    }
}

impl Outcome {
    fn score(self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

fn outcome(opponent: Hand, myself: Hand) -> Outcome {
    match opponent {
        Hand::Rock => match myself {
            Hand::Rock => Outcome::Draw,
            Hand::Paper => Outcome::Win,
            Hand::Scissors => Outcome::Lose,
        },
        Hand::Paper => match myself {
            Hand::Rock => Outcome::Lose,
            Hand::Paper => Outcome::Draw,
            Hand::Scissors => Outcome::Win,
        },
        Hand::Scissors => match myself {
            Hand::Rock => Outcome::Win,
            Hand::Paper => Outcome::Lose,
            Hand::Scissors => Outcome::Draw,
        },
    }
}

fn parse_hands(line: &str) -> (Hand, Hand) {
    let bytes = line.as_bytes();
    (
        match bytes[0] {
            b'A' => Hand::Rock,
            b'B' => Hand::Paper,
            b'C' => Hand::Scissors,
            _ => panic!(),
        },
        match bytes[2] {
            b'X' => Hand::Rock,
            b'Y' => Hand::Paper,
            b'Z' => Hand::Scissors,
            _ => panic!(),
        },
    )
}

fn parse_hand_and_outcome(line: &str) -> (Hand, Outcome) {
    let bytes = line.as_bytes();
    (
        match bytes[0] {
            b'A' => Hand::Rock,
            b'B' => Hand::Paper,
            b'C' => Hand::Scissors,
            _ => panic!(),
        },
        match bytes[2] {
            b'X' => Outcome::Lose,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            _ => panic!(),
        },
    )
}
