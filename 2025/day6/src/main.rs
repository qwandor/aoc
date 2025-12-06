use eyre::{Report, eyre};
use std::{
    io::{BufRead, stdin},
    str::FromStr,
};
use utils::grid::Grid;

fn main() -> Result<(), Report> {
    let problems = parse(stdin().lock())?;

    println!("Sum of all solutions: {}", sum_solutions(&problems));

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

fn parse(input: impl BufRead) -> Result<Vec<Problem>, Report> {
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

fn sum_solutions(problems: &[Problem]) -> u64 {
    problems.iter().map(|problem| problem.solution()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            parse(
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
    fn sum_example_problems() {
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
}
