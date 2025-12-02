use eyre::{Context, OptionExt, Report};
use std::{io::stdin, ops::RangeInclusive};

fn main() -> Result<(), Report> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    let ranges = parse(line.trim())?;

    println!(
        "Sum of numbers with digits repeated twice: {}",
        sum_matching(&ranges, digits_repeated_twice)
    );
    println!(
        "Sum of numbers with digits repeated at least twice: {}",
        sum_matching(&ranges, digits_repeated_at_least_twice)
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

fn sum_matching(ranges: &[RangeInclusive<u128>], f: impl Fn(u128) -> bool) -> u128 {
    ranges
        .iter()
        .flat_map(|range| range.clone().filter(|value| f(*value)))
        .sum()
}

/// Returns whether the given number contains the same digits repeated twice.
fn digits_repeated_twice(value: u128) -> bool {
    let num_digits = value.ilog10() + 1;
    let multiplier = 10u128.pow(num_digits / 2);
    value / multiplier == value % multiplier
}

/// Returns whether the given number contains the same digits repeated at least twice.
fn digits_repeated_at_least_twice(value: u128) -> bool {
    let num_digits = value.ilog10() + 1;
    for repeats in 2..=num_digits {
        // The number of repeats must evenly divide the number of digits.
        if num_digits % repeats == 0 {
            let repeated_digits = num_digits / repeats;
            let multiplier = 10u128.pow(repeated_digits);
            let repeated_value = value % multiplier;
            if (1..repeats)
                .all(|repeat| value / multiplier.pow(repeat) % multiplier == repeated_value)
            {
                return true;
            }
        }
    }
    false
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
    fn repeated_twice() {
        assert!(digits_repeated_twice(11));
        assert!(!digits_repeated_twice(12));
        assert!(digits_repeated_twice(22));
        assert!(digits_repeated_twice(123123));
        assert!(!digits_repeated_twice(121));
    }

    #[test]
    fn example_sum_twice() {
        assert_eq!(
            sum_matching(
                &[
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
                ],
                digits_repeated_twice
            ),
            1227775554
        );
    }

    #[test]
    fn repeated_at_least_twice() {
        assert!(digits_repeated_at_least_twice(11));
        assert!(digits_repeated_at_least_twice(22));
        assert!(digits_repeated_at_least_twice(123123));
        assert!(!digits_repeated_at_least_twice(121));
        assert!(digits_repeated_at_least_twice(123123123));
        assert!(digits_repeated_at_least_twice(34343434));
        assert!(digits_repeated_at_least_twice(33333));
    }

    #[test]
    fn example_sum_at_least_twice() {
        assert_eq!(
            sum_matching(
                &[
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
                ],
                digits_repeated_at_least_twice
            ),
            4174379265
        );
    }
}
