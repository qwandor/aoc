use std::io::stdin;

use eyre::Report;

fn main() -> Result<(), Report> {
    let buyer_initial_numbers = stdin()
        .lines()
        .map(|line| Ok(line?.parse()?))
        .collect::<Result<Vec<u64>, Report>>()?;

    let sum_2000th = buyer_initial_numbers
        .into_iter()
        .map(|initial| iterate(next_secret_number, initial, 2000))
        .sum::<u64>();
    println!("Sum of 2000th secret numbers: {}", sum_2000th);

    Ok(())
}

fn next_secret_number(number: u64) -> u64 {
    let a = (number ^ (number * 64)) % 16777216;
    let b = (a ^ (a / 32)) % 16777216;
    (b ^ (b * 2048)) % 16777216
}

fn iterate<T>(f: impl Fn(T) -> T, initial: T, iterations: usize) -> T {
    let mut value = initial;
    for _ in 0..iterations {
        value = f(value);
    }
    value
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
        assert_eq!(iterate(next_secret_number, 1, 2000), 8685429);
        assert_eq!(iterate(next_secret_number, 10, 2000), 4700978);
        assert_eq!(iterate(next_secret_number, 100, 2000), 15273692);
        assert_eq!(iterate(next_secret_number, 2024, 2000), 8667524);
    }
}
