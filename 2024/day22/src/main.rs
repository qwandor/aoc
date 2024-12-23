use std::io::stdin;

use eyre::Report;

fn main() -> Result<(), Report> {
    let buyer_initial_numbers = stdin()
        .lines()
        .map(|line| Ok(line?.parse()?))
        .collect::<Result<Vec<u64>, Report>>()?;

    let sum_2000th = buyer_initial_numbers
        .iter()
        .map(|initial| {
            SecretNumberIterator { next: *initial }
                .skip(2000)
                .next()
                .unwrap()
        })
        .sum::<u64>();
    println!("Sum of 2000th secret numbers: {}", sum_2000th);

    let best_sequence = find_best_sequence(&buyer_initial_numbers);
    let best_sequence_profit = total_profit_for_sequence(&buyer_initial_numbers, &best_sequence);
    println!(
        "Best sequence is {:?}, for {} bananas",
        best_sequence, best_sequence_profit
    );

    Ok(())
}

fn next_secret_number(number: u64) -> u64 {
    let a = (number ^ (number << 6)) & 0xffffff;
    let b = (a ^ (a >> 5)) & 0xffffff;
    (b ^ (b << 11)) & 0xffffff
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SecretNumberIterator {
    next: u64,
}

impl Iterator for SecretNumberIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let number = self.next;
        self.next = next_secret_number(self.next);
        Some(number)
    }
}

fn changes(prices: &[u64]) -> Vec<i64> {
    prices
        .windows(2)
        .map(|prices| (prices[1] as i64) - (prices[0] as i64))
        .collect()
}

fn profit_for_sequence(initial_number: u64, sequence: &[i64; 4]) -> u64 {
    let prices = SecretNumberIterator {
        next: initial_number,
    }
    .take(2001)
    .map(|number| number % 10)
    .collect::<Vec<_>>();
    prices
        .windows(5)
        .filter_map(|prices| {
            if changes(prices) == sequence {
                Some(*prices.last().unwrap())
            } else {
                None
            }
        })
        .max()
        .unwrap_or_default()
}

/// Returns the total number of bananas that would be gained by giving the negotating monkey the
/// given sequence of changes.
fn total_profit_for_sequence(initial_numbers: &[u64], sequence: &[i64; 4]) -> u64 {
    initial_numbers
        .iter()
        .map(|initial_number| profit_for_sequence(*initial_number, sequence))
        .sum()
}

/// Finds the best sequence of changes to tell the monkey for the given set of initial buyer secret
/// numbers.
fn find_best_sequence(initial_numbers: &[u64]) -> [i64; 4] {
    let mut best_sequence = [0; 4];
    let mut best_profit = 0;
    for a in -9..9 {
        for b in -9..9 {
            println!("Trying {}, {}, ...", a, b);
            for c in -9..9 {
                println!("Trying {}, {}, {}, ...", a, b, c);
                for d in -9..9 {
                    let profit = total_profit_for_sequence(initial_numbers, &[a, b, c, d]);
                    if profit > best_profit {
                        best_sequence = [a, b, c, d];
                        best_profit = profit;
                    }
                }
            }
        }
    }
    best_sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_next() {
        assert_eq!(next_secret_number(123), 15887950);
        assert_eq!(next_secret_number(15887950), 16495136);
        assert_eq!(next_secret_number(16495136), 527345);
        assert_eq!(next_secret_number(527345), 704524);
        assert_eq!(next_secret_number(704524), 1553684);
        assert_eq!(next_secret_number(1553684), 12683156);
        assert_eq!(next_secret_number(12683156), 11100544);
        assert_eq!(next_secret_number(11100544), 12249484);
        assert_eq!(next_secret_number(12249484), 7753432);
        assert_eq!(next_secret_number(7753432), 5908254);
    }

    #[test]
    fn iterate_example() {
        assert_eq!(
            SecretNumberIterator { next: 1 }.skip(2000).next().unwrap(),
            8685429
        );
        assert_eq!(
            SecretNumberIterator { next: 10 }.skip(2000).next().unwrap(),
            4700978
        );
        assert_eq!(
            SecretNumberIterator { next: 100 }
                .skip(2000)
                .next()
                .unwrap(),
            15273692
        );
        assert_eq!(
            SecretNumberIterator { next: 2024 }
                .skip(2000)
                .next()
                .unwrap(),
            8667524
        );
    }

    #[test]
    fn example_sequence_profit() {
        assert_eq!(profit_for_sequence(1, &[-2, 1, -1, 3]), 7);
        assert_eq!(profit_for_sequence(2, &[-2, 1, -1, 3]), 7);
        assert_eq!(profit_for_sequence(3, &[-2, 1, -1, 3]), 0);
        assert_eq!(profit_for_sequence(2024, &[-2, 1, -1, 3]), 9);
    }

    #[test]
    #[ignore] // Long test, so skip by default.
    fn best_sequence_example() {
        assert_eq!(find_best_sequence(&[1, 2, 3, 2024]), [-2, 1, -1, 3]);
    }
}
