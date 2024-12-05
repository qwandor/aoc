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
    let word = word.chars().collect::<Vec<_>>();
    let word_reversed = word.iter().rev().copied().collect::<Vec<_>>();

    // Check for horizonal matches.
    grid.iter()
        .map(|row| count_1d_matches(row, &[&word, &word_reversed]))
        // Check for vertical matches.
        .chain(columns(grid).map(|column| count_1d_matches(&column, &[&word, &word_reversed])))
        // Check for diagonal matches.
        .chain(
            diagonals(grid).map(|diagonal| count_1d_matches(&diagonal, &[&word, &word_reversed])),
        )
        .sum::<usize>()
}

/// Returns all columns of the given grid.
fn columns<T: Copy>(grid: &[Vec<T>]) -> impl Iterator<Item = Vec<T>> + '_ {
    let width = grid.first().map(Vec::len).unwrap_or_default();
    (0..width).map(|x| grid.iter().map(|row| row[x]).collect::<Vec<_>>())
}

/// Returns all diagonals of the given grid.
fn diagonals<T: Copy>(grid: &[Vec<T>]) -> impl Iterator<Item = Vec<T>> + '_ {
    let height = grid.len();
    let width = grid.first().map(Vec::len).unwrap_or_default();
    (1..width + height).flat_map(move |i| {
        [
            // Down to the right.
            (0..height)
                .filter_map(|j| grid[j].get((i + j).checked_sub(height)?).copied())
                .collect::<Vec<_>>(),
            // Down to the left.
            (0..height)
                .filter_map(|j| grid[j].get((i).checked_sub(j + 1)?).copied())
                .collect::<Vec<_>>(),
        ]
    })
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
            "A".chars().collect(),
            "a".chars().collect(),
            "aB".chars().collect(),
            "bA".chars().collect(),
            "bC".chars().collect(),
            "cB".chars().collect(),
            "c".chars().collect(),
            "C".chars().collect(),
        ];
        assert_eq!(
            diagonals(&["abc".chars().collect(), "ABC".chars().collect()]).collect::<Vec<_>>(),
            expected
        );
        // abcd
        // ABCD
        let expected: Vec<Vec<char>> = vec![
            "A".chars().collect(),
            "a".chars().collect(),
            "aB".chars().collect(),
            "bA".chars().collect(),
            "bC".chars().collect(),
            "cB".chars().collect(),
            "cD".chars().collect(),
            "dC".chars().collect(),
            "d".chars().collect(),
            "D".chars().collect(),
        ];
        assert_eq!(
            diagonals(&["abcd".chars().collect(), "ABCD".chars().collect()]).collect::<Vec<_>>(),
            expected
        );
        // aA
        // bB
        // cC
        let expected: Vec<Vec<char>> = vec![
            "c".chars().collect(),
            "a".chars().collect(),
            "bC".chars().collect(),
            "Ab".chars().collect(),
            "aB".chars().collect(),
            "Bc".chars().collect(),
            "A".chars().collect(),
            "C".chars().collect(),
        ];
        assert_eq!(
            diagonals(&[
                "aA".chars().collect(),
                "bB".chars().collect(),
                "cC".chars().collect()
            ])
            .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn count_empty() {
        assert_eq!(count_matches(&[], "XMAS"), 0);
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
