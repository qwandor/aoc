use eyre::{bail, OptionExt, Report};
use regex::Regex;
use std::io::{read_to_string, stdin};

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

fn main() -> Result<(), Report> {
    let (mut registers, program) = parse(&read_to_string(stdin().lock())?)?;
    let output = run(&mut registers, &program)?;
    println!(
        "Output: {}",
        output
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    let a = find_quine(&program)?;
    println!("Register A must be {} for quine.", a);

    Ok(())
}

/// Finds the lowest value for register A which makes the given program produce a copy of itself.
fn find_quine(program: &[u8]) -> Result<u64, Report> {
    for a in 0.. {
        let mut registers = [a, 0, 0];
        let res = run(&mut registers, program)?;
        if a % 100000 == 0 {
            println!("{} => {:?} ({})", a, res, res.len());
        }
        if res == program {
            return Ok(a);
        }
    }
    bail!("No quine found.");
}

fn parse(input: &str) -> Result<([u64; 3], Vec<u8>), Report> {
    let pattern = Regex::new(
        "Register A: (\\d+)\nRegister B: (\\d+)\nRegister C: (\\d+)\n\nProgram: ([0-7,]+)",
    )
    .unwrap();
    let captures = pattern.captures(input).ok_or_eyre("Invalid input format")?;
    let registers = [
        captures.get(1).unwrap().as_str().parse()?,
        captures.get(2).unwrap().as_str().parse()?,
        captures.get(3).unwrap().as_str().parse()?,
    ];
    let program = captures
        .get(4)
        .unwrap()
        .as_str()
        .split(',')
        .map(|part| part.parse())
        .collect::<Result<_, _>>()?;

    Ok((registers, program))
}

fn run(registers: &mut [u64; 3], program: &[u8]) -> Result<Vec<u8>, Report> {
    let mut output = Vec::new();
    let mut pc = 0;

    while pc + 1 < program.len() {
        let instruction = program[pc];
        let operand = program[pc + 1];

        match instruction {
            ADV => {
                registers[0] /= 2u64.pow(get_combo(registers, operand)?.try_into()?);
            }
            BXL => {
                registers[1] ^= u64::from(operand);
            }
            BST => {
                registers[1] = get_combo(registers, operand)? & 0b111;
            }
            JNZ => {
                if registers[0] != 0 {
                    pc = operand.into();
                    continue;
                }
            }
            BXC => {
                registers[1] ^= registers[2];
            }
            OUT => {
                output.push((get_combo(registers, operand)? & 0b111) as u8);
            }
            BDV => {
                registers[1] = registers[0] / 2u64.pow(get_combo(registers, operand)?.try_into()?);
            }
            CDV => {
                registers[2] = registers[0] / 2u64.pow(get_combo(registers, operand)?.try_into()?);
            }
            _ => {
                bail!("Invalid instruction {}", instruction);
            }
        }
        pc += 2;
    }

    Ok(output)
}

fn get_combo(registers: &[u64; 3], combo_operand: u8) -> Result<u64, Report> {
    if combo_operand >= 7 {
        bail!("Invalid combo operand {}", combo_operand);
    } else if combo_operand < 4 {
        Ok(combo_operand.into())
    } else {
        Ok(registers[usize::from(combo_operand - 4)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let (mut registers, program) = parse(
            "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
",
        )
        .unwrap();

        assert_eq!(registers, [729, 0, 0]);
        assert_eq!(program, vec![0, 1, 5, 4, 3, 0]);

        assert_eq!(
            run(&mut registers, &program).unwrap(),
            vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]
        );
    }

    #[test]
    fn find_quine_example() {
        assert_eq!(find_quine(&[0, 3, 5, 4, 3, 0]).unwrap(), 117440);
    }
}
