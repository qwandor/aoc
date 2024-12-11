use eyre::{OptionExt, Report};
use std::{
    collections::HashSet,
    io::{stdin, BufRead},
};
use utils::{grid::Grid, Direction};

fn main() -> Result<(), Report> {
    let grid = parse(stdin().lock())?;
    // Find trailheads.
    let trailheads = trailheads(&grid);
    let score_sum = sum_trailheads(&grid, &trailheads, trail_score);
    println!("Sum of trailhead scores is {}", score_sum);
    let rating_sum = sum_trailheads(&grid, &trailheads, trail_rating);
    println!("Sum of trailhead ratings is {}", rating_sum);

    Ok(())
}

/// Parses the input into a grid of heights.
fn parse(input: impl BufRead) -> Result<Grid<u8>, Report> {
    input
        .lines()
        .map(|line| {
            line?
                .trim()
                .chars()
                .map(|c| Ok(c.to_digit(10).ok_or_eyre("Not a digit")? as u8))
                .collect::<Result<_, Report>>()
        })
        .collect::<Result<Vec<Vec<u8>>, Report>>()?
        .try_into()
}

/// Returns the (x, y) co-ordinates of all 0 height points.
fn trailheads(grid: &Grid<u8>) -> Vec<(usize, usize)> {
    grid.rows()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, e)| if *e == 0 { Some((x, y)) } else { None })
        })
        .collect()
}

/// Sums the scores of all trailheads according to the given scoring function.
fn sum_trailheads(
    grid: &Grid<u8>,
    trailheads: &[(usize, usize)],
    scoring_function: fn(&Grid<u8>, (usize, usize)) -> usize,
) -> usize {
    trailheads
        .iter()
        .map(|trailhead| scoring_function(grid, *trailhead))
        .sum()
}

/// Returns the score of the trail starting at the given trailhead.
fn trail_score(grid: &Grid<u8>, trailhead: (usize, usize)) -> usize {
    let peaks = trail_peaks(grid, trailhead)
        .into_iter()
        .collect::<HashSet<_>>();
    peaks.len()
}

/// Returns the rating of the trail starting at the given trailhead.
fn trail_rating(grid: &Grid<u8>, trailhead: (usize, usize)) -> usize {
    trail_peaks(grid, trailhead).len()
}

/// Returns all peaks reachable from the given starting point by ascending one each time.
fn trail_peaks(grid: &Grid<u8>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let height = *grid.get(start.0, start.1).unwrap();
    if height == 9 {
        vec![start]
    } else {
        Direction::ALL
            .into_iter()
            .flat_map(|direction| {
                if let Some(next_position) = direction.move_from(start, grid.width(), grid.height())
                {
                    if *grid.get(next_position.0, next_position.1).unwrap() == height + 1 {
                        trail_peaks(grid, next_position)
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_small() {
        assert_eq!(
            parse("".as_bytes()).unwrap(),
            Grid::try_from(vec![]).unwrap()
        );
        assert_eq!(
            parse(
                "\
0123
4567            
"
                .as_bytes()
            )
            .unwrap(),
            Grid::try_from(vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7]]).unwrap()
        );
    }

    #[test]
    fn example_trailheads() {
        let grid = parse(
            "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"
            .as_bytes(),
        )
        .unwrap();
        let trailheads = trailheads(&grid);
        assert_eq!(
            trailheads,
            vec![
                (2, 0),
                (4, 0),
                (4, 2),
                (6, 4),
                (2, 5),
                (5, 5),
                (0, 6),
                (6, 6),
                (1, 7),
            ]
        );
        assert_eq!(trail_score(&grid, (2, 0)), 5);
        assert_eq!(trail_rating(&grid, (2, 0)), 20);
        assert_eq!(sum_trailheads(&grid, &trailheads, trail_score), 36);
        assert_eq!(sum_trailheads(&grid, &trailheads, trail_rating), 81);
    }
}
