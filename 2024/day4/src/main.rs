mod grid;

use eyre::Report;
use grid::Grid;
use std::io::stdin;

fn main() -> Result<(), Report> {
    let grid = stdin()
        .lines()
        .map(|line| Ok(line?.chars().collect()))
        .collect::<Result<Vec<Vec<_>>, Report>>()?
        .try_into()?;

    let matches = count_matches(&grid, &charvec("XMAS"));
    println!("{} XMAS matches", matches);

    let x_mas_matches = count_x_mas(&grid);
    println!("{} X-MAS matches", x_mas_matches);

    Ok(())
}

fn count_x_mas(grid: &Grid<char>) -> usize {
    let x_mas = vec![
        vec![Some('M'), None, Some('S')],
        vec![None, Some('A'), None],
        vec![Some('M'), None, Some('S')],
    ]
    .try_into()
    .unwrap();
    count_2d_matches(&grid, &x_mas)
        + count_2d_matches(&grid, &x_mas.flip_horizonal())
        + count_2d_matches(&grid, &x_mas.rotate_clockwise())
        + count_2d_matches(&grid, &x_mas.flip_horizonal().rotate_clockwise())
}

/// Returns the number of times the word can be found in the grid, either horizontally, vertically
/// or diagonally in either direction.
fn count_matches<T: Copy + PartialEq>(grid: &Grid<T>, word: &[T]) -> usize {
    let word_reversed = word.iter().rev().copied().collect::<Vec<_>>();

    // Check for horizonal matches.
    grid.rows()
        .map(|row| count_1d_matches(row, &[word, &word_reversed]))
        // Check for vertical matches.
        .chain(
            grid.columns()
                .map(|column| count_1d_matches(&column, &[word, &word_reversed])),
        )
        // Check for diagonal matches.
        .chain(
            grid.diagonals()
                .map(|diagonal| count_1d_matches(&diagonal, &[word, &word_reversed])),
        )
        .sum::<usize>()
}

/// Counts the number of times the given words occur in the given slice, including overlaps.
fn count_1d_matches<T: PartialEq>(slice: &[T], words: &[&[T]]) -> usize {
    words
        .iter()
        .map(|word| {
            slice
                .windows(word.len())
                .filter(|window| window == word)
                .count()
        })
        .sum()
}

fn count_2d_matches<T: PartialEq>(grid: &Grid<T>, pattern: &Grid<Option<T>>) -> usize {
    if pattern.width() > grid.width() || pattern.height() > grid.height() {
        return 0;
    }

    (0..=grid.height() - pattern.height())
        .map(|start_y| {
            (0..=grid.width() - pattern.width())
                .filter(|start_x| subgrid_matches(grid, *start_x, start_y, pattern))
                .count()
        })
        .sum()
}

fn subgrid_matches<T: PartialEq>(
    grid: &Grid<T>,
    start_x: usize,
    start_y: usize,
    pattern: &Grid<Option<T>>,
) -> bool {
    pattern.rows().enumerate().all(|(pattern_y, pattern_row)| {
        pattern_row
            .iter()
            .enumerate()
            .all(|(pattern_x, pattern_element)| {
                pattern_element.is_none()
                    || grid.get(start_x + pattern_x, start_y + pattern_y)
                        == pattern_element.as_ref()
            })
    })
}

fn charvec(s: &str) -> Vec<char> {
    s.chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_empty() {
        assert_eq!(count_matches(&Grid::new(0, 0), &charvec("XMAS")), 0);
    }

    #[test]
    fn count_minimal() {
        assert_eq!(
            count_matches(
                &vec![
                    charvec("..X..."),
                    charvec(".SAMX."),
                    charvec(".A..A."),
                    charvec("XMAS.S"),
                    charvec(".X...."),
                ]
                .try_into()
                .unwrap(),
                &charvec("XMAS")
            ),
            4
        );
    }

    #[test]
    fn count_example() {
        assert_eq!(
            count_matches(
                &vec![
                    charvec("MMMSXXMASM"),
                    charvec("MSAMXMSMSA"),
                    charvec("AMXSXMAAMM"),
                    charvec("MSAMASMSMX"),
                    charvec("XMASAMXAMM"),
                ]
                .try_into()
                .unwrap(),
                &charvec("XMAS")
            ),
            6
        );
        assert_eq!(
            count_matches(
                &vec![
                    charvec("MMMSXXMASM"),
                    charvec("MSAMXMSMSA"),
                    charvec("AMXSXMAAMM"),
                    charvec("MSAMASMSMX"),
                    charvec("XMASAMXAMM"),
                    charvec("XXAMMXXAMA"),
                    charvec("SMSMSASXSS"),
                    charvec("SAXAMASAAA"),
                    charvec("MAMMMXMMMM"),
                    charvec("MXMXAXMASX"),
                ]
                .try_into()
                .unwrap(),
                &charvec("XMAS")
            ),
            18
        );
    }

    #[test]
    fn count_example_x_mas() {
        assert_eq!(
            count_x_mas(
                &vec![
                    charvec("MMMSXXMASM"),
                    charvec("MSAMXMSMSA"),
                    charvec("AMXSXMAAMM"),
                    charvec("MSAMASMSMX"),
                    charvec("XMASAMXAMM"),
                    charvec("XXAMMXXAMA"),
                    charvec("SMSMSASXSS"),
                    charvec("SAXAMASAAA"),
                    charvec("MAMMMXMMMM"),
                    charvec("MXMXAXMASX"),
                ]
                .try_into()
                .unwrap(),
            ),
            9
        );
    }
}
