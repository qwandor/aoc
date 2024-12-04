use eyre::Report;
use regex::Regex;
use std::io::{stdin, Read};

fn main() -> Result<(), Report> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let result = calculate(&input);
    println!("Result: {}", result);

    Ok(())
}

fn calculate(input: &str) -> u64 {
    run(&parse(input))
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Mul(u64, u64),
}

fn parse(input: &str) -> Vec<Instruction> {
    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    mul_regex
        .captures_iter(&input)
        .map(|capture| {
            Instruction::Mul(
                capture.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                capture.get(2).unwrap().as_str().parse::<u64>().unwrap(),
            )
        })
        .collect()
}

fn run(instructions: &[Instruction]) -> u64 {
    let mut sum = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Mul(a, b) => {
                sum += a * b;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_example() {
        assert_eq!(
            calculate("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }
}
