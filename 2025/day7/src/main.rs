use eyre::{OptionExt, Report};
use std::io::stdin;
use utils::{grid::Grid, parse_chargrid};

fn main() -> Result<(), Report> {
    let grid = parse_chargrid(stdin().lock())?;
    println!("Beam splits: {}", count_splits(&grid)?);

    Ok(())
}

fn count_splits(grid: &Grid<char>) -> Result<usize, Report> {
    let mut beams = grid
        .rows()
        .next()
        .ok_or_eyre("Empty grid")?
        .iter()
        .map(|&c| c == 'S')
        .collect::<Vec<_>>();

    let mut split_count = 0;

    let width = grid.width();
    for row in grid.rows().skip(1) {
        let mut new_beams = vec![false; width];
        for (x, (&beam, &c)) in beams.iter().zip(row.iter()).enumerate() {
            if beam {
                if c == '^' {
                    split_count += 1;
                    if x > 0 {
                        new_beams[x - 1] = true;
                    }
                    if x < width - 1 {
                        new_beams[x + 1] = true;
                    }
                } else {
                    new_beams[x] = true;
                }
            }
        }
        beams = new_beams;
    }

    Ok(split_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let grid = parse_chargrid(
            "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"
            .as_bytes(),
        )
        .unwrap();

        assert_eq!(count_splits(&grid).unwrap(), 21);
    }
}
