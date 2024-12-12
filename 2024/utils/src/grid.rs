use eyre::{bail, Report};
use std::{
    fmt::{self, Debug, Formatter},
    iter::repeat_with,
};

/// A 2D grid of items.
#[derive(Clone, Default, Eq, PartialEq)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    elements: Box<[T]>,
}

impl<T> Grid<T> {
    /// Returns the width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Gets element at the given position, if it is within bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if y < self.height && x < self.width {
            Some(&self.elements[y * self.width + x])
        } else {
            None
        }
    }

    /// Gets a mutable reference to the element at the given position, if it is within bounds.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if y < self.height && x < self.width {
            Some(&mut self.elements[y * self.width + x])
        } else {
            None
        }
    }

    /// Returns an iterator over rows of the grid.
    pub fn rows(&self) -> impl DoubleEndedIterator<Item = &[T]> {
        if self.elements.is_empty() {
            // `chunks_exact` will panic if we pass 0, but it doesn't actually matter what we pass
            // as `elements` is empty anyway.
            self.elements.chunks_exact(1)
        } else {
            self.elements.chunks_exact(self.width)
        }
    }
}

impl<T: Copy> Grid<T> {
    /// Returns an iterator over columns of the grid.
    pub fn columns(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        (0..self.width).map(|x| self.rows().map(|row| row[x]).collect::<Vec<_>>())
    }

    /// Returns all diagonals of the given grid.
    pub fn diagonals(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        (1..self.width + self.height).flat_map(move |i| {
            [
                // Down to the right.
                (0..self.height)
                    .filter_map(|j| self.get((i + j).checked_sub(self.height)?, j).copied())
                    .collect::<Vec<_>>(),
                // Down to the left.
                (0..self.height)
                    .filter_map(|j| self.get((i).checked_sub(j + 1)?, j).copied())
                    .collect::<Vec<_>>(),
            ]
        })
    }

    /// Returns a copy of the grid flipped vertically.
    #[allow(unused)]
    pub fn flip_vertical(&self) -> Self {
        let elements = self.rows().rev().flatten().copied().collect();
        Self {
            width: self.width,
            height: self.height,
            elements,
        }
    }

    /// Returns a copy of the grid flipped horizontally.
    pub fn flip_horizonal(&self) -> Self {
        let elements = self
            .rows()
            .flat_map(|row| row.iter().rev())
            .copied()
            .collect();
        Self {
            width: self.width,
            height: self.height,
            elements,
        }
    }

    /// Returns a copy of the grid rotated 90° clockwise.
    pub fn rotate_clockwise(&self) -> Self {
        let elements = (0..self.width)
            .flat_map(|new_y| {
                (0..self.height)
                    .map(move |new_x| self.get(new_y, self.height() - new_x - 1).unwrap())
            })
            .copied()
            .collect();
        Self {
            width: self.height,
            height: self.width,
            elements,
        }
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[")?;
        for y in 0..self.height {
            if y != 0 {
                write!(f, ", ")?;
            }
            write!(
                f,
                "{:?}",
                &self.elements[y * self.width..(y + 1) * self.width],
            )?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T: Default> Grid<T> {
    /// Creates a new empty grid of the given size.
    #[allow(unused)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            elements: repeat_with(|| Default::default())
                .take(width * height)
                .collect(),
        }
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T> {
    type Error = Report;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let height = value.len();
        let width = value.first().map(Vec::len).unwrap_or_default();
        for row in value.iter() {
            if row.len() != width {
                bail!(
                    "First row was {} elements long but another row is {} elements",
                    width,
                    row.len()
                );
            }
        }
        Ok(Self {
            width,
            height,
            elements: value.into_iter().flatten().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::charvec;

    #[test]
    fn size() {
        let grid: Grid<u32> = Grid::new(2, 3);

        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 3);
    }

    #[test]
    fn get_bounds() {
        let grid: Grid<u32> = Grid::new(2, 3);

        assert_eq!(grid.get(0, 0), Some(&0));
        assert_eq!(grid.get(1, 0), Some(&0));
        assert_eq!(grid.get(0, 2), Some(&0));
        assert_eq!(grid.get(1, 2), Some(&0));

        assert_eq!(grid.get(2, 0), None);
        assert_eq!(grid.get(0, 3), None);
        assert_eq!(grid.get(2, 3), None);
        assert_eq!(grid.get(20, 30), None);
    }

    #[test]
    fn from_vecvec() {
        assert_eq!(Grid::try_from(vec![]).unwrap(), Grid::<u32>::new(0, 0));

        let row_grid = Grid::try_from(vec![vec![1, 2, 3]]).unwrap();
        assert_eq!(row_grid.width(), 3);
        assert_eq!(row_grid.height(), 1);
        assert_eq!(row_grid.elements, Box::from([1, 2, 3]));

        let column_grid = Grid::try_from(vec![vec![1], vec![2], vec![3]]).unwrap();
        assert_eq!(column_grid.width(), 1);
        assert_eq!(column_grid.height(), 3);
        assert_eq!(column_grid.elements, Box::from([1, 2, 3]));

        let grid = Grid::try_from(vec![vec![11, 12], vec![21, 22], vec![31, 32]]).unwrap();
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid.elements, Box::from([11, 12, 21, 22, 31, 32]));

        assert!(Grid::try_from(vec![vec![], vec![1]]).is_err());
    }

    #[test]
    fn get_diagonals() {
        // abc
        // ABC
        let expected: Vec<Vec<char>> = vec![
            charvec("A"),
            charvec("a"),
            charvec("aB"),
            charvec("bA"),
            charvec("bC"),
            charvec("cB"),
            charvec("c"),
            charvec("C"),
        ];
        assert_eq!(
            Grid::try_from(vec![charvec("abc"), charvec("ABC")])
                .unwrap()
                .diagonals()
                .collect::<Vec<_>>(),
            expected
        );
        // abcd
        // ABCD
        let expected: Vec<Vec<char>> = vec![
            charvec("A"),
            charvec("a"),
            charvec("aB"),
            charvec("bA"),
            charvec("bC"),
            charvec("cB"),
            charvec("cD"),
            charvec("dC"),
            charvec("d"),
            charvec("D"),
        ];
        assert_eq!(
            Grid::try_from(vec![charvec("abcd"), charvec("ABCD")])
                .unwrap()
                .diagonals()
                .collect::<Vec<_>>(),
            expected
        );
        // aA
        // bB
        // cC
        let expected: Vec<Vec<char>> = vec![
            charvec("c"),
            charvec("a"),
            charvec("bC"),
            charvec("Ab"),
            charvec("aB"),
            charvec("Bc"),
            charvec("A"),
            charvec("C"),
        ];
        assert_eq!(
            Grid::try_from(vec![charvec("aA"), charvec("bB"), charvec("cC")])
                .unwrap()
                .diagonals()
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[test]
    fn rotate() {
        //  1  2  3
        // 10 20 30
        let grid = Grid::try_from(vec![vec![1, 2, 3], vec![10, 20, 30]]).unwrap();
        let expected = Grid::try_from(vec![vec![10, 1], vec![20, 2], vec![30, 3]]).unwrap();
        assert_eq!(grid.rotate_clockwise(), expected);
    }
}
