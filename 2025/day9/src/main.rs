use eyre::{OptionExt, Report};
use std::{
    cmp::{max, min},
    io::{BufRead, stdin},
};

fn main() -> Result<(), Report> {
    let positions = parse(stdin().lock())?;

    println!("Largest rectangle: {}", largest_rectangle(&positions));
    println!(
        "Largest filled rectangle: {}",
        largest_filled_rectangle(&positions)
    );

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

fn find_midpoints(positions: &[(u64, u64)]) -> Vec<(u64, u64)> {
    (0..positions.len())
        .map(|i| {
            let j = (i + 1) % positions.len();
            let midpoint = (
                (positions[i].0 + positions[j].0) / 2,
                (positions[i].1 + positions[j].1) / 2,
            );
            midpoint
        })
        .collect()
}

fn largest_filled_rectangle(positions: &[(u64, u64)]) -> u64 {
    let midpoints = find_midpoints(positions);

    positions
        .iter()
        .map(|&a| {
            positions
                .iter()
                .filter(|&&b| {
                    !midpoints
                        .iter()
                        .any(|&point| rectangle_contains_point(a, b, point))
                })
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

/// Returns whether the given rectangle strictly contains the given point.
fn rectangle_contains_point(a: (u64, u64), b: (u64, u64), point: (u64, u64)) -> bool {
    let min_x = min(a.0, b.0);
    let min_y = min(a.1, b.1);
    let max_x = max(a.0, b.0);
    let max_y = max(a.1, b.1);
    point.0 > min_x && point.0 < max_x && point.1 > min_y && point.1 < max_y
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
    fn example_part1() {
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

    #[test]
    fn midpoints() {
        assert_eq!(find_midpoints(&[]), vec![]);
        assert_eq!(
            find_midpoints(&[(1, 1), (1, 5), (3, 5), (3, 1)]),
            vec![(1, 3), (2, 5), (3, 3), (2, 1)]
        );
        assert_eq!(
            find_midpoints(&[
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3),
            ]),
            vec![
                (9, 1),
                (11, 4),
                (10, 7),
                (9, 6),
                (5, 5),
                (2, 4),
                (4, 3),
                (7, 2),
            ]
        );
    }

    #[test]
    fn contains_point() {
        assert!(rectangle_contains_point((2, 3), (9, 7), (7, 5)));
    }

    #[test]
    fn example_part2() {
        assert_eq!(
            largest_filled_rectangle(&[
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3),
            ]),
            24
        );
    }
}
