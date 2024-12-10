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
