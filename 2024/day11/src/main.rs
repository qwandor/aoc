use eyre::Report;
use std::io::stdin;

fn main() -> Result<(), Report> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    let mut stones = line
        .split_whitespace()
        .map(|stone| Ok(stone.parse()?))
        .collect::<Result<Vec<u64>, Report>>()?;
    for i in 1..=75 {
        stones = blink(&stones);
        println!("{} stones after blinking {i} times.", stones.len());
    }

    Ok(())
}

fn blink(stones: &[u64]) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|stone| {
            if *stone == 0 {
                vec![1]
            } else if let Some((a, b)) = split(*stone) {
                vec![a, b]
            } else {
                vec![stone * 2024]
            }
        })
        .collect()
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
        let stones = blink(&stones);
        assert_eq!(stones, vec![253000, 1, 7]);
        let stones = blink(&stones);
        assert_eq!(stones, vec![253, 0, 2024, 14168]);
        let stones = blink(&stones);
        assert_eq!(stones, vec![512072, 1, 20, 24, 28676032]);
        let stones = blink(&stones);
        let stones = blink(&stones);
        let stones = blink(&stones);
        assert_eq!(stones.len(), 22);
    }
}
