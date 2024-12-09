use eyre::Report;
use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
    ops::{Add, Mul, Sub},
};
use utils::parse_chargrid;

fn main() -> Result<(), Report> {
    let world = parse(stdin().lock())?;

    let original_antinode_count = count_antinodes(&world, original_antinodes_for_antennas);
    println!("{} antinodes by original model.", original_antinode_count);

    let expanded_antinode_count = count_antinodes(&world, expanded_antinodes_for_antennas);
    println!("{} antinodes by expanded model.", expanded_antinode_count);

    Ok(())
}

fn parse(input: impl BufRead) -> Result<World, Report> {
    let grid = parse_chargrid(input)?;
    let antennas = grid
        .rows()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| {
                if *c == '.' {
                    None
                } else {
                    Some(Antenna {
                        frequency: *c,
                        position: (x, y),
                    })
                }
            })
        })
        .collect();

    Ok(World {
        antennas,
        width: grid.width(),
        height: grid.height(),
    })
}

fn count_antinodes(
    world: &World,
    antinodes_for_antennas: fn((usize, usize), (usize, usize), usize, usize) -> Vec<(usize, usize)>,
) -> usize {
    let mut antennas_by_frequency: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for antenna in &world.antennas {
        antennas_by_frequency
            .entry(antenna.frequency)
            .or_default()
            .push(antenna.position);
    }

    let antinodes: HashSet<(usize, usize)> = antennas_by_frequency
        .values()
        .flat_map(|antenna_positions| {
            antenna_positions
                .iter()
                .enumerate()
                .flat_map(|(i, antenna1)| {
                    antenna_positions[0..i].iter().flat_map(|antenna2| {
                        antinodes_for_antennas(*antenna1, *antenna2, world.width, world.height)
                    })
                })
        })
        .collect();

    antinodes.len()
}

fn original_antinodes_for_antennas(
    antenna1: (usize, usize),
    antenna2: (usize, usize),
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let mut antinodes = Vec::new();
    if let (Some(x), Some(y)) = (
        (antenna1.0 * 2).checked_sub(antenna2.0),
        (antenna1.1 * 2).checked_sub(antenna2.1),
    ) {
        if x < width && y < height {
            antinodes.push((x, y));
        }
    }
    if let (Some(x), Some(y)) = (
        (antenna2.0 * 2).checked_sub(antenna1.0),
        (antenna2.1 * 2).checked_sub(antenna1.1),
    ) {
        if x < width && y < height {
            antinodes.push((x, y));
        }
    }
    antinodes
}

fn expanded_antinodes_for_antennas(
    antenna1: (usize, usize),
    antenna2: (usize, usize),
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let antenna1 = IVec2D(antenna1.0 as isize, antenna1.1 as isize);
    let antenna2 = IVec2D(antenna2.0 as isize, antenna2.1 as isize);
    let width = width as isize;
    let height = height as isize;
    let difference = antenna2 - antenna1;
    let mut antinodes = Vec::new();
    for i in 0.. {
        let antinode = antenna1 + difference * i;
        if antinode.0 < 0 || antinode.1 < 0 || antinode.0 >= width || antinode.1 >= height {
            break;
        }
        antinodes.push((antinode.0 as usize, antinode.1 as usize));
    }
    for i in 1.. {
        let antinode = antenna1 - difference * i;
        if antinode.0 < 0 || antinode.1 < 0 || antinode.0 >= width || antinode.1 >= height {
            break;
        }
        antinodes.push((antinode.0 as usize, antinode.1 as usize));
    }
    antinodes
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct IVec2D(isize, isize);

impl Add for IVec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for IVec2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<isize> for IVec2D {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Antenna {
    frequency: char,
    position: (usize, usize),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct World {
    antennas: Vec<Antenna>,
    width: usize,
    height: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let world = parse(
            "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            world,
            World {
                width: 12,
                height: 12,
                antennas: vec![
                    Antenna {
                        frequency: '0',
                        position: (8, 1),
                    },
                    Antenna {
                        frequency: '0',
                        position: (5, 2),
                    },
                    Antenna {
                        frequency: '0',
                        position: (7, 3),
                    },
                    Antenna {
                        frequency: '0',
                        position: (4, 4),
                    },
                    Antenna {
                        frequency: 'A',
                        position: (6, 5),
                    },
                    Antenna {
                        frequency: 'A',
                        position: (8, 8),
                    },
                    Antenna {
                        frequency: 'A',
                        position: (9, 9),
                    },
                ]
            }
        );
    }

    #[test]
    fn count_antinodes_example() {
        let world = parse(
            "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(count_antinodes(&world, original_antinodes_for_antennas), 14);
        assert_eq!(count_antinodes(&world, expanded_antinodes_for_antennas), 34);
    }

    #[test]
    fn expanded_antinodes_pair() {
        assert_eq!(
            expanded_antinodes_for_antennas((2, 2), (3, 4), 10, 10),
            vec![(2, 2), (3, 4), (4, 6), (5, 8), (1, 0)]
        );
        assert_eq!(
            expanded_antinodes_for_antennas((4, 2), (3, 4), 10, 10),
            vec![(4, 2), (3, 4), (2, 6), (1, 8), (5, 0)]
        );
    }
}
