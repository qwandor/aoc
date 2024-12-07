use eyre::{bail, OptionExt, Report};
use std::{
    collections::HashSet,
    io::{stdin, BufRead},
};
use utils::{grid::Grid, parse_chargrid};

fn main() -> Result<(), Report> {
    let initial_state = State::parse(stdin().lock())?;
    let visited_positions_count = count_visited_positions(initial_state.clone());
    println!(
        "The guard will visit {} positions.",
        visited_positions_count
    );
    let looping_obstruction_count = count_looping_obstacles(initial_state);
    println!(
        "There are {} possible extra obstruction positions to cause the guard to loop.",
        looping_obstruction_count
    );

    Ok(())
}

/// Finds all positions which the guard will visit before leaving.
fn find_visited_positions(mut state: State) -> Grid<bool> {
    let mut visited = Grid::new(state.width, state.height);
    *visited
        .get_mut(state.guard_position.0, state.guard_position.1)
        .unwrap() = true;
    while state.step_guard() {
        *visited
            .get_mut(state.guard_position.0, state.guard_position.1)
            .unwrap() = true;
    }
    visited
}

fn count_visited_positions(initial_state: State) -> usize {
    let visited_positions = find_visited_positions(initial_state);
    visited_positions
        .rows()
        .map(|row| row.iter().copied().map(usize::from).sum::<usize>())
        .sum()
}

/// Checks whether the given state will result in the guard walking round in circles.
fn will_loop(mut state: State) -> bool {
    let mut guard_states = HashSet::new();
    while state.step_guard() {
        let guard_state = (state.guard_position, state.guard_direction);
        if guard_states.contains(&guard_state) {
            return true;
        }
        guard_states.insert(guard_state);
    }
    false
}

/// Returns the number of positions in which a single obstactle could be placed to make the guard
/// loop.
fn count_looping_obstacles(initial_state: State) -> usize {
    // Find candidate positions by checking where the guard will visit without obstactles.
    let candidates = find_visited_positions(initial_state.clone())
        .rows()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, visited)| if *visited { Some((x, y)) } else { None })
        })
        .collect::<Vec<_>>();

    // Check which will actually result in loops.
    candidates
        .into_iter()
        .filter(|new_obstruction| {
            let mut state = initial_state.clone();
            state.obstructions.push(*new_obstruction);
            will_loop(state)
        })
        .count()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    width: usize,
    height: usize,
    /// The (x, y) co-ordinates of all obstructions.
    obstructions: Vec<(usize, usize)>,
    /// The (x, y) co-ordinate of the guard.
    guard_position: (usize, usize),
    guard_direction: Direction,
}

impl State {
    fn parse(input: impl BufRead) -> Result<Self, Report> {
        let grid = parse_chargrid(input)?;

        let mut obstructions = Vec::new();
        let mut guard_position = None;
        let mut guard_direction = Direction::Up;
        for (y, row) in grid.rows().enumerate() {
            for (x, c) in row.iter().enumerate() {
                match c {
                    '#' => {
                        obstructions.push((x, y));
                    }
                    '>' | '<' | '^' | 'v' if guard_position.is_some() => {
                        bail!("Found two guards.");
                    }
                    '>' => {
                        guard_direction = Direction::Right;
                        guard_position = Some((x, y));
                    }
                    '<' => {
                        guard_direction = Direction::Left;
                        guard_position = Some((x, y));
                    }
                    '^' => {
                        guard_direction = Direction::Up;
                        guard_position = Some((x, y));
                    }
                    'v' => {
                        guard_direction = Direction::Down;
                        guard_position = Some((x, y));
                    }
                    '.' => {}
                    _ => {
                        bail!("Unexpected character in input: '{}'", c);
                    }
                }
            }
        }

        Ok(Self {
            width: grid.width(),
            height: grid.height(),
            obstructions,
            guard_position: guard_position.ok_or_eyre("No guard")?,
            guard_direction,
        })
    }

    /// Move the guard one step, or return false if the guard would move out of bounds.
    fn step_guard(&mut self) -> bool {
        let next_guard_position = match self.guard_direction {
            Direction::Left => {
                if self.guard_position.0 == 0 {
                    return false;
                }
                (self.guard_position.0 - 1, self.guard_position.1)
            }
            Direction::Right => {
                if self.guard_position.0 + 1 == self.width {
                    return false;
                }
                (self.guard_position.0 + 1, self.guard_position.1)
            }
            Direction::Up => {
                if self.guard_position.1 == 0 {
                    return false;
                }
                (self.guard_position.0, self.guard_position.1 - 1)
            }
            Direction::Down => {
                if self.guard_position.1 + 1 == self.height {
                    return false;
                }
                (self.guard_position.0, self.guard_position.1 + 1)
            }
        };
        if self.obstructions.contains(&next_guard_position) {
            self.guard_direction = self.guard_direction.rotate_clockwise();
        } else {
            self.guard_position = next_guard_position;
        }
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_clockwise(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let parsed = State::parse(
            "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(
            parsed,
            State {
                width: 10,
                height: 10,
                guard_position: (4, 6),
                guard_direction: Direction::Up,
                obstructions: vec![
                    (4, 0),
                    (9, 1),
                    (2, 3),
                    (7, 4),
                    (1, 6),
                    (8, 7),
                    (0, 8),
                    (6, 9),
                ]
            }
        );
    }

    #[test]
    fn count_visited_example() {
        let initial_state = State::parse(
            "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(count_visited_positions(initial_state), 41);
    }

    #[test]
    fn example_loops() {
        assert!(!will_loop(
            State::parse(
                "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
                .as_bytes()
            )
            .unwrap()
        ));
        assert!(will_loop(
            State::parse(
                "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#.#^.....
........#.
#.........
......#...
"
                .as_bytes()
            )
            .unwrap()
        ));
        assert!(will_loop(
            State::parse(
                "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
......#.#.
#.........
......#...
"
                .as_bytes()
            )
            .unwrap()
        ));
    }

    #[test]
    fn example_loop_count() {
        let initial_state = State::parse(
            "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(count_looping_obstacles(initial_state), 6);
    }
}
