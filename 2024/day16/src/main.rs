use eyre::{OptionExt, Report};
use std::{
    collections::{HashMap, HashSet},
    io::stdin,
};
use utils::{grid::Grid, parse_chargrid, Direction};

fn main() -> Result<(), Report> {
    let maze = parse_chargrid(stdin().lock())?;
    println!("Best score: {}", best_path_score(&maze)?);

    Ok(())
}

/// Returns the score of the best path through the maze.
fn best_path_score(maze: &Grid<char>) -> Result<u64, Report> {
    let start = maze
        .elements()
        .find_map(|(x, y, e)| if *e == 'S' { Some((x, y)) } else { None })
        .ok_or_eyre("No start point")?;
    let mut visited = HashSet::new();
    let mut memo = HashMap::new();
    best_score_from_memoised(maze, start, Direction::Right, &mut visited, &mut memo, 0)
        .score()
        .ok_or_eyre("No path to end")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SearchResult {
    Loop,
    Unreachable,
    Score(u64),
}

impl SearchResult {
    fn score(self) -> Option<u64> {
        if let Self::Score(score) = self {
            Some(score)
        } else {
            None
        }
    }
}

fn best_score_from_memoised(
    maze: &Grid<char>,
    start: (usize, usize),
    direction: Direction,
    visited: &mut HashSet<((usize, usize), Direction)>,
    memo: &mut HashMap<((usize, usize), Direction), SearchResult>,
    depth: usize,
) -> SearchResult {
    let indent = str::repeat(" ", depth);
    if let Some(score) = memo.get(&(start, direction)) {
        println!(
            "{}Using memoised {:?} for {:?} from {:?}",
            indent, score, direction, start
        );
        *score
    } else if visited.contains(&(start, direction)) {
        SearchResult::Loop
    } else {
        visited.insert((start, direction));
        let score = best_score_from(maze, start, direction, visited, memo, depth);
        visited.remove(&(start, direction));
        if matches!(score, SearchResult::Unreachable | SearchResult::Score(_)) {
            println!(
                "{}Memoising {:?} for {:?} from {:?}",
                indent, score, direction, start
            );
            memo.insert((start, direction), score);
        }
        score
    }
}

/// Finds the best score starting from the given position, by a recursive depth-first search.
fn best_score_from(
    maze: &Grid<char>,
    start: (usize, usize),
    direction: Direction,
    visited: &mut HashSet<((usize, usize), Direction)>,
    memo: &mut HashMap<((usize, usize), Direction), SearchResult>,
    depth: usize,
) -> SearchResult {
    println!(
        "{}Looking {:?} from {:?}",
        str::repeat(" ", depth),
        direction,
        start
    );
    if *maze.get(start.0, start.1).unwrap() == 'E' {
        println!("Reached end");
        SearchResult::Score(0)
    } else {
        let mut scores = Vec::new();
        let mut looped = false;
        match best_score_from_memoised(
            maze,
            start,
            direction.rotate_clockwise(),
            visited,
            memo,
            depth + 1,
        ) {
            SearchResult::Score(score) => {
                scores.push(score + 1000);
            }
            SearchResult::Loop => {
                looped = true;
            }
            SearchResult::Unreachable => {}
        }
        match best_score_from_memoised(
            maze,
            start,
            direction.rotate_anticlockwise(),
            visited,
            memo,
            depth + 1,
        ) {
            SearchResult::Score(score) => {
                scores.push(score + 1000);
            }
            SearchResult::Loop => {
                looped = true;
            }
            SearchResult::Unreachable => {}
        }
        if let Some(ahead) = direction.move_from(start, maze.width(), maze.height()) {
            if *maze.get(ahead.0, ahead.1).unwrap() != '#' {
                match best_score_from_memoised(maze, ahead, direction, visited, memo, depth + 1) {
                    SearchResult::Score(score) => {
                        scores.push(score + 1);
                    }
                    SearchResult::Loop => {
                        looped = true;
                    }
                    SearchResult::Unreachable => {}
                }
            }
        }
        if let Some(score) = scores.into_iter().min() {
            SearchResult::Score(score)
        } else if looped {
            SearchResult::Loop
        } else {
            SearchResult::Unreachable
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimal_score() {
        let maze = parse_chargrid(
            "\
####
#.E#
#S.#
####
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(best_path_score(&maze).unwrap(), 1002);
    }

    #[test]
    fn example_score() {
        let maze = parse_chargrid(
            "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(best_path_score(&maze).unwrap(), 7036);
    }

    #[test]
    fn example2_score() {
        let maze = parse_chargrid(
            "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(best_path_score(&maze).unwrap(), 11048);
    }
}
