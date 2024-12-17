pub mod grid;

use eyre::Report;
use grid::Grid;
use std::io::BufRead;

pub fn charvec(s: &str) -> Vec<char> {
    s.chars().collect()
}

pub fn parse_chargrid(input: impl BufRead) -> Result<Grid<char>, Report> {
    input
        .lines()
        .map(|line| Ok(charvec(line?.trim())))
        .collect::<Result<Vec<_>, Report>>()?
        .try_into()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    pub fn rotate_clockwise(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn rotate_anticlockwise(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    pub fn move_from(
        self,
        position: (usize, usize),
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Self::Left => {
                if position.0 == 0 {
                    None
                } else {
                    Some((position.0 - 1, position.1))
                }
            }
            Self::Right => {
                if position.0 + 1 == width {
                    None
                } else {
                    Some((position.0 + 1, position.1))
                }
            }
            Self::Up => {
                if position.1 == 0 {
                    None
                } else {
                    Some((position.0, position.1 - 1))
                }
            }
            Self::Down => {
                if position.1 + 1 == height {
                    None
                } else {
                    Some((position.0, position.1 + 1))
                }
            }
        }
    }
}
