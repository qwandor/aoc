use eyre::Report;
use std::io::stdin;
use utils::{grid::Grid, parse_chargrid};

fn main() -> Result<(), Report> {
    let grid = parse_chargrid(stdin().lock())?;

    println!("Accessible rolls: {}", count_accessible(&grid));

    Ok(())
}

/// Returns the number of rolls of paper with fewer than four adjacent rolls.
fn count_accessible(grid: &Grid<char>) -> usize {
    grid.elements()
        .filter(|&(x, y, &value)| {
            value == '@'
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
                    grid.get(adjacent_x, adjacent_y) == Some(&'@')
                })
                .count()
                    < 4
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let grid = parse_chargrid(
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
}
