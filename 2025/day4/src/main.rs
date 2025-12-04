use eyre::Report;
use std::io::{BufRead, stdin};
use utils::{grid::Grid, parse_chargrid};

fn main() -> Result<(), Report> {
    let grid = parse(stdin().lock())?;

    println!("Accessible rolls: {}", count_accessible(&grid));
    println!(
        "Accessible rolls with removing others: {}",
        count_accessible_with_removals(&grid)
    );

    Ok(())
}

fn parse(input: impl BufRead) -> Result<Grid<bool>, Report> {
    Ok(parse_chargrid(input)?.map(|&entry| entry == '@'))
}

/// Returns the number of rolls of paper with fewer than four adjacent rolls.
fn count_accessible(grid: &Grid<bool>) -> usize {
    find_accessible(grid).len()
}

/// Returns the number of rolls that can be removed, removing others first if necessary.
fn count_accessible_with_removals(grid: &Grid<bool>) -> usize {
    let accessible = find_accessible(grid);
    if accessible.len() == 0 {
        return 0;
    } else {
        let mut new_grid = grid.clone();
        for &(x, y) in &accessible {
            *new_grid.get_mut(x, y).unwrap() = false;
        }
        accessible.len() + count_accessible_with_removals(&new_grid)
    }
}

/// Returns a list of the co-ordinates of rolls of paper with fewer than four adjacent rolls.
fn find_accessible(grid: &Grid<bool>) -> Vec<(usize, usize)> {
    grid.elements()
        .filter(|&(x, y, &value)| {
            value
                && [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, -1),
                ]
                .into_iter()
                .filter(|&(x_off, y_off)| {
                    let Some(adjacent_x) = x.checked_add_signed(x_off) else {
                        return false;
                    };
                    let Some(adjacent_y) = y.checked_add_signed(y_off) else {
                        return false;
                    };
                    grid.get(adjacent_x, adjacent_y) == Some(&true)
                })
                .count()
                    < 4
        })
        .map(|(x, y, _)| (x, y))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let grid = parse(
            "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(count_accessible(&grid), 13);
    }

    #[test]
    fn example_with_removals() {
        let grid = parse(
            "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(count_accessible_with_removals(&grid), 43);
    }
}
