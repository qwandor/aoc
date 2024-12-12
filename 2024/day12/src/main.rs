use eyre::Report;
use std::io::stdin;
use utils::{grid::Grid, parse_chargrid, Direction};

fn main() -> Result<(), Report> {
    let garden = parse_chargrid(stdin().lock())?;
    let total_price = total_price(&garden);
    println!("Total price: {}", total_price);

    Ok(())
}

fn total_price(garden: &Grid<char>) -> usize {
    let regions = split_regions(garden);
    regions
        .iter()
        .map(|region| area(region) * perimeter(region))
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
                    .filter(|direction| {
                        if let Some(neighbour) =
                            direction.move_from((x, y), region.width(), region.height())
                        {
                            !*region.get(neighbour.0, neighbour.1).unwrap()
                        } else {
                            true
                        }
                    })
                    .count()
            } else {
                0
            }
        })
        .sum()
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
}
