use eyre::{Report, eyre};
use std::io::{BufRead, stdin};

fn main() -> Result<(), Report> {
    let banks = parse(stdin().lock())?;

    println!("Max total joltage: {}", total_max_joltage(&banks));

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

fn total_max_joltage(banks: &[Vec<u32>]) -> u32 {
    banks.iter().map(|bank| max_joltage(bank)).sum()
}

/// Returns the maximum joltage for a single bank.
fn max_joltage(bank: &[u32]) -> u32 {
    if bank.len() < 2 {
        return 0;
    }
    // The first index can't be the last value, as there must be space left for the second index.
    let first_index = first_max_index(&bank[0..bank.len() - 1]).unwrap();
    let second_index = first_max_index(&bank[first_index + 1..]).unwrap() + first_index + 1;
    bank[first_index] * 10 + bank[second_index]
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
    fn example_max_joltage() {
        assert_eq!(
            max_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            98
        );
        assert_eq!(
            max_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            89
        );
        assert_eq!(
            max_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            78
        );
        assert_eq!(
            max_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            92
        );
    }

    #[test]
    fn example_max_total_joltage() {
        assert_eq!(
            total_max_joltage(&[
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
            ]),
            357
        );
    }
}
