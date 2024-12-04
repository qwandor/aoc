use eyre::Report;
use std::{
    cmp::Ordering,
    io::stdin,
    ops::{Deref, RangeInclusive},
};

const SAFE_CHANGE_RANGE: RangeInclusive<u64> = 1..=3;

fn main() -> Result<(), Report> {
    let reports: Vec<Vec<u64>> = stdin()
        .lines()
        .map(|line| {
            let line = line?;
            line.split_whitespace()
                .map(|level| Ok(level.parse()?))
                .collect()
        })
        .collect::<Result<_, Report>>()?;

    let safe_count = count(&reports, safe);
    let safe_with_dampener_count = count(&reports, safe_with_dampener);
    println!("{} reports are safe.", safe_count);
    println!(
        "{} reports are safe with the problem dampener.",
        safe_with_dampener_count
    );

    Ok(())
}

fn count<'a, E: Deref<Target = D> + 'a, D: ?Sized>(
    elements: impl IntoIterator<Item = &'a E>,
    f: impl Fn(&D) -> bool,
) -> usize {
    elements.into_iter().filter(|report| f(report)).count()
}

/// Returns whether the levels are either all increasing or all decreasing, and adjacent levels
/// differ by `SAFE_CHANGE_RANGE`.
fn safe(levels: &[u64]) -> bool {
    if levels.len() < 2 {
        return true;
    };
    let first_direction = levels[0].cmp(&levels[1]);
    first_direction != Ordering::Equal
        && levels.windows(2).all(|window| {
            let [a, b] = window else { unreachable!() };
            SAFE_CHANGE_RANGE.contains(&a.abs_diff(*b)) && a.cmp(b) == first_direction
        })
}

/// Returns whether the levels are safe if one is removed.
fn safe_with_dampener(levels: &[u64]) -> bool {
    (0..levels.len()).into_iter().any(|level_to_remove| {
        let mut levels = levels.to_vec();
        levels.remove(level_to_remove);
        safe(&levels)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safety_example() {
        assert!(safe(&[7, 6, 4, 2, 1]));
        assert!(!safe(&[1, 2, 7, 8, 9]));
        assert!(!safe(&[9, 7, 6, 2, 1]));
        assert!(!safe(&[1, 3, 2, 4, 5]));
        assert!(!safe(&[8, 6, 4, 4, 1]));
        assert!(safe(&[1, 3, 6, 7, 9]));
    }

    #[test]
    fn safety_empty() {
        assert!(safe(&[]));
    }

    #[test]
    fn safety_single() {
        assert!(safe(&[0]));
        assert!(safe(&[1]));
    }

    #[test]
    fn safety_dampener_example() {
        assert!(safe_with_dampener(&[7, 6, 4, 2, 1]));
        assert!(!safe_with_dampener(&[1, 2, 7, 8, 9]));
        assert!(!safe_with_dampener(&[9, 7, 6, 2, 1]));
        assert!(safe_with_dampener(&[1, 3, 2, 4, 5]));
        assert!(safe_with_dampener(&[8, 6, 4, 4, 1]));
        assert!(safe_with_dampener(&[1, 3, 6, 7, 9]));
    }
}
