use eyre::{Report, bail, eyre};
use std::{
    io::{BufRead, stdin},
    str::FromStr,
};

fn main() -> Result<(), Report> {
    let machines = parse(stdin().lock())?;

    println!("Minimum button presses: {}", find_min_presses(&machines));

    Ok(())
}

fn parse(input: impl BufRead) -> Result<Vec<Machine>, Report> {
    input
        .lines()
        .map(|line| {
            let line = line?;
            line.parse()
        })
        .collect()
}

fn find_min_presses(machines: &[Machine]) -> u64 {
    machines.iter().map(Machine::min_presses).sum()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u64>,
}

impl Machine {
    fn min_presses(&self) -> u64 {
        0
    }
}

impl FromStr for Machine {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((lights, rest)) = s.split_once("] (") else {
            bail!("Missing end of lights");
        };
        if !lights.starts_with('[') {
            bail!("Missing start of lights");
        }
        let lights = lights
            .chars()
            .skip(1)
            .map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                _ => Err(eyre!("Unexpected light character '{c}'")),
            })
            .collect::<Result<_, _>>()?;

        let Some((buttons, joltages)) = rest.split_once(") {") else {
            bail!("Missing start of joltages");
        };
        let buttons = buttons
            .split(") (")
            .map(|button| button.split(',').map(|light| light.parse()).collect())
            .collect::<Result<_, _>>()?;

        let joltages = joltages
            .trim_end_matches('}')
            .split(',')
            .map(|joltage| joltage.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self {
            lights,
            buttons,
            joltages,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        assert_eq!(
            parse(
                "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
                .as_bytes(),
            )
            .unwrap(),
            vec![
                Machine {
                    lights: vec![false, true, true, false],
                    buttons: vec![
                        vec![3],
                        vec![1, 3],
                        vec![2],
                        vec![2, 3],
                        vec![0, 2],
                        vec![0, 1],
                    ],
                    joltages: vec![3, 5, 4, 7],
                },
                Machine {
                    lights: vec![false, false, false, true, false],
                    buttons: vec![
                        vec![0, 2, 3, 4],
                        vec![2, 3],
                        vec![0, 4],
                        vec![0, 1, 2],
                        vec![1, 2, 3, 4],
                    ],
                    joltages: vec![7, 5, 12, 7, 2],
                },
                Machine {
                    lights: vec![false, true, true, true, false, true],
                    buttons: vec![
                        vec![0, 1, 2, 3, 4],
                        vec![0, 3, 4],
                        vec![0, 1, 2, 4, 5],
                        vec![1, 2],
                    ],
                    joltages: vec![10, 11, 11, 5, 10, 5],
                }
            ]
        );
    }

    #[test]
    fn example_min_presses() {
        assert_eq!(
            find_min_presses(&[
                Machine {
                    lights: vec![false, true, true, false],
                    buttons: vec![
                        vec![3],
                        vec![1, 3],
                        vec![2],
                        vec![2, 3],
                        vec![0, 2],
                        vec![0, 1],
                    ],
                    joltages: vec![3, 5, 4, 7],
                },
                Machine {
                    lights: vec![false, false, false, true, false],
                    buttons: vec![
                        vec![0, 2, 3, 4],
                        vec![2, 3],
                        vec![0, 4],
                        vec![0, 1, 2],
                        vec![1, 2, 3, 4],
                    ],
                    joltages: vec![7, 5, 12, 7, 2],
                },
                Machine {
                    lights: vec![false, true, true, true, false, true],
                    buttons: vec![
                        vec![0, 1, 2, 3, 4],
                        vec![0, 3, 4],
                        vec![0, 1, 2, 4, 5],
                        vec![1, 2],
                    ],
                    joltages: vec![10, 11, 11, 5, 10, 5],
                }
            ]),
            7
        );
    }
}
