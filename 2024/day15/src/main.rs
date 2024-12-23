use eyre::{bail, eyre, OptionExt, Report};
use std::io::{stdin, BufRead};
use utils::{charvec, grid::Grid, Direction};

fn main() -> Result<(), Report> {
    let (state, directions) = parse(stdin().lock())?;

    {
        let mut state = state.clone();
        state.run(&directions)?;
        let box_gps_sum = state.box_gps_sum();
        println!("Box GPS sum: {}", box_gps_sum);
    }

    {
        let mut scaled_state = state.scale_up()?;
        scaled_state.run(&directions)?;
        let box_gps_sum = scaled_state.box_gps_sum();
        println!("Box GPS sum for scaled map: {}", box_gps_sum);
    }

    Ok(())
}

fn parse(input: impl BufRead) -> Result<(State, Vec<Direction>), Report> {
    let mut lines = input.lines();

    let map: Grid<char> = (&mut lines)
        .map_while(|line| match line {
            Err(e) => Some(Err(e.into())),
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    None
                } else {
                    Some(Ok(charvec(line)))
                }
            }
        })
        .collect::<Result<Vec<_>, Report>>()?
        .try_into()?;

    let state = State { map };

    let mut directions = Vec::new();
    for line in lines {
        let line = line?;
        for c in line.trim().chars() {
            directions.push(match c {
                '^' => Direction::Up,
                '<' => Direction::Left,
                '>' => Direction::Right,
                'v' => Direction::Down,
                _ => bail!("Invalid direction '{}'", c),
            });
        }
    }

    Ok((state, directions))
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    map: Grid<char>,
}

impl State {
    /// Moves the robot according to all the given directions.
    fn run(&mut self, directions: &[Direction]) -> Result<(), Report> {
        for direction in directions {
            self.step(*direction)?;
        }
        Ok(())
    }

    fn robot_position(&self) -> Option<(usize, usize)> {
        for (x, y, element) in self.map.elements() {
            if *element == '@' {
                return Some((x, y));
            }
        }
        None
    }

    /// Moves the one robot one step in the given direction, if possible.
    fn step(&mut self, direction: Direction) -> Result<(), Report> {
        let robot_position = self.robot_position().ok_or_eyre("No robot")?;
        self.push_box(robot_position, direction, false)?;
        Ok(())
    }

