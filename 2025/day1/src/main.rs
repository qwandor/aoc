use eyre::{Report, bail};
use std::io::{BufRead, stdin};

fn main() -> Result<(), Report> {
    let rotations = parse(stdin().lock())?;

    let zeros = count_zeros(50, &rotations);
    let click_zeros = count_click_zeros(50, &rotations);

    println!("Zeros: {zeros}");
    println!("Zero on any click: {click_zeros}");

    Ok(())
}

fn parse(input: impl BufRead) -> Result<Vec<i64>, Report> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            let Some((first, rest)) = line.split_at_checked(1) else {
                bail!("Empty line");
            };
            let sign = match first {
                "L" => -1,
                "R" => 1,
                c => bail!("Unexpected start of line '{c}'"),
            };
            let value = rest.parse::<i64>()?;
            Ok(sign * value)
        })
        .collect()
}

fn count_zeros(start: i64, rotations: &[i64]) -> u64 {
    let mut count = if start == 0 { 1 } else { 0 };
    if let [first, rest @ ..] = rotations {
        count += count_zeros((start + first).rem_euclid(100), rest);
    }
    count
}

fn count_click_zeros(start: i64, rotations: &[i64]) -> u64 {
    if let [first, rest @ ..] = rotations {
        let count = count_single_rotation_zeros(start, *first);
        count + count_click_zeros((start + first).rem_euclid(100), rest)
    } else {
        0
    }
}

/// Returns the number of times the dial will pass or end on 0 in the given rotation from the given
/// start.
fn count_single_rotation_zeros(start: i64, rotation: i64) -> u64 {
    let end = start + rotation;
    (end.abs() / 100
        + if end == 0 || (start > 0 && end <= 0) {
            1
        } else {
            0
        }) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            parse("L68\nL30\nR48\nR0".as_bytes()).unwrap(),
            vec![-68, -30, 48, 0]
        )
    }

    #[test]
    fn count_zeros_example() {
        assert_eq!(
            count_zeros(50, &[-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]),
            3
        );
    }

    #[test]
    fn count_click_zeros_example() {
        assert_eq!(
            count_click_zeros(50, &[-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]),
            6
        );
    }

    #[test]
    fn count_zeroes_empty() {
        assert_eq!(count_zeros(50, &[]), 0);
        assert_eq!(count_zeros(0, &[]), 1);
    }

    #[test]
    fn count_click_zeroes_empty() {
        assert_eq!(count_click_zeros(50, &[]), 0);
        assert_eq!(count_click_zeros(0, &[]), 0);
    }

    #[test]
    fn count_single_rotation() {
        assert_eq!(count_single_rotation_zeros(99, 2), 1);
        assert_eq!(count_single_rotation_zeros(99, 102), 2);
        assert_eq!(count_single_rotation_zeros(99, 1), 1);
        assert_eq!(count_single_rotation_zeros(0, 100), 1);
        assert_eq!(count_single_rotation_zeros(0, 1), 0);
        assert_eq!(count_single_rotation_zeros(0, 0), 1);
        assert_eq!(count_single_rotation_zeros(1, -2), 1);
        assert_eq!(count_single_rotation_zeros(0, -1), 0);
        assert_eq!(count_single_rotation_zeros(0, -99), 0);
        assert_eq!(count_single_rotation_zeros(0, -100), 1);
        assert_eq!(count_single_rotation_zeros(0, -101), 1);
        assert_eq!(count_single_rotation_zeros(1, -1), 1);

        assert_eq!(count_single_rotation_zeros(50, -68), 1);
        assert_eq!(count_single_rotation_zeros(95, 60), 1);
        assert_eq!(count_single_rotation_zeros(14, -82), 1);

        assert_eq!(count_single_rotation_zeros(52, 48), 1);
        assert_eq!(count_single_rotation_zeros(55, -55), 1);
    }
}
