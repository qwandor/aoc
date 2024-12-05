use eyre::Report;
use std::io::stdin;

fn main() -> Result<(), Report> {
    let grid: Vec<Vec<char>> = stdin()
        .lines()
        .map(|line| Ok(line?.chars().collect()))
        .collect::<Result<_, Report>>()?;
    let matches = count_matches(&grid, "XMAS");
    println!("{} matches", matches);

    Ok(())
}

/// Returns the number of times the word can be found in the grid, either horizontally, vertically
/// or diagonally in either direction.
fn count_matches(grid: &[Vec<char>], word: &str) -> usize {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }
    let width = grid[0].len();

    let word = word.chars().collect::<Vec<_>>();
    let word_reversed = word.iter().rev().copied().collect::<Vec<_>>();

    // Check for horizonal matches.
    grid.iter()
        .map(|row| count_1d_matches(row, &[&word, &word_reversed]))
        // Check for vertical matches.
        .chain((0..width).map(|x| {
            let column = grid.iter().map(|row| row[x]).collect::<Vec<_>>();
            count_1d_matches(&column, &[&word, &word_reversed])
        }))
        // Check for diagonal matches.
        .chain(
            diagonals(grid)
                .into_iter()
                .map(|diagonal| count_1d_matches(&diagonal, &[&word, &word_reversed])),
        )
        .sum::<usize>()
}

/// Returns all diagonals of the given grid.
fn diagonals<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    if grid.is_empty() || grid[0].is_empty() {
        return Vec::new();
    }
    let height = grid.len();
    let width = grid[0].len();
    (1..width + height)
        .map(|i| {
            // Down to the right.
            (0..height)
                .filter_map(|j| grid[j].get((i + j).checked_sub(height)?).copied())
                .collect::<Vec<_>>()
        })
        .chain((0..width + height - 1).map(|i| {
            // Down to the left.
            (0..height)
                .filter_map(|j| grid[j].get((i).checked_sub(j)?).copied())
                .collect::<Vec<_>>()
        }))
        .collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_diagonals() {
        // abc
        // ABC
        let expected: Vec<Vec<char>> = vec![
            // Down to the right.
            "A".chars().collect(),
            "aB".chars().collect(),
            "bC".chars().collect(),
            "c".chars().collect(),
            // Down to the left.
            "a".chars().collect(),
            "bA".chars().collect(),
            "cB".chars().collect(),
            "C".chars().collect(),
        ];
        assert_eq!(
            diagonals(&["abc".chars().collect(), "ABC".chars().collect()]),
            expected
        );
        // abcd
        // ABCD
        let expected: Vec<Vec<char>> = vec![
            // Down to the right.
            "A".chars().collect(),
            "aB".chars().collect(),
            "bC".chars().collect(),
            "cD".chars().collect(),
            "d".chars().collect(),
            // Down to the left.
            "a".chars().collect(),
            "bA".chars().collect(),
            "cB".chars().collect(),
            "dC".chars().collect(),
            "D".chars().collect(),
        ];
        assert_eq!(
            diagonals(&["abcd".chars().collect(), "ABCD".chars().collect()]),
            expected
        );
        // aA
        // bB
        // cC
        let expected: Vec<Vec<char>> = vec![
            // Down to the right.
            "c".chars().collect(),
            "bC".chars().collect(),
            "aB".chars().collect(),
            "A".chars().collect(),
            // Down to the left.
            "a".chars().collect(),
            "Ab".chars().collect(),
            "Bc".chars().collect(),
            "C".chars().collect(),
        ];
        assert_eq!(
            diagonals(&[
                "aA".chars().collect(),
                "bB".chars().collect(),
                "cC".chars().collect()
            ]),
            expected
        );
    }

    #[test]
    fn count_minimal() {
        assert_eq!(
            count_matches(
                &[
                    "..X...".chars().collect(),
                    ".SAMX.".chars().collect(),
                    ".A..A.".chars().collect(),
                    "XMAS.S".chars().collect(),
                    ".X....".chars().collect(),
                ],
                "XMAS"
            ),
            4
        );
    }

    #[test]
    fn count_example() {
        assert_eq!(
            count_matches(
                &[
                    "MMMSXXMASM".chars().collect(),
                    "MSAMXMSMSA".chars().collect(),
                    "AMXSXMAAMM".chars().collect(),
                    "MSAMASMSMX".chars().collect(),
                    "XMASAMXAMM".chars().collect(),
                ],
                "XMAS"
            ),
            6
        );
        assert_eq!(
            count_matches(
                &[
                    "MMMSXXMASM".chars().collect(),
                    "MSAMXMSMSA".chars().collect(),
                    "AMXSXMAAMM".chars().collect(),
                    "MSAMASMSMX".chars().collect(),
                    "XMASAMXAMM".chars().collect(),
                    "XXAMMXXAMA".chars().collect(),
                    "SMSMSASXSS".chars().collect(),
                    "SAXAMASAAA".chars().collect(),
                    "MAMMMXMMMM".chars().collect(),
                    "MXMXAXMASX".chars().collect(),
                ],
                "XMAS"
            ),
            18
        );
    }
}
