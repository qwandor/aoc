use eyre::{Report, eyre};
use std::io::{BufRead, stdin};

fn main() -> Result<(), Report> {
    let banks = parse(stdin().lock())?;

    println!(
        "Max total joltage with 2 cells: {}",
        total_max_joltage(&banks, 2)
    );
    println!(
        "Max total joltage with 12 cells: {}",
        total_max_joltage(&banks, 12)
    );

    Ok(())
}

fn parse(input: impl BufRead) -> Result<Vec<Vec<u32>>, Report> {
    input
        .lines()
        .map(|line| {
            line?
                .chars()
                .map(|c| c.to_digit(10).ok_or_else(|| eyre!("Invalid digit '{c}'")))
                .collect()
        })
        .collect()
}

fn total_max_joltage(banks: &[Vec<u32>], cell_count: usize) -> u64 {
    banks.iter().map(|bank| max_joltage(bank, cell_count)).sum()
}

/// Returns the maximum joltage for a single bank.
fn max_joltage(mut bank: &[u32], cell_count: usize) -> u64 {
    if bank.len() < cell_count {
        return 0;
    }

    let mut total = 0;
    for remaining_digits in (0..cell_count).rev() {
        // Leave space for the remaining digits.
        let next_index = first_max_index(&bank[0..bank.len() - remaining_digits]).unwrap();
        total = total * 10 + u64::from(bank[next_index]);
        bank = &bank[next_index + 1..];
    }

    total
}

/// Returns the index of the first value which is equal to the maximum.
fn first_max_index(bank: &[u32]) -> Option<usize> {
    let mut max_so_far = 0;
    let mut max_index = None;
    for (i, &value) in bank.iter().enumerate() {
        if value > max_so_far {
            max_index = Some(i);
            max_so_far = value;
        }
    }
    max_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            parse(
                "987654321111111\n811111111111119\n234234234234278\n818181911112111\n".as_bytes()
            )
            .unwrap(),
            vec![
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
            ]
        )
    }

    #[test]
    fn example_max_joltage_2() {
        assert_eq!(
            max_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2),
            98
        );
        assert_eq!(
            max_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2),
            89
        );
        assert_eq!(
            max_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2),
            78
        );
        assert_eq!(
            max_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2),
            92
        );
    }

    #[test]
    fn example_max_total_joltage_2() {
        assert_eq!(
            total_max_joltage(
                &[
                    vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                    vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                    vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                    vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
                ],
                2
            ),
            357
        );
    }

    #[test]
    fn example_max_joltage_12() {
        assert_eq!(
            max_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
            987654321111
        );
        assert_eq!(
            max_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12),
            811111111119
        );
        assert_eq!(
            max_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12),
            434234234278
        );
        assert_eq!(
            max_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12),
            888911112111
        );
    }

    #[test]
    fn example_max_total_joltage_12() {
        assert_eq!(
            total_max_joltage(
                &[
                    vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                    vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                    vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                    vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
                ],
                12
            ),
            3121910778619
        );
    }
}
