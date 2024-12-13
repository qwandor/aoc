use eyre::Report;
use std::io::stdin;
use utils::{grid::Grid, parse_chargrid, Direction};

fn main() -> Result<(), Report> {
    let garden = parse_chargrid(stdin().lock())?;
    let total_price = total_price(&garden);
    println!("Total price: {}", total_price);
    let total_discount_price = total_discount_price(&garden);
    println!("Total discount price: {}", total_discount_price);

    Ok(())
}

fn total_price(garden: &Grid<char>) -> usize {
    let regions = split_regions(garden);
    regions
        .iter()
        .map(|region| area(region) * perimeter(region))
        .sum::<usize>()
}

fn total_discount_price(garden: &Grid<char>) -> usize {
    let regions = split_regions(garden);
    regions
        .iter()
        .map(|region| area(region) * sides(region))
        .sum::<usize>()
}

fn area(region: &Grid<bool>) -> usize {
    region.elements().filter(|(_, _, e)| **e).count()
}

fn perimeter(region: &Grid<bool>) -> usize {
    region
        .elements()
        .map(|(x, y, e)| {
            if *e {
                Direction::ALL
                    .into_iter()
                    .filter(|direction| !in_region(region, x, y, *direction))
                    .count()
            } else {
                0
            }
        })
        .sum()
}

fn sides(region: &Grid<bool>) -> usize {
    // The number of sides is equivalent to the number of corners.
    region
        .elements()
        .map(|(x, y, e)| {
            let up = in_region(region, x, y, Direction::Up);
            let right = in_region(region, x, y, Direction::Right);
            let down = in_region(region, x, y, Direction::Down);
            let left = in_region(region, x, y, Direction::Left);
            match (*e, up, right, down, left) {
                // Outside corners.
                (true, true, true, false, false) => 1,
                (true, false, true, true, false) => 1,
                (true, false, false, true, true) => 1,
                (true, true, false, false, true) => 1,
                // Double outside corners.
                (true, true, false, false, false) => 2,
                (true, false, true, false, false) => 2,
                (true, false, false, true, false) => 2,
                (true, false, false, false, true) => 2,
                // Quadruple outside corners.
                (true, false, false, false, false) => 4,
                // Inside corners.
                (false, false, false, true, true) => 1,
                (false, true, false, false, true) => 1,
                (false, true, true, false, false) => 1,
                (false, false, true, true, false) => 1,
                // Double inside corners.
                (false, false, true, true, true) => 2,
                (false, true, false, true, true) => 2,
                (false, true, true, false, true) => 2,
                (false, true, true, true, false) => 2,
                // Quadruple inside corners.
                (false, true, true, true, true) => 4,
                _ => 0,
            }
        })
        .sum()
}

/// Returns whether moving one space in the given direction is within the region.
///
/// Returns false if the space would be out of bounds.
fn in_region(region: &Grid<bool>, x: usize, y: usize, direction: Direction) -> bool {
    if let Some(neigbour) = direction.move_from((x, y), region.width(), region.height()) {
        *region.get(neigbour.0, neigbour.1).unwrap()
    } else {
        false
    }
}

/// Given a garden, return a grid for each region within it.
fn split_regions(garden: &Grid<char>) -> Vec<Grid<bool>> {
    let mut remaining_plots = garden.map(|plot| Some(*plot));
    let mut regions = Vec::new();
    for y in 0..remaining_plots.height() {
        for x in 0..remaining_plots.width() {
            if remaining_plots.get(x, y).unwrap().is_some() {
                let mut region = Grid::new(remaining_plots.width(), remaining_plots.height());
                add_to_region(&mut region, &mut remaining_plots, x, y);
                regions.push(region);
            }
        }
    }
    regions
}

/// Adds the given position to the region, along with all adjacent plots of the same plant recursively.
fn add_to_region(
    region: &mut Grid<bool>,
    remaining_plots: &mut Grid<Option<char>>,
    x: usize,
    y: usize,
) {
    let plant = remaining_plots.get_mut(x, y).unwrap().take().unwrap();
    *region.get_mut(x, y).unwrap() = true;
    for direction in Direction::ALL {
        if let Some((neighbour_x, neighbour_y)) =
            direction.move_from((x, y), remaining_plots.width(), remaining_plots.height())
        {
            if *remaining_plots.get(neighbour_x, neighbour_y).unwrap() == Some(plant) {
                add_to_region(region, remaining_plots, neighbour_x, neighbour_y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use utils::charvec;

    use super::*;

    #[test]
    fn split_small() {
        let garden = Grid::try_from(vec![charvec("aab"), charvec("cab")]).unwrap();
        let regions = split_regions(&garden);
        assert_eq!(
            regions,
            vec![
                Grid::try_from(vec![vec![true, true, false], vec![false, true, false]]).unwrap(),
                Grid::try_from(vec![vec![false, false, true], vec![false, false, true]]).unwrap(),
                Grid::try_from(vec![vec![false, false, false], vec![true, false, false]]).unwrap(),
            ]
        );
    }

    #[test]
    fn example_prices() {
        assert_eq!(
            total_price(
                &parse_chargrid(
                    "\
AAAA
BBCD
BBCC
EEEC
"
                    .as_bytes(),
                )
                .unwrap()
            ),
            140
        );
        assert_eq!(
            total_price(
                &parse_chargrid(
                    "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"
                    .as_bytes(),
                )
                .unwrap()
            ),
            772
        );
    }

    #[test]
    fn count_sides() {
        assert_eq!(
            sides(&Grid::try_from(vec![vec![true, true], vec![true, true]]).unwrap()),
            4
        );
        assert_eq!(
            sides(&Grid::try_from(vec![vec![true, true], vec![false, false]]).unwrap()),
            4
        );
        assert_eq!(
            sides(&Grid::try_from(vec![vec![true, false], vec![false, false]]).unwrap()),
            4
        );
        assert_eq!(
            sides(&Grid::try_from(vec![vec![true, true], vec![true, false]]).unwrap()),
            6
        );
        assert_eq!(
            sides(&Grid::try_from(vec![vec![true, true, true], vec![true, false, true]]).unwrap()),
            8
        );
        assert_eq!(
            sides(&Grid::try_from(vec![vec![true, true, true], vec![false, true, false]]).unwrap()),
            8
        );
        assert_eq!(
            sides(
                &Grid::try_from(vec![
                    vec![true, true, true],
                    vec![true, false, true],
                    vec![true, true, false]
                ])
                .unwrap()
            ),
            10
        );
    }

    #[test]
    fn example_discount_prices() {
        assert_eq!(
            total_discount_price(
                &parse_chargrid(
                    "\
AAAA
BBCD
BBCC
EEEC
"
                    .as_bytes(),
                )
                .unwrap()
            ),
            80
        );
        assert_eq!(
            total_discount_price(
                &parse_chargrid(
                    "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
"
                    .as_bytes(),
                )
                .unwrap()
            ),
            236
        );
    }
}
