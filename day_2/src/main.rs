#![allow(unused, dead_code)]

use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();
    let rounds = input
        .split("\n")
        .map(|i| i.to_string())
        .map(Round::try_from)
        .map(|r| r.unwrap())
        .collect::<Vec<Round>>();

    part_two(rounds);
}

// fn part_one(input: Vec<Round>) {
//     let total_score = input
//         .into_iter()
//         .fold(0, |a, i| a + i.points());
//
//     println!("score: {total_score}");
// }

fn part_two(input: Vec<Round>) {
    let total_score = input
        .into_iter()
        .fold(0, |a, i| a + i.points());

    println!("score: {total_score}");
}

struct Round(Choice, Outcome);
impl Round {
    fn points(&self) -> usize {
        self.1.points()
        + self.1.expected_outcome(&self.0).points()
    }
}
impl TryFrom<String> for Round {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let items = value
            .split(" ")
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        let choice = items[0].to_owned().try_into().unwrap();
        let outcome = items[1].to_owned().try_into().unwrap();

        Ok(
            Self(choice, outcome)
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}
impl Choice {
    fn points(&self) -> usize {
        match self {
            Self::ROCK     => 1,
            Self::PAPER    => 2,
            Self::SCISSORS => 3,
        }
    }
}
impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        match (self, other) {
            (Self::ROCK, Self::PAPER)     => Some(Ordering::Less),
            (Self::ROCK, Self::SCISSORS)  => Some(Ordering::Greater),
            (Self::PAPER, Self::ROCK)     => Some(Ordering::Greater),
            (Self::PAPER, Self::SCISSORS) => Some(Ordering::Less),
            (Self::SCISSORS, Self::ROCK)  => Some(Ordering::Less),
            (Self::SCISSORS, Self::PAPER) => Some(Ordering::Greater),

            _ => unreachable!()
        }
    }
}
impl TryFrom<String> for Choice {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "A" => Ok(Self::ROCK),
            "B" => Ok(Self::PAPER),
            "C" => Ok(Self::SCISSORS),
            _ => Err(format!("unknown choice {value}"))
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Outcome {
    WIN,
    DRAW,
    LOSS,
}
impl Outcome {
    fn expected_outcome(&self, opponent: &Choice) -> Choice {
        match (opponent, self) {
            (Choice::ROCK, Self::LOSS)     => Choice::SCISSORS,
            (Choice::ROCK, Self::DRAW)     => Choice::ROCK,
            (Choice::ROCK, Self::WIN)      => Choice::PAPER,
            (Choice::PAPER, Self::LOSS)    => Choice::ROCK,
            (Choice::PAPER, Self::DRAW)    => Choice::PAPER,
            (Choice::PAPER, Self::WIN)     => Choice::SCISSORS,
            (Choice::SCISSORS, Self::LOSS) => Choice::PAPER,
            (Choice::SCISSORS, Self::DRAW) => Choice::SCISSORS,
            (Choice::SCISSORS, Self::WIN)  => Choice::ROCK
        }
    }

    fn points(&self) -> usize {
        match self {
            Self::WIN => 6,
            Self::DRAW => 3,
            Self::LOSS => 0,
        }
    }
}
impl TryFrom<String> for Outcome {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "X" => Ok(Self::LOSS),
            "Y" => Ok(Self::DRAW),
            "Z" => Ok(Self::WIN),

            _ => Err(format!("unknown outcome {value}"))
        }
    }
}