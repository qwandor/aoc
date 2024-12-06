use eyre::{bail, eyre, Report};
use std::{io::stdin, str::FromStr};

fn main() {
    let rounds: Vec<Round> = stdin()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    println!(
        "{}",
        rounds.into_iter().map(|round| round.score()).sum::<u32>()
    );
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn score(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn score_against(self, other: Self) -> u32 {
        match (self, other) {
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => 3, // Draw
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => 6, // We win
            (Self::Scissors, Self::Rock)
            | (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors) => 0, // Other wins
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Round {
    opponent: Play,
    me: Play,
}

impl Round {
    fn score(&self) -> u32 {
        self.me.score() + self.me.score_against(self.opponent)
    }
}

impl FromStr for Round {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let opponent = match chars
            .next()
            .ok_or_else(|| eyre!("Line too short: \"{}\"", s))?
        {
            'A' => Play::Rock,
            'B' => Play::Paper,
            'C' => Play::Scissors,
            c => bail!("Invalid opponent play {}", c),
        };

        let separator = chars
            .next()
            .ok_or_else(|| eyre!("Line too short: \"{}\"", s))?;
        if separator != ' ' {
            bail!("Invalid separator '{}'", separator);
        }

        let me = match chars
            .next()
            .ok_or_else(|| eyre!("Line too short: \"{}\"", s))?
        {
            'X' => Play::Rock,
            'Y' => Play::Paper,
            'Z' => Play::Scissors,
            c => bail!("Invalid self play {}", c),
        };

        Ok(Self { opponent, me })
    }
}
