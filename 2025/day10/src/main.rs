use eyre::{Report, bail};
use std::{
    cmp::min,
    io::{BufRead, stdin},
    str::FromStr,
    u32,
};

fn main() -> Result<(), Report> {
    let machines = parse(stdin().lock())?;

    println!(
        "Minimum button presses for lights: {}",
        find_min_presses(&machines, Machine::min_light_presses)
    );
    println!(
        "Minimum button presses for joltage: {}",
        find_min_presses(&machines, Machine::min_joltage_presses)
    );

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

fn find_min_presses(machines: &[Machine], min_presses: fn(&Machine) -> u32) -> u32 {
    machines.iter().map(min_presses).sum()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Machine {
    /// A bitset, 1 means on.
    lights: u64,
    /// A 1 bit for a button means it toggles that light.
    buttons: Vec<u64>,
    joltages: Vec<u64>,
}

impl Machine {
    fn min_light_presses(&self) -> u32 {
        let mut min_presses = u32::MAX;
        // Try pressing all possible combinations of buttons.
        for combination in 0u64..(1 << self.buttons.len()) {
            let mut light_result = 0;
            for (i, button) in self.buttons.iter().enumerate() {
                if combination & (1 << i) != 0 {
                    light_result ^= button;
                }
            }
            if light_result == self.lights {
                min_presses = min(min_presses, combination.count_ones());
            }
        }
        min_presses
    }

    fn min_joltage_presses(&self) -> u32 {
        for presses in 0..=self
            .joltages
            .iter()
            .copied()
            .sum::<u64>()
            .try_into()
            .unwrap()
        {
            println!("Trying {presses} presses");
            if self.can_make_joltage_with_presses(presses, &mut vec![0; self.joltages.len()]) {
                return presses;
            }
        }
        u32::MAX
    }

    // Returns whether it is possible to make the desired joltages with no more than the given
    // number of button presses, starting from the given joltages.
    fn can_make_joltage_with_presses(&self, max_presses: u32, counters: &mut [u64]) -> bool {
        if counters == self.joltages {
            true
        } else if max_presses == 0
            || counters
                .iter()
                .enumerate()
                .any(|(i, &counter)| counter > self.joltages[i])
        {
            false
        } else {
            for button in &self.buttons {
                for bit in 0..size_of::<u64>() {
                    if button & (1 << bit) != 0 {
                        counters[bit] += 1;
                    }
                }
                if self.can_make_joltage_with_presses(max_presses - 1, counters) {
                    return true;
                }
                for bit in 0..size_of::<u64>() {
                    if button & (1 << bit) != 0 {
                        counters[bit] -= 1;
                    }
                }
            }
            false
        }
    }
}

impl FromStr for Machine {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((lights_str, rest)) = s.split_once("] (") else {
            bail!("Missing end of lights");
        };
        if !lights_str.starts_with('[') {
            bail!("Missing start of lights");
        }

        let mut lights = 0;
        for c in lights_str[1..].chars().rev() {
            lights <<= 1;
            match c {
                '#' => lights |= 1,
                '.' => {}
                _ => bail!("Unexpected light character '{c}'"),
            }
        }

        let Some((buttons, joltages)) = rest.split_once(") {") else {
            bail!("Missing start of joltages");
        };
        let buttons = buttons
            .split(") (")
            .map(|button| {
                button
                    .split(',')
                    .map(|light| 1 << light.parse::<u8>().unwrap())
                    .sum()
            })
            .collect();

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
                    lights: 0b0110,
                    buttons: vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011],
                    joltages: vec![3, 5, 4, 7],
                },
                Machine {
                    lights: 0b01000,
                    buttons: vec![0b11101, 0b01100, 0b10001, 0b00111, 0b11110],
                    joltages: vec![7, 5, 12, 7, 2],
                },
                Machine {
                    lights: 0b101110,
                    buttons: vec![0b011111, 0b011001, 0b110111, 0b000110],
                    joltages: vec![10, 11, 11, 5, 10, 5],
                },
            ]
        );
    }

    #[test]
    fn example_min_light_presses() {
        let machine1 = Machine {
            lights: 0b0110,
            buttons: vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011],
            joltages: vec![3, 5, 4, 7],
        };
        let machine2 = Machine {
            lights: 0b01000,
            buttons: vec![0b11101, 0b01100, 0b10001, 0b00111, 0b11110],
            joltages: vec![7, 5, 12, 7, 2],
        };
        let machine3 = Machine {
            lights: 0b101110,
            buttons: vec![0b011111, 0b011001, 0b110111, 0b000110],
            joltages: vec![10, 11, 11, 5, 10, 5],
        };
        assert_eq!(machine1.min_light_presses(), 2);
        assert_eq!(machine2.min_light_presses(), 3);
        assert_eq!(machine3.min_light_presses(), 2);
        assert_eq!(
            find_min_presses(&[machine1, machine2, machine3], Machine::min_light_presses),
            7
        );
    }

    #[test]
    fn simple_min_joltage_presses() {
        assert_eq!(
            Machine {
                lights: 0,
                buttons: vec![0b01, 0b10, 0b11],
                joltages: vec![0, 0],
            }
            .min_joltage_presses(),
            0
        );
        assert_eq!(
            Machine {
                lights: 0,
                buttons: vec![0b01, 0b10, 0b11],
                joltages: vec![0, 1],
            }
            .min_joltage_presses(),
            1
        );
        assert_eq!(
            Machine {
                lights: 0,
                buttons: vec![0b01, 0b10, 0b11],
                joltages: vec![1, 0],
            }
            .min_joltage_presses(),
            1
        );
        assert_eq!(
            Machine {
                lights: 0,
                buttons: vec![0b01, 0b10, 0b11],
                joltages: vec![1, 1],
            }
            .min_joltage_presses(),
            1
        );
        assert_eq!(
            Machine {
                lights: 0,
                buttons: vec![0b01],
                joltages: vec![1, 0],
            }
            .min_joltage_presses(),
            1
        );
        assert_eq!(
            Machine {
                lights: 0,
                buttons: vec![0b01],
                joltages: vec![2, 0],
            }
            .min_joltage_presses(),
            2
        );
        assert_eq!(
            Machine {
                lights: 0,
                buttons: vec![0b01, 0b10],
                joltages: vec![1, 1],
            }
            .min_joltage_presses(),
            2
        );
    }

    #[test]
    fn example_min_joltage_presses() {
        let machine1 = Machine {
            lights: 0b0110,
            buttons: vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011],
            joltages: vec![3, 5, 4, 7],
        };
        let machine2 = Machine {
            lights: 0b01000,
            buttons: vec![0b11101, 0b01100, 0b10001, 0b00111, 0b11110],
            joltages: vec![7, 5, 12, 7, 2],
        };
        let machine3 = Machine {
            lights: 0b101110,
            buttons: vec![0b011111, 0b011001, 0b110111, 0b000110],
            joltages: vec![10, 11, 11, 5, 10, 5],
        };
        assert_eq!(machine1.min_joltage_presses(), 10);
        assert_eq!(machine2.min_joltage_presses(), 12);
        assert_eq!(machine3.min_joltage_presses(), 11);
        assert_eq!(
            find_min_presses(
                &[machine1, machine2, machine3],
                Machine::min_joltage_presses
            ),
            33
        );
    }
}
