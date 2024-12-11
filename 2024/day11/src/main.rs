use eyre::Report;
use std::{collections::HashMap, io::stdin};

fn main() -> Result<(), Report> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    let stones = line
        .split_whitespace()
        .map(|stone| Ok(stone.parse()?))
        .collect::<Result<Vec<u64>, Report>>()?;
    let mut stone_counts = count(&stones);
    for i in 1..=75 {
        stone_counts = blink(&stone_counts);
        println!(
            "{} stones ({} distinct) after blinking {i} times.",
            stone_counts.values().sum::<usize>(),
            stone_counts.len(),
        );
    }

    Ok(())
}

fn count(stones: &[u64]) -> HashMap<u64, usize> {
    let mut counts = HashMap::new();
    for stone in stones {
        *counts.entry(*stone).or_default() += 1;
    }
    counts
}

fn blink(stone_counts: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_counts = HashMap::new();
    for (stone, count) in stone_counts {
        if *stone == 0 {
            *new_counts.entry(1).or_default() += count;
        } else if let Some((a, b)) = split(*stone) {
            *new_counts.entry(a).or_default() += count;
            *new_counts.entry(b).or_default() += count;
        } else {
            *new_counts.entry(stone * 2024).or_default() += count;
        }
    }
    new_counts
}

fn split(stone: u64) -> Option<(u64, u64)> {
    let digit_count = stone.ilog10() + 1;
    if digit_count % 2 != 0 {
        None
    } else {
        let high_multiplier = 10u64.pow(digit_count / 2);
        let high = stone / high_multiplier;
        let low = stone - high * high_multiplier;
        Some((high, low))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blink_example() {
        let stones = vec![125, 17];
        let stone_counts = count(&stones);
        let stone_counts = blink(&stone_counts);
        assert_eq!(stone_counts, count(&[253000, 1, 7]));
        let stone_counts = blink(&stone_counts);
        assert_eq!(stone_counts, count(&[253, 0, 2024, 14168]));
        let stone_counts = blink(&stone_counts);
        assert_eq!(stone_counts, count(&[512072, 1, 20, 24, 28676032]));
        let stone_counts = blink(&stone_counts);
        let stone_counts = blink(&stone_counts);
        let stone_counts = blink(&stone_counts);
        assert_eq!(stone_counts.values().sum::<usize>(), 22);
    }
}
