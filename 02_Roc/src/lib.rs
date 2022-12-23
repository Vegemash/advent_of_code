use std::{str::FromStr, string::ParseError};

pub fn process_part1(input: String) -> String {
    input
        .split("\n")
        .map(|game| part_1_evaluate(game.to_string()))
        .sum::<u32>()
        .to_string()
}
pub fn process_part2(input: String) -> String {
    input
        .split("\n")
        .map(|game| part_2_evaluate(game.to_string()))
        .sum::<u32>()
        .to_string()
}

enum RPS {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl FromStr for Outcome {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        match &s[0..1] {
            "X" => return Ok(Outcome::Loss),
            "Y" => return Ok(Outcome::Draw),
            "Z" => return Ok(Outcome::Win),
            _ => panic!("{}", &s[0..1]),
        }
    }
}

impl FromStr for RPS {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        match &s[0..1] {
            "A" | "X" => return Ok(RPS::Rock),
            "B" | "Y" => return Ok(RPS::Paper),
            "C" | "Z" => return Ok(RPS::Scissors),
            _ => panic!("{}", &s[0..1]),
        }
    }
}

fn fight(you: &RPS, me: &RPS) -> Outcome {
    match me {
        RPS::Rock => match you {
            RPS::Rock => Outcome::Draw,
            RPS::Paper => Outcome::Loss,
            RPS::Scissors => Outcome::Win,
        },
        RPS::Paper => match you {
            RPS::Rock => Outcome::Win,
            RPS::Paper => Outcome::Draw,
            RPS::Scissors => Outcome::Loss,
        },
        RPS::Scissors => match you {
            RPS::Rock => Outcome::Loss,
            RPS::Paper => Outcome::Win,
            RPS::Scissors => Outcome::Draw,
        },
    }
}
fn inverse_fight(you: &RPS, outcome: &Outcome) -> RPS {
    match you {
        RPS::Rock => match outcome {
            Outcome::Win => RPS::Paper,
            Outcome::Loss => RPS::Scissors,
            Outcome::Draw => RPS::Rock,
        },
        RPS::Paper => match outcome {
            Outcome::Win => RPS::Scissors,
            Outcome::Loss => RPS::Rock,
            Outcome::Draw => RPS::Paper,
        },
        RPS::Scissors => match outcome {
            Outcome::Win => RPS::Rock,
            Outcome::Loss => RPS::Paper,
            Outcome::Draw => RPS::Scissors,
        },
    }
}

trait Score {
    fn score(&self) -> u32;
}

impl Score for RPS {
    fn score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

fn part_1_evaluate(game: String) -> u32 {
    if game == "".to_string() {
        return 0;
    }
    let parts = game
        .split(" ")
        .map(|s| s.parse::<RPS>().unwrap())
        .take(2)
        .collect::<Vec<RPS>>();
    fight(&parts[0], &parts[1]).score() + &parts[1].score()
}
fn part_2_evaluate(game: String) -> u32 {
    if game == "".to_string() {
        return 0;
    }
    let parts = game.split(" ").take(2).collect::<Vec<&str>>();
    let you = parts[0].parse::<RPS>().unwrap();
    let outcome = parts[1].parse::<Outcome>().unwrap();
    outcome.score() + inverse_fight(&you, &outcome).score()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_to_string("data/test_input").unwrap();
        assert_eq!(process_part1(input), "15".to_string())
    }
    #[test]
    fn test_part_2() {
        let input = read_to_string("data/test_input").unwrap();
        assert_eq!(process_part2(input), "12".to_string())
    }
}
