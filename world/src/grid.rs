use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GridError {
    #[error("Index [{0}] out of bounds: {1} >= {2}")]
    IndexOutOfBounds(isize, isize, isize),
}

fn calc_size<const N: usize>(dims: &[isize; N]) -> usize {
    let mut n: usize = 1;
    for i in 0..N {
        if dims[i] < 0 {
            panic!("Dimensions must be positive");
        }
        n *= dims[i] as usize;
    }
    n
}

#[derive(Debug, Clone)]
pub struct Grid<T: Copy + Debug, const N: usize> {
    dims: [isize; N],
    data: Vec<T>,
}

impl<T: Copy + Debug, const N: usize> Grid<T, N> {
    pub fn new(dims: [isize; N], default_value: T) -> Self {
        let size: usize = calc_size(&dims);
        let data = vec![default_value; size];
        Self { dims, data }
    }

    pub fn dims(&self) -> &[isize; N] {
        &self.dims
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn set(&mut self, index: &[isize; N], value: T) -> Result<(), GridError> {
        self.valid_indices(index)?;
        let x = self.linear_index(index)?;
        self.data[x] = value;
        Ok(())
    }

    pub fn get(&self, index: &[isize; N]) -> Result<T, GridError> {
        self.valid_indices(index)?;
        let x = self.linear_index(index)?;
        Ok(self.data[x])
    }

    pub fn get_ref(&self, index: &[isize; N]) -> Result<&T, GridError> {
        self.valid_indices(index)?;
        let x = self.linear_index(index)?;
        Ok(&self.data[x])
    }

    pub fn get_mut(&mut self, index: &[isize; N]) -> Result<&mut T, GridError> {
        self.valid_indices(index)?;
        let x = self.linear_index(index)?;
        Ok(&mut self.data[x])
    }

    fn valid_indices(&self, index: &[isize; N]) -> Result<(), GridError> {
        for i in 0..N {
            if index[i] >= self.dims[i] {
                return Err(GridError::IndexOutOfBounds(
                    i as isize,
                    index[i],
                    self.dims[i],
                ));
            }
        }
        Ok(())
    }

    fn linear_index(&self, index: &[isize; N]) -> Result<usize, GridError> {
        let mut x: usize = 0;
        for i in 0..N {
            let mut m: usize = 1;
            for j in i + 1..N {
                m *= self.dims[j] as usize;
            }
            x += m * index[i] as usize;
        }
        Ok(x)
    }
}

impl<T: Copy + Debug, const N: usize> Index<&[isize; N]> for Grid<T, N> {
    type Output = T;

    fn index(&self, index: &[isize; N]) -> &Self::Output {
        match self.get_ref(index) {
            Ok(value) => value,
            Err(e) => {
                panic!("Error reading Grid index {index:#?}: {e}");
            }
        }
    }
}

impl<T: Copy + Debug, const N: usize> IndexMut<&[isize; N]> for Grid<T, N> {
    fn index_mut(&mut self, index: &[isize; N]) -> &mut Self::Output {
        match self.get_mut(index) {
            Ok(value) => value,
            Err(e) => {
                panic!("Error reading Grid index {index:#?}: {e}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid: Grid<isize, 3> = Grid::new([3isize, 4isize, 5isize], 0);
        assert_eq!(grid.dims(), &[3, 4, 5]);

        // Test that all values are initialized to default
        for x in 0..3 {
            for y in 0..4 {
                for z in 0..5 {
                    assert_eq!(grid.get(&[x, y, z]).unwrap(), 0);
                }
            }
        }
    }

    #[test]
    fn test_grid_set_get() {
        let mut grid: Grid<isize, 3> = Grid::new([3isize, 4isize, 5isize], 0);

        // Set some values
        grid.set(&[1, 2, 3], 42).unwrap();
        grid.set(&[0, 0, 0], 10).unwrap();

        // Check the values
        assert_eq!(grid.get(&[1, 2, 3]).unwrap(), 42);
        assert_eq!(grid.get(&[0, 0, 0]).unwrap(), 10);
        assert_eq!(grid.get(&[2, 3, 4]).unwrap(), 0); // Default value

        // Test out of bounds
        assert!(grid.get(&[3, 0, 0]).is_err());
        assert!(grid.set(&[0, 4, 0], 100).is_err());
    }

    #[test]
    fn test_grid_indexing() {
        let mut grid: Grid<isize, 3> = Grid::new([3isize, 4isize, 5isize], 0);

        // Set using indexing
        grid[&[1, 2, 3]] = 42;
        grid[&[0, 0, 0]] = 10;

        // Get using indexing
        assert_eq!(grid[&[1, 2, 3]], 42);
        assert_eq!(grid[&[0, 0, 0]], 10);
        assert_eq!(grid[&[2, 3, 4]], 0); // Default value
    }

    #[test]
    fn test_valid_indices() {
        let grid: Grid<isize, 3> = Grid::new([3isize, 4isize, 5isize], 0);

        // Test valid indices
        assert!(grid.valid_indices(&[0, 0, 0]).is_ok());
        assert!(grid.valid_indices(&[2, 3, 4]).is_ok());
        assert!(grid.valid_indices(&[1, 2, 3]).is_ok());
    }

    #[test]
    fn test_invalid_indices() {
        let grid: Grid<isize, 3> = Grid::new([3isize, 4isize, 5isize], 0);

        // Test out-of-bounds indices
        assert!(grid.valid_indices(&[3, 0, 0]).is_err());
        assert!(grid.valid_indices(&[0, 4, 0]).is_err());
        assert!(grid.valid_indices(&[0, 0, 5]).is_err());
        assert!(grid.valid_indices(&[10, 10, 10]).is_err());
    }
}