    /// Attempts to push the small item at the given position in the given direction.
    ///
    /// This should only be called by `push_box`.
    fn push_small(
        &mut self,
        position: (usize, usize),
        direction: Direction,
        c: char,
        dry_run: bool,
    ) -> Result<bool, Report> {
        let Some(target_position) =
            direction.move_from(position, self.map.width(), self.map.height())
        else {
            // Can't push off the edge of the map.
            return Ok(false);
        };
        // Push whatever is in the target position first.
        if self.push_box(target_position, direction, dry_run)? {
            if !dry_run {
                *self
                    .map
                    .get_mut(target_position.0, target_position.1)
                    .unwrap() = c;
                *self.map.get_mut(position.0, position.1).unwrap() = '.';
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Attempts to push the box or robot (if any) at the given location in the given direction.
    ///
    /// Returns true if the box was pushed, or false if it couldn't be.
    fn push_box(
        &mut self,
        position: (usize, usize),
        direction: Direction,
        dry_run: bool,
    ) -> Result<bool, Report> {
        let c = *self.map.get(position.0, position.1).unwrap();
        match c {
            '#' => {
                // Can't push a wall.
                Ok(false)
            }
            'O' | '@' => self.push_small(position, direction, c, dry_run),
            '[' => {
                if matches!(direction, Direction::Left | Direction::Right) {
                    // Same as pushing a small box.
                    self.push_small(position, direction, c, dry_run)
                } else {
                    // Check if both halves can be pushed before actually pushing them.
                    let can_push = self.push_small(position, direction, c, true)?
                        && self.push_small((position.0 + 1, position.1), direction, ']', true)?;
                    if dry_run || !can_push {
                        Ok(can_push)
                    } else {
                        Ok(self.push_small(position, direction, c, false)?
                            && self.push_small(
                                (position.0 + 1, position.1),
                                direction,
                                ']',
                                false,
                            )?)
                    }
                }
            }
            ']' => {
                if matches!(direction, Direction::Left | Direction::Right) {
                    // Same as pushing a small box.
                    self.push_small(position, direction, c, dry_run)
                } else {
                    // Check if both halves can be pushed before actually pushing them.
                    let can_push = self.push_small(position, direction, c, true)?
                        && self.push_small((position.0 - 1, position.1), direction, '[', true)?;
                    if dry_run || !can_push {
                        Ok(can_push)
                    } else {
                        Ok(self.push_small(position, direction, c, false)?
                            && self.push_small(
                                (position.0 - 1, position.1),
                                direction,
                                '[',
                                false,
                            )?)
                    }
                }
            }
            '.' => {
                // Nothing to push.
                Ok(true)
            }
            c => bail!("Unexpected map character '{}'", c),
        }
    }

    /// Returns the sum of the GPS co-ordinates of all boxes on the map.
    fn box_gps_sum(&self) -> usize {
        self.map
            .elements()
            .map(|(x, y, e)| {
                if matches!(*e, 'O' | '[') {
                    x + 100 * y
                } else {
                    0
                }
            })
            .sum()
    }

    fn scale_up(self) -> Result<Self, Report> {
        let scaled_map = self
            .map
            .rows()
            .map(|row| {
                Ok(row
                    .iter()
                    .map(|c| match c {
                        '#' => Ok(['#', '#']),
                        'O' => Ok(['[', ']']),
                        '.' => Ok(['.', '.']),
                        '@' => Ok(['@', '.']),
                        _ => Err(eyre!("Invalid character '{}'", c)),
                    })
                    .collect::<Result<Vec<_>, Report>>()?
                    .into_iter()
                    .flatten()
                    .collect())
            })
            .collect::<Result<Vec<_>, Report>>()?
            .try_into()
            .unwrap();
        Ok(Self { map: scaled_map })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_small_example() {
        let (state, directions) = parse(
            "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(state.robot_position().unwrap(), (2, 2));
        assert_eq!(
            state.map,
            Grid::try_from(vec![
                charvec("########"),
                charvec("#..O.O.#"),
                charvec("##@.O..#"),
                charvec("#...O..#"),
                charvec("#.#.O..#"),
                charvec("#...O..#"),
                charvec("#......#"),
                charvec("########"),
            ])
            .unwrap()
        );
        assert_eq!(
            directions,
            vec![
                Direction::Left,
                Direction::Up,
                Direction::Up,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Down,
                Direction::Left,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Left,
            ]
        );
    }

    #[test]
    fn move_small_example() {
        let (mut state, directions) = parse(
            "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"
            .as_bytes(),
        )
        .unwrap();
        state.step(directions[0]).unwrap();
        assert_eq!(state.robot_position().unwrap(), (2, 2));
        state.step(directions[1]).unwrap();
        assert_eq!(state.robot_position().unwrap(), (2, 1));
        state.step(directions[2]).unwrap();
        assert_eq!(state.robot_position().unwrap(), (2, 1));
        state.step(directions[3]).unwrap();
        assert_eq!(state.robot_position().unwrap(), (3, 1));
        state.step(directions[4]).unwrap();
        assert_eq!(state.robot_position().unwrap(), (4, 1));
        state.step(directions[5]).unwrap();
        assert_eq!(state.robot_position().unwrap(), (4, 1));
        state.step(directions[6]).unwrap();
        assert_eq!(state.robot_position().unwrap(), (4, 2));
        state.step(directions[7]).unwrap();
        assert_eq!(state.robot_position().unwrap(), (4, 2));
    }

    #[test]
    fn run_small_example() {
        let (mut state, directions) = parse(
            "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
"
            .as_bytes(),
        )
        .unwrap();
        state.run(&directions).unwrap();
        assert_eq!(state.box_gps_sum(), 2028);
    }

    #[test]
    fn run_example() {
        let (mut state, directions) = parse(
            "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
            .as_bytes(),
        )
        .unwrap();
        state.run(&directions).unwrap();
        assert_eq!(state.box_gps_sum(), 10092);
    }

    #[test]
    fn run_scaled_example() {
        let (state, directions) = parse(
            "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
            .as_bytes(),
        )
        .unwrap();
        let mut state = state.scale_up().unwrap();
        state.run(&directions).unwrap();
        assert_eq!(state.box_gps_sum(), 9021);
    }
}
