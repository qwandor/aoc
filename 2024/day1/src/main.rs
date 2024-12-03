use eyre::{OptionExt, Report};
use std::io::stdin;

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

    println!("Total distance: {}", total_distance);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_distance() {
        assert_eq!(total_distance(&[1, 2, 3, 3, 3, 4], &[3, 3, 3, 4, 5, 9]), 11);
    }
}
