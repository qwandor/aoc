use eyre::{eyre, Report};
use regex::Regex;
use std::io::{stdin, BufRead};
use utils::grid::Grid;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn main() -> Result<(), Report> {
    let mut robots = parse(stdin().lock())?;

    {
        let mut robots = robots.clone();
        run(&mut robots, 100, WIDTH, HEIGHT);
        let safety_factor = safety_factor(&robots, WIDTH, HEIGHT);
        println!("Safety factor after 100 seconds: {}", safety_factor);
    }

    find_easter_egg(&mut robots, WIDTH, HEIGHT);

    Ok(())
}

fn find_easter_egg(robots: &mut [Robot], width: i64, height: i64) {
    for i in 0.. {
        if contains_easter_egg(robots, width, height) {
            println!("Found easter egg after {} seconds:", i);
            break;
        }
        step_all(robots, width, height);
    }
    print_robots(robots, width, height);
}

fn contains_easter_egg(robots: &[Robot], width: i64, height: i64) -> bool {
    let grid = robot_grid(robots, width, height);
    let no_overlaps = grid.elements().all(|(_, _, count)| *count <= 1);
    no_overlaps
}

fn robot_grid(robots: &[Robot], width: i64, height: i64) -> Grid<usize> {
    let mut grid: Grid<usize> = Grid::new(width.try_into().unwrap(), height.try_into().unwrap());
    for robot in robots {
        *grid
            .get_mut(
                robot.position.0.try_into().unwrap(),
                robot.position.1.try_into().unwrap(),
            )
            .unwrap() += 1;
    }
    grid
}

fn print_robots(robots: &[Robot], width: i64, height: i64) {
    let grid = robot_grid(robots, width, height);
    for row in grid.rows() {
        for entry in row {
            if *entry == 0 {
                print!(".");
            } else {
                print!("{}", entry)
            }
        }
        println!();
    }
}

/// Runs the given robots for the given number of seconds.
fn run(robots: &mut [Robot], seconds: u64, width: i64, height: i64) {
    for _ in 0..seconds {
        step_all(robots, width, height);
    }
}

fn step_all(robots: &mut [Robot], width: i64, height: i64) {
    for robot in &mut *robots {
        robot.step(width, height);
    }
}

fn parse(input: impl BufRead) -> Result<Vec<Robot>, Report> {
    let pattern = Regex::new(r"p=([0-9-]+),([0-9-]+) v=([0-9-]+),([0-9-]+)").unwrap();
    input
        .lines()
        .map(|line| {
            let line = line?;
            let captures = pattern
                .captures(&line)
                .ok_or_else(|| eyre!("Invalid line: '{}'", line))?;
            Ok(Robot {
                position: (
                    captures.get(1).unwrap().as_str().parse()?,
                    captures.get(2).unwrap().as_str().parse()?,
                ),
                velocity: (
                    captures.get(3).unwrap().as_str().parse()?,
                    captures.get(4).unwrap().as_str().parse()?,
                ),
            })
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    /// Moves the robot one step, i.e. for one second.
    fn step(&mut self, width: i64, height: i64) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        if self.position.0 < 0 {
            self.position.0 += width;
        } else if self.position.0 >= width {
            self.position.0 -= width;
        }
        if self.position.1 < 0 {
            self.position.1 += height;
        } else if self.position.1 >= height {
            self.position.1 -= height;
        }
    }
}

fn safety_factor(robots: &[Robot], width: i64, height: i64) -> u64 {
    // Top left, top right, bottom left, bottom right.
    let mut quadrant_counts = [0; 4];
    for robot in robots {
        if robot.position.1 * 2 == height - 1 || robot.position.0 * 2 == width - 1 {
            // Ignore robot exactly in the middle.
        } else if robot.position.1 * 2 < height {
            if robot.position.0 * 2 < width {
                quadrant_counts[0] += 1;
            } else {
                quadrant_counts[1] += 1;
            }
        } else {
            if robot.position.0 * 2 < width {
                quadrant_counts[2] += 1;
            } else {
                quadrant_counts[3] += 1;
            }
        }
    }
    quadrant_counts.into_iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_small() {
        assert_eq!(
            parse(
                "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
"
                .as_bytes()
            )
            .unwrap(),
            vec![
                Robot {
                    position: (0, 4),
                    velocity: (3, -3),
                },
                Robot {
                    position: (6, 3),
                    velocity: (-1, -3),
                },
                Robot {
                    position: (10, 3),
                    velocity: (-1, 2),
                },
            ]
        );
    }

    #[test]
    fn run_example() {
        let mut robots = parse(
            "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"
            .as_bytes(),
        )
        .unwrap();
        run(&mut robots, 100, 11, 7);
        assert_eq!(safety_factor(&robots, 11, 7), 12);
    }
}
