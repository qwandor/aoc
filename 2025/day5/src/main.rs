use eyre::{Report, eyre};
use std::{
    io::{BufRead, stdin},
    ops::RangeInclusive,
};

fn main() -> Result<(), Report> {
    let ingredients = parse(stdin().lock())?;

    println!("Fresh ingredient count: {}", ingredients.fresh_count());
    println!("Total fresh IDs: {}", ingredients.fresh_ranges_size());

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

    /// Returns the total number of ingredients covered by the fresh ranges.
    fn fresh_ranges_size(&self) -> u64 {
        let merged_ranges = merge_ranges(&self.fresh_ranges);
        println!(
            "Merged {} ranges into {} ranges",
            self.fresh_ranges.len(),
            merged_ranges.len()
        );
        merged_ranges
            .into_iter()
            .map(|range| range.end() - range.start() + 1)
            .sum()
    }
}

fn merge_ranges(ranges: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    let mut sorted = ranges.to_owned();
    sorted.sort_by_key(|range| *range.start());

    let mut merged = Vec::<RangeInclusive<u64>>::new();
    for range in sorted {
        if let Some(last) = merged.last_mut()
            && last.contains(range.start())
        {
            if !last.contains(range.end()) {
                *last = *last.start()..=*range.end()
            }
        } else {
            merged.push(range)
        }
    }
    merged
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

    #[test]
    fn fresh_range_size_example() {
        let ingredients = Ingredients {
            fresh_ranges: vec![3..=5, 10..=14, 16..=20, 12..=18],
            available_ingredients: vec![1, 5, 8, 11, 17, 32],
        };
        assert_eq!(ingredients.fresh_ranges_size(), 14);
    }

    #[test]
    fn merge_more() {
        assert_eq!(merge_ranges(&[]), vec![]);
        assert_eq!(merge_ranges(&[0..=1, 1..=2]), vec![0..=2]);
        assert_eq!(merge_ranges(&[1..=5, 2..=3]), vec![1..=5]);
    }
}
