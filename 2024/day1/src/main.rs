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

    // Calculate and sum the distances.
    let total_distance = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum::<u64>();

    println!("Total distance: {}", total_distance);

    Ok(())
}
