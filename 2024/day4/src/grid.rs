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

    /// Returns an iterator over rows of the grid.
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        if self.elements.is_empty() {
            // `chunks_exact` will panic if we pass 0, but it doesn't actually matter what we pass
            // as `elements` is empty anyway.
            self.elements.chunks_exact(1)
        } else {
            self.elements.chunks_exact(self.width)
        }
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[")?;
        for y in 0..self.height {
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
}
