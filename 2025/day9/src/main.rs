use eyre::{OptionExt, Report};
use std::io::{BufRead, stdin};

fn main() -> Result<(), Report> {
    let positions = parse(stdin().lock())?;

    println!("Largest rectangle: {}", largest_rectangle(&positions));

    Ok(())
}

fn parse(input: impl BufRead) -> Result<Vec<(u64, u64)>, Report> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            let (x, y) = line.split_once(',').ok_or_eyre("Missing ','")?;
            Ok((x.parse()?, y.parse()?))
        })
        .collect()
}

fn largest_rectangle(positions: &[(u64, u64)]) -> u64 {
    positions
        .iter()
        .map(|&a| {
            positions
                .iter()
                .map(|&b| rectangle_area(a, b))
                .max()
                .unwrap_or_default()
        })
        .max()
        .unwrap_or_default()
}

fn rectangle_area(a: (u64, u64), b: (u64, u64)) -> u64 {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            parse(
                "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
                .as_bytes(),
            )
            .unwrap(),
            vec![
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3),
            ]
        );
    }

    #[test]
    fn example() {
        assert_eq!(
            largest_rectangle(&[
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3),
            ]),
            50
        );
    }
}
