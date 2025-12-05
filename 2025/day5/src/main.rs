use eyre::{Report, eyre};
use std::{
    io::{BufRead, stdin},
    ops::RangeInclusive,
};

fn main() -> Result<(), Report> {
    let ingredients = parse(stdin().lock())?;

    println!("Fresh ingredient count: {}", ingredients.fresh_count());

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Ingredients {
    fresh_ranges: Vec<RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl Ingredients {
    /// Returns the number of available ingredients that are within at least one fresh range.
    fn fresh_count(&self) -> usize {
        self.available_ingredients
            .iter()
            .filter(|ingredient| {
                self.fresh_ranges
                    .iter()
                    .any(|range| range.contains(ingredient))
            })
            .count()
    }
}

fn parse_range(line: &str) -> Result<RangeInclusive<u64>, Report> {
    let (start, end) = line
        .split_once('-')
        .ok_or_else(|| eyre!("Invalid range {line}"))?;
    Ok(start.parse()?..=end.parse()?)
}

fn parse(input: impl BufRead) -> Result<Ingredients, Report> {
    let mut lines = input.lines();
    let fresh_ranges = (&mut lines)
        .map_while(|line| match line {
            Err(e) => Some(Err(e.into())),
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    None
                } else {
                    Some(parse_range(line))
                }
            }
        })
        .collect::<Result<_, Report>>()?;

    let available_ingredients = lines
        .map(|line| Ok(line?.parse::<u64>()?))
        .collect::<Result<_, Report>>()?;

    Ok(Ingredients {
        fresh_ranges,
        available_ingredients,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            parse(
                "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
                .as_bytes()
            )
            .unwrap(),
            Ingredients {
                fresh_ranges: vec![3..=5, 10..=14, 16..=20, 12..=18],
                available_ingredients: vec![1, 5, 8, 11, 17, 32]
            }
        )
    }

    #[test]
    fn fresh_count_example() {
        let ingredients = Ingredients {
            fresh_ranges: vec![3..=5, 10..=14, 16..=20, 12..=18],
            available_ingredients: vec![1, 5, 8, 11, 17, 32],
        };
        assert_eq!(ingredients.fresh_count(), 3);
    }
}
