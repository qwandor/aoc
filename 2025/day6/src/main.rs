use eyre::{Report, bail, eyre};
use std::{
    io::{BufRead, read_to_string, stdin},
    str::FromStr,
};
use utils::{grid::Grid, parse_chargrid};

fn main() -> Result<(), Report> {
    let input = read_to_string(stdin().lock())?;

    let problems1 = parse1(input.as_bytes())?;
    println!(
        "Sum of all solutions, part 1: {}",
        sum_solutions(&problems1)
    );

    let problems2 = parse2(input.as_bytes())?;
    println!(
        "Sum of all solutions, part 2: {}",
        sum_solutions(&problems2)
    );

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Problem {
    operation: Operation,
    numbers: Vec<u64>,
}

impl Problem {
    fn solution(&self) -> u64 {
        match self.operation {
            Operation::Addition => self.numbers.iter().sum(),
            Operation::Multiplication => self.numbers.iter().product(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Operation {
    Addition,
    Multiplication,
}

impl FromStr for Operation {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Addition),
            "*" => Ok(Self::Multiplication),
            _ => Err(eyre!("Invalid operation {s}")),
        }
    }
}

fn parse1(input: impl BufRead) -> Result<Vec<Problem>, Report> {
    let grid_entries = input
        .lines()
        .map(|line| Ok(line?.split_whitespace().map(ToOwned::to_owned).collect()))
        .collect::<Result<Vec<_>, Report>>()?;
    let grid = Grid::try_from(grid_entries)?;
    grid.columns()
        .map(|column| {
            Ok(Problem {
                operation: column.last().unwrap().parse()?,
                numbers: column[0..column.len() - 1]
                    .iter()
                    .map(|number| number.parse::<u64>())
                    .collect::<Result<_, _>>()?,
            })
        })
        .collect()
}

fn parse2(input: impl BufRead) -> Result<Vec<Problem>, Report> {
    let chargrid = parse_chargrid(input)?;
    let mut problems = Vec::new();
    let mut numbers = Vec::new();
    for column in chargrid.columns().rev() {
        if column.iter().all(|&c| c == ' ') {
            numbers = Vec::new();
        } else {
            numbers.push(
                column[0..column.len() - 1]
                    .iter()
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()?,
            );
            match *column.last().unwrap() {
                '+' => problems.push(Problem {
                    operation: Operation::Addition,
                    numbers: numbers.clone(),
                }),
                '*' => problems.push(Problem {
                    operation: Operation::Multiplication,
                    numbers: numbers.clone(),
                }),
                ' ' => {}
                c => bail!("Unexpected operation {c}"),
            }
        }
    }
    Ok(problems)
}

fn sum_solutions(problems: &[Problem]) -> u64 {
    problems.iter().map(|problem| problem.solution()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_part_1() {
        assert_eq!(
            parse1(
                "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
                .as_bytes()
            )
            .unwrap(),
            vec![
                Problem {
                    numbers: vec![123, 45, 6],
                    operation: Operation::Multiplication,
                },
                Problem {
                    numbers: vec![328, 64, 98],
                    operation: Operation::Addition,
                },
                Problem {
                    numbers: vec![51, 387, 215],
                    operation: Operation::Multiplication,
                },
                Problem {
                    numbers: vec![64, 23, 314],
                    operation: Operation::Addition,
                },
            ]
        )
    }

    #[test]
    fn parse_example_part_2() {
        assert_eq!(
            parse2(
                "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
                .as_bytes()
            )
            .unwrap(),
            vec![
                Problem {
                    numbers: vec![4, 431, 623],
                    operation: Operation::Addition,
                },
                Problem {
                    numbers: vec![175, 581, 32],
                    operation: Operation::Multiplication,
                },
                Problem {
                    numbers: vec![8, 248, 369],
                    operation: Operation::Addition,
                },
                Problem {
                    numbers: vec![356, 24, 1],
                    operation: Operation::Multiplication,
                },
            ]
        )
    }

    #[test]
    fn sum_example_part_1_problems() {
        assert_eq!(
            sum_solutions(&[
                Problem {
                    numbers: vec![123, 45, 6],
                    operation: Operation::Multiplication,
                },
                Problem {
                    numbers: vec![328, 64, 98],
                    operation: Operation::Addition,
                },
                Problem {
                    numbers: vec![51, 387, 215],
                    operation: Operation::Multiplication,
                },
                Problem {
                    numbers: vec![64, 23, 314],
                    operation: Operation::Addition,
                },
            ]),
            4277556
        );
    }

    #[test]
    fn sum_example_part_2_problems() {
        assert_eq!(
            sum_solutions(&[
                Problem {
                    numbers: vec![4, 431, 623],
                    operation: Operation::Addition,
                },
                Problem {
                    numbers: vec![175, 581, 32],
                    operation: Operation::Multiplication,
                },
                Problem {
                    numbers: vec![8, 248, 369],
                    operation: Operation::Addition,
                },
                Problem {
                    numbers: vec![356, 24, 1],
                    operation: Operation::Multiplication,
                },
            ]),
            3263827
        );
    }
}
