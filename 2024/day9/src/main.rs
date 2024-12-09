use eyre::{OptionExt, Report};
use std::{io::stdin, iter::repeat_n};

fn main() -> Result<(), Report> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;

    let lengths = parse_digits(line.trim())?;
    let mut blocks = lengths_to_blocks(&lengths);
    compact(&mut blocks);
    let checksum = checksum(&blocks);
    println!("Checksum after compacting: {}", checksum);

    Ok(())
}

fn parse_digits(line: &str) -> Result<Vec<usize>, Report> {
    line.chars()
        .map(|c| Ok(c.to_digit(10).ok_or_eyre("Non-digit character")? as _))
        .collect()
}

fn lengths_to_blocks(lengths: &[usize]) -> Vec<Option<usize>> {
    let mut blocks = Vec::new();
    for (i, length) in lengths.iter().enumerate() {
        if i % 2 == 0 {
            // File
            let file_id = i / 2;
            blocks.extend(repeat_n(Some(file_id), *length));
        } else {
            // Free space
            blocks.extend(repeat_n(None, *length));
        }
    }
    blocks
}

fn compact(blocks: &mut [Option<usize>]) {
    // The left-most possibly unused block.
    let mut left = 0;
    // The right-most possibly used block.
    let mut right = blocks.len() - 1;

    while left < right {
        if blocks[left].is_some() {
            left += 1;
        } else if blocks[right].is_none() {
            right -= 1;
        } else {
            blocks.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

fn checksum(blocks: &[Option<usize>]) -> usize {
    blocks
        .iter()
        .enumerate()
        .map(|(i, contents)| i * contents.unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let digits = parse_digits(&"2333133121414131402").unwrap();
        assert_eq!(
            digits,
            vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2]
        );
    }

    #[test]
    fn compact_small_example() {
        let lengths = parse_digits(&"12345").unwrap();
        let mut blocks = lengths_to_blocks(&lengths);
        assert_eq!(
            blocks,
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2),
            ]
        );
        compact(&mut blocks);
        assert_eq!(
            blocks,
            vec![
                Some(0),
                Some(2),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None,
            ]
        );
    }

    #[test]
    fn example_checksum() {
        assert_eq!(
            checksum(&[
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(8),
                Some(1),
                Some(1),
                Some(1),
                Some(8),
                Some(8),
                Some(8),
                Some(2),
                Some(7),
                Some(7),
                Some(7),
                Some(3),
                Some(3),
                Some(3),
                Some(6),
                Some(4),
                Some(4),
                Some(6),
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(6),
                Some(6),
                None,
                None,
                None,
            ]),
            1928
        );
    }
}
