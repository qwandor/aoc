use std::{io::stdin, str::FromStr};

use eyre::{OptionExt, Report};

fn main() -> Result<(), Report> {
    let equations = stdin()
        .lines()
        .map(|line| line?.parse())
        .collect::<Result<Vec<Equation>, Report>>()?;

    let plus_minus_valid_sum = valid_sum(&equations, &[Operator::Plus, Operator::Times]);
    println!(
        "Sum of equations valid with + or *: {}",
        plus_minus_valid_sum
    );
    let valid_sum = valid_sum(&equations, &Operator::ALL);
    println!("Sum of equations valid with +, * or ||: {}", valid_sum);

    Ok(())
}

/// Returns the sum of test values from possibly valid equations.
fn valid_sum(equations: &[Equation], operators: &[Operator]) -> u64 {
    equations
        .iter()
        .filter(|equation| can_make_true(equation, operators))
        .map(|equation| equation.test_value)
        .sum()
}

/// Returns whether the given equation can be made true by inserting operators.
fn can_make_true(equation: &Equation, operators: &[Operator]) -> bool {
    match equation.values.as_slice() {
        [] => false,
        [single] => *single == equation.test_value,
        [first, second, rest @ ..] => {
            for operator in operators {
                let mut new_values = vec![operator.apply(*first, *second)];
                new_values.extend_from_slice(rest);
                if can_make_true(
                    &Equation {
                        test_value: equation.test_value,
                        values: new_values,
                    },
                    operators,
                ) {
                    return true;
                }
            }
            false
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Equation {
    test_value: u64,
    values: Vec<u64>,
}

impl FromStr for Equation {
    type Err = Report;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (test_value, rest) = line.split_once(": ").ok_or_eyre("Missing :")?;
        Ok(Self {
            test_value: test_value.parse()?,
            values: rest
                .split_whitespace()
                .map(u64::from_str)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operator {
    Plus,
    Times,
    Concatenate,
}

impl Operator {
    const ALL: [Self; 3] = [Self::Plus, Self::Times, Self::Concatenate];

    fn apply(self, left: u64, right: u64) -> u64 {
        match self {
            Self::Plus => left + right,
            Self::Times => left * right,
            Self::Concatenate => left * 10u64.pow(right.ilog10() + 1) + right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_one() {
        assert_eq!(
            "123: 1 2 3 42".parse::<Equation>().unwrap(),
            Equation {
                test_value: 123,
                values: vec![1, 2, 3, 42],
            }
        );
    }

    #[test]
    fn concatenate() {
        assert_eq!(Operator::Concatenate.apply(1, 1), 11);
        assert_eq!(Operator::Concatenate.apply(12, 34), 1234);
        assert_eq!(Operator::Concatenate.apply(12, 9), 129);
        assert_eq!(Operator::Concatenate.apply(12, 99), 1299);
        assert_eq!(Operator::Concatenate.apply(12, 100), 12100);
    }

    #[test]
    fn example_valid_sum() {
        let equations = &[
            "190: 10 19".parse().unwrap(),
            "3267: 81 40 27".parse().unwrap(),
            "83: 17 5".parse().unwrap(),
            "156: 15 6".parse().unwrap(),
            "7290: 6 8 6 15".parse().unwrap(),
            "161011: 16 10 13".parse().unwrap(),
            "192: 17 8 14".parse().unwrap(),
            "21037: 9 7 18 13".parse().unwrap(),
            "292: 11 6 16 20".parse().unwrap(),
        ];
        assert_eq!(
            valid_sum(equations, &[Operator::Plus, Operator::Times]),
            3749
        );
        assert_eq!(valid_sum(equations, &Operator::ALL), 11387);
    }
}
