use eyre::Report;
use regex::{Captures, Regex};
use std::io::{stdin, Read};

fn main() -> Result<(), Report> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let instructions = parse(&input);
    let result = run(&instructions);
    println!("Result: {}", result);
    let result = run_without_disable(&instructions);
    println!("Result ignoring do/don't: {}", result);

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Do,
    Dont,
    Mul(u64, u64),
}

fn parse(mut input: &str) -> Vec<Instruction> {
    let patterns: &[(_, fn(&Captures) -> _)] = &[
        (Regex::new(r"^do\(\)").unwrap(), |_| Instruction::Do),
        (Regex::new(r"^don't\(\)").unwrap(), |_| Instruction::Dont),
        (
            Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap(),
            |captures| {
                Instruction::Mul(
                    captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                )
            },
        ),
    ];

    let mut instructions = Vec::new();
    'parse: while !input.is_empty() {
        for (regex, f) in patterns {
            if let Some(captures) = regex.captures(&input) {
                instructions.push(f(&captures));
                input = &input[captures.len()..];
                continue 'parse;
            }
        }
        input = &input[1..];
    }
    instructions
}

fn run<'a>(instructions: impl IntoIterator<Item = &'a Instruction>) -> u64 {
    let mut enabled = true;
    let mut sum = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Do => {
                enabled = true;
            }
            Instruction::Dont => {
                enabled = false;
            }
            Instruction::Mul(a, b) => {
                if enabled {
                    sum += a * b;
                }
            }
        }
    }
    sum
}

fn run_without_disable(instructions: &[Instruction]) -> u64 {
    run(instructions
        .iter()
        .filter(|instruction| matches!(instruction, Instruction::Mul(_, _))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_example() {
        let instructions =
            parse("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(run(&instructions), 161);
        assert_eq!(run_without_disable(&instructions), 161);
    }

    #[test]
    fn calculate_example2() {
        let instructions =
            parse("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(run(&instructions), 48);
        assert_eq!(run_without_disable(&instructions), 161);
    }
}
