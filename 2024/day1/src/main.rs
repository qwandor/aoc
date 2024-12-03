use eyre::{OptionExt, Report};
use std::{collections::HashMap, io::stdin};

fn main() -> Result<(), Report> {
    // Read lists from stdin, parsing and splitting into two lists.
    let (mut left, mut right) = stdin()
        .lines()
        .map(|line| {
            let line = line?;
            let (left, right) = line.split_once(' ').ok_or_eyre("Missing delimiter")?;
            Ok((left.parse::<u64>()?, right.trim().parse::<u64>()?))
        })
        .collect::<Result<(Vec<_>, Vec<_>), Report>>()?;

    // Sort them both.
    left.sort();
    right.sort();

    let total_distance = total_distance(&left, &right);
    let similarity_score = similarity_score(&left, &right);

    println!("Total distance: {}", total_distance);
    println!("Similarity score: {}", similarity_score);

    Ok(())
}

/// Calculates and sums the distances.
fn total_distance(left_sorted: &[u64], right_sorted: &[u64]) -> u64 {
    left_sorted
        .into_iter()
        .zip(right_sorted.into_iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

/// Calculates the total similarity score.
fn similarity_score(left: &[u64], right: &[u64]) -> u64 {
    let mut right_counts: HashMap<u64, u64> = HashMap::new();
    for right in right {
        *right_counts.entry(*right).or_default() += 1;
    }
    left.into_iter()
        .map(|left| left * right_counts.get(left).copied().unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_distance() {
        assert_eq!(total_distance(&[1, 2, 3, 3, 3, 4], &[3, 3, 3, 4, 5, 9]), 11);
    }

    #[test]
    fn example_similarity() {
        assert_eq!(
            similarity_score(&[1, 2, 3, 3, 3, 4], &[3, 3, 3, 4, 5, 9]),
            31
        );
    }
}
