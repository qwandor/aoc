use eyre::Report;
use regex::Regex;
use std::io::{read_to_string, stdin};

/// The cost in tokens to press button A.
const COST_A: u64 = 3;
/// The cost in tokens to press button B.
const COST_B: u64 = 1;

fn main() -> Result<(), Report> {
    let machines = parse(&read_to_string(stdin().lock())?)?;

    let total_prize_cost = machines.iter().filter_map(prize_cost).sum::<u64>();
    println!(
        "Total cost of all winnable prizes: {} tokens",
        total_prize_cost
    );

    let corrected_total_prize_cost = machines
        .into_iter()
        .filter_map(|machine| prize_cost(&correct_prize(machine)))
        .sum::<u64>();
    println!(
        "Total cost of all winnable prizes after correction: {} tokens",
        corrected_total_prize_cost
    );

    Ok(())
}

fn correct_prize(machine: Machine) -> Machine {
    Machine {
        prize: (
            machine.prize.0 + 10000000000000,
            machine.prize.1 + 10000000000000,
        ),
        ..machine
    }
}

fn parse(input: &str) -> Result<Vec<Machine>, Report> {
    let pattern = Regex::new(
        "(?m)Button A: X\\+(\\d+), Y\\+(\\d+)\nButton B: X\\+(\\d+), Y\\+(\\d+)\nPrize: X=(\\d+), Y=(\\d+)\n"
    )
    .unwrap();

    pattern
        .captures_iter(input)
        .map(|captures| {
            Ok(Machine {
                button_a: (
                    captures.get(1).unwrap().as_str().parse()?,
                    captures.get(2).unwrap().as_str().parse()?,
                ),
                button_b: (
                    captures.get(3).unwrap().as_str().parse()?,
                    captures.get(4).unwrap().as_str().parse()?,
                ),
                prize: (
                    captures.get(5).unwrap().as_str().parse()?,
                    captures.get(6).unwrap().as_str().parse()?,
                ),
            })
        })
        .collect()
}

fn prize_cost(machine: &Machine) -> Option<u64> {
    let divisor = machine.button_b.1 * machine.button_a.0 - machine.button_b.0 * machine.button_a.1;
    let quotient_a = machine.prize.0 * machine.button_b.1 - machine.prize.1 * machine.button_b.0;
    let quotient_b = machine.prize.1 * machine.button_a.0 - machine.prize.0 * machine.button_a.1;
    if quotient_a % divisor != 0 || quotient_b % divisor != 0 {
        return None;
    }
    let a = u64::try_from(quotient_a / divisor).ok()?;
    let b = u64::try_from(quotient_b / divisor).ok()?;
    Some(a * COST_A + b * COST_B)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            parse(
                "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176
"
            )
            .unwrap(),
            vec![
                Machine {
                    button_a: (94, 34),
                    button_b: (22, 67),
                    prize: (8400, 5400),
                },
                Machine {
                    button_a: (26, 66),
                    button_b: (67, 21),
                    prize: (12748, 12176),
                },
            ],
        );
    }

    #[test]
    fn example_prize_costs() {
        assert_eq!(
            prize_cost(&Machine {
                button_a: (94, 34),
                button_b: (22, 67),
                prize: (8400, 5400),
            }),
            Some(280)
        );
        assert_eq!(
            prize_cost(&Machine {
                button_a: (26, 66),
                button_b: (67, 21),
                prize: (12748, 12176),
            }),
            None
        );
        assert_eq!(
            prize_cost(&Machine {
                button_a: (17, 86),
                button_b: (84, 37),
                prize: (7870, 6450),
            }),
            Some(200)
        );
        assert_eq!(
            prize_cost(&Machine {
                button_a: (69, 23),
                button_b: (27, 71),
                prize: (18641, 10279),
            }),
            None
        );
    }

    #[test]
    fn corrected_example_prize_costs() {
        assert_eq!(
            prize_cost(&correct_prize(Machine {
                button_a: (94, 34),
                button_b: (22, 67),
                prize: (8400, 5400),
            })),
            None
        );
        assert_eq!(
            prize_cost(&correct_prize(Machine {
                button_a: (26, 66),
                button_b: (67, 21),
                prize: (12748, 12176),
            })),
            Some(459236326669)
        );
        assert_eq!(
            prize_cost(&correct_prize(Machine {
                button_a: (17, 86),
                button_b: (84, 37),
                prize: (7870, 6450),
            })),
            None
        );
        assert_eq!(
            prize_cost(&correct_prize(Machine {
                button_a: (69, 23),
                button_b: (27, 71),
                prize: (18641, 10279),
            })),
            Some(416082282239)
        );
    }
}
