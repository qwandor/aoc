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

    let antinodes: HashSet<(usize, usize)> = antennas_by_frequency
        .values()
        .flat_map(|antenna_positions| {
            antenna_positions
                .iter()
                .enumerate()
                .flat_map(|(i, antenna1)| {
                    antenna_positions[0..i].iter().flat_map(|antenna2| {
                        antinodes_for_antennas(antenna1, antenna2, world.width, world.height)
                    })
                })
        })
        .collect();

    antinodes.len()
}

fn antinodes_for_antennas(
    antenna1: &(usize, usize),
    antenna2: &(usize, usize),
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
