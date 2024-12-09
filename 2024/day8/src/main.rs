use eyre::Report;
use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};
use utils::parse_chargrid;

fn main() -> Result<(), Report> {
    let world = parse(stdin().lock())?;
    let antinode_count = count_antinodes(&world);
    println!("{} antinodes.", antinode_count);

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

fn count_antinodes(world: &World) -> usize {
    let mut antennas_by_frequency: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for antenna in &world.antennas {
        antennas_by_frequency
            .entry(antenna.frequency)
            .or_default()
            .push(antenna.position);
    }

    let mut antinodes = HashSet::<(usize, usize)>::new();
    for antenna_positions in antennas_by_frequency.values() {
        for (i, antenna1) in antenna_positions.iter().enumerate() {
            for antenna2 in &antenna_positions[0..i] {
                if let (Some(x), Some(y)) = (
                    (antenna1.0 * 2).checked_sub(antenna2.0),
                    (antenna1.1 * 2).checked_sub(antenna2.1),
                ) {
                    if x < world.width && y < world.height {
                        antinodes.insert((x, y));
                    }
                }
                if let (Some(x), Some(y)) = (
                    (antenna2.0 * 2).checked_sub(antenna1.0),
                    (antenna2.1 * 2).checked_sub(antenna1.1),
                ) {
                    if x < world.width && y < world.height {
                        antinodes.insert((x, y));
                    }
                }
            }
        }
    }

    antinodes.len()
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
        assert_eq!(count_antinodes(&world), 14);
    }
}
