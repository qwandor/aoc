use eyre::{bail, Report};
use std::io::{stdin, BufRead};

fn main() -> Result<(), Report> {
    let rotations = parse(stdin().lock())?;

    let zeros = count_zeros(50, &rotations);

    println!("Zeros: {zeros}");

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
    fn modulo() {
        assert_eq!((-66i64).rem_euclid(100), 34);
    }
}
