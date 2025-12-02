use eyre::{Context, OptionExt, Report};
use std::{io::stdin, ops::RangeInclusive};

fn main() -> Result<(), Report> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    let ranges = parse(line.trim())?;

    println!(
        "Sum of numbers with digits repeated twice: {}",
        sum_digits_repeated_twice(&ranges)
    );

    Ok(())
}

fn parse(input: &str) -> Result<Vec<RangeInclusive<u128>>, Report> {
    input
        .split(',')
        .map(|part| {
            let (start, end) = part.split_once('-').ok_or_eyre("Missing '-'")?;
            Ok(start
                .parse()
                .with_context(|| format!("Parsing start '{start}'"))?
                ..=end
                    .parse()
                    .with_context(|| format!("Parsing end '{end}'"))?)
        })
        .collect()
}

fn sum_digits_repeated_twice(ranges: &[RangeInclusive<u128>]) -> u128 {
    ranges
        .iter()
        .flat_map(|range| range.clone().filter(|value| digits_repeated_twice(*value)))
        .sum()
}

/// Returns whether the given number contains the same digits repeated twice.
fn digits_repeated_twice(value: u128) -> bool {
    let num_digits = value.ilog10() + 1;
    let multiplier = 10u128.pow(num_digits / 2);
    value / multiplier == value % multiplier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            parse(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            )
            .unwrap(),
            vec![
                11..=22,
                95..=115,
                998..=1012,
                1188511880..=1188511890,
                222220..=222224,
                1698522..=1698528,
                446443..=446449,
                38593856..=38593862,
                565653..=565659,
                824824821..=824824827,
                2121212118..=2121212124,
            ]
        )
    }

    #[test]
    fn silly_pattern() {
        assert!(digits_repeated_twice(11));
        assert!(!digits_repeated_twice(12));
        assert!(digits_repeated_twice(22));
        assert!(digits_repeated_twice(123123));
        assert!(!digits_repeated_twice(121));
    }

    #[test]
    fn example_sum() {
        assert_eq!(
            sum_digits_repeated_twice(&[
                11..=22,
                95..=115,
                998..=1012,
                1188511880..=1188511890,
                222220..=222224,
                1698522..=1698528,
                446443..=446449,
                38593856..=38593862,
                565653..=565659,
                824824821..=824824827,
                2121212118..=2121212124,
            ]),
            1227775554
        );
    }
}
