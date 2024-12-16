use eyre::{bail, OptionExt, Report};
use std::io::{stdin, BufRead};
use utils::{charvec, grid::Grid, Direction};

fn main() -> Result<(), Report> {
    let (mut state, directions) = parse(stdin().lock())?;
    state.run(&directions)?;
    let box_gps_sum = state.box_gps_sum();
    println!("Box GPS sum: {}", box_gps_sum);

    Ok(())
}

fn parse(input: impl BufRead) -> Result<(State, Vec<Direction>), Report> {
    let mut lines = input.lines();

    let mut map: Grid<char> = (&mut lines)
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

    let mut robot = None;
    for (x, y, element) in map.elements_mut() {
        if *element == '@' {
            robot = Some((x, y));
            *element = '.';
            break;
        }
    }
    let state = State {
        map,
        robot: robot.ok_or_eyre("No robot")?,
    };

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
    robot: (usize, usize),
}

impl State {
    /// Moves the robot according to all the given directions.
    fn run(&mut self, directions: &[Direction]) -> Result<(), Report> {
        for direction in directions {
            self.step(*direction)?;
        }
        Ok(())
    }

    /// Moves the one robot one step in the given direction, if possible.
    fn step(&mut self, direction: Direction) -> Result<(), Report> {
        println!("Robot at {:?}, moving {:?}", self.robot, direction);
        let Some(robot_next) = direction.move_from(self.robot, self.map.width(), self.map.height())
        else {
            // Robot can't move off the map, so do nothing.
            println!("Can't move off map");
            return Ok(());
        };

        match self.map.get(robot_next.0, robot_next.1).unwrap() {
            '#' => {
                // Robot can't move into a wall, so do nothing.
                println!("Robot hit a wall");
                return Ok(());
            }
            'O' => {
                // Move the box first, if possible.
                println!("Robot trying to push a box");
                if !push_box(&mut self.map, robot_next, direction)? {
                    return Ok(());
                }
            }
            '.' => {}
            c => bail!("Unexpected map character '{}'", c),
        }
        self.robot = robot_next;

        Ok(())
    }

    /// Returns the sum of the GPS co-ordinates of all boxes on the map.
    fn box_gps_sum(&self) -> usize {
        self.map
            .elements()
            .map(|(x, y, e)| if *e == 'O' { x + 100 * y } else { 0 })
            .sum()
    }
}

/// Attempts to push the box (if any) at the given location in the given direction.
///
/// Returns true if the box was pushed, or false if it couldn't be.
fn push_box(
    map: &mut Grid<char>,
    position: (usize, usize),
    direction: Direction,
) -> Result<bool, Report> {
    match map.get(position.0, position.1).unwrap() {
        '#' => {
            // Can't push a wall.
            Ok(false)
        }
        'O' => {
            let Some(target_position) = direction.move_from(position, map.width(), map.height())
            else {
                // Can't push off the edge of the map.
                return Ok(false);
            };
            // Push whatever is in the target position first.
            if push_box(map, target_position, direction)? {
                *map.get_mut(target_position.0, target_position.1).unwrap() = 'O';
                *map.get_mut(position.0, position.1).unwrap() = '.';
                Ok(true)
            } else {
                Ok(false)
            }
        }
        '.' => {
            // Nothing to push.
            Ok(true)
        }
        c => bail!("Unexpected map character '{}'", c),
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
        assert_eq!(state.robot, (2, 2));
        assert_eq!(
            state.map,
            Grid::try_from(vec![
                charvec("########"),
                charvec("#..O.O.#"),
                charvec("##..O..#"),
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
        assert_eq!(state.robot, (2, 2));
        state.step(directions[1]).unwrap();
        assert_eq!(state.robot, (2, 1));
        state.step(directions[2]).unwrap();
        assert_eq!(state.robot, (2, 1));
        state.step(directions[3]).unwrap();
        assert_eq!(state.robot, (3, 1));
        state.step(directions[4]).unwrap();
        assert_eq!(state.robot, (4, 1));
        state.step(directions[5]).unwrap();
        assert_eq!(state.robot, (4, 1));
        state.step(directions[6]).unwrap();
        assert_eq!(state.robot, (4, 2));
        state.step(directions[7]).unwrap();
        assert_eq!(state.robot, (4, 2));
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
}
