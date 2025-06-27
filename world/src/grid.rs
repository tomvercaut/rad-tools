use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GridError {
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(String),
}

/// A 3D grid of numeric data with a transform for converting between indices and coordinates.
#[derive(Debug, Clone)]
pub struct Grid3D<T: Copy + Debug> {
    /// The 3D array of data
    data: Vec<T>,
    /// The dimensions of the grid (x, y, z)
    dimensions: (usize, usize, usize),
    /// Default value for uninitialized cells
    default_value: T,
}

impl<T: Copy + Debug> Grid3D<T> {
    /// Create a new 3D grid with the given dimensions and default value.
    pub fn new(dimensions: (usize, usize, usize), default_value: T) -> Self {
        let size = dimensions.0 * dimensions.1 * dimensions.2;
        let data = vec![default_value; size];

        Self {
            data,
            dimensions,
            default_value,
        }
    }

    /// Get the dimensions of the grid.
    pub fn dims(&self) -> (usize, usize, usize) {
        self.dimensions
    }

    /// Get the value at the given indices.
    ///
    /// Returns an error if the indices are out of bounds.
    pub fn get(&self, x: usize, y: usize, z: usize) -> Result<T, GridError> {
        let index = self.linear_index(x, y, z)?;
        Ok(self.data[index])
    }

    /// Set the value at the given indices.
    ///
    /// Returns an error if the indices are out of bounds.
    pub fn set(&mut self, x: usize, y: usize, z: usize, value: T) -> Result<(), GridError> {
        let index = self.linear_index(x, y, z)?;
        self.data[index] = value;
        Ok(())
    }

    /// Calculate the 1D index from 3D indices.
    fn linear_index(&self, x: usize, y: usize, z: usize) -> Result<usize, GridError> {
        self.valid_indices(x, y, z)?;
        Ok(x + y * self.dimensions.0 + z * self.dimensions.0 * self.dimensions.1)
    }

    /// Checks if the given indices are within the grid's dimensions.
    ///
    /// # Arguments
    /// * `x` - The x-axis index to check
    /// * `y` - The y-axis index to check
    /// * `z` - The z-axis index to check
    ///
    /// # Returns
    /// * `Ok(())` if the indices are valid
    /// * `Err(GridError::IndexOutOfBounds)` if any index is out of bounds
    fn valid_indices(&self, x: usize, y: usize, z: usize) -> Result<(), GridError> {
        if x >= self.dimensions.0 || y >= self.dimensions.1 || z >= self.dimensions.2 {
            return Err(GridError::IndexOutOfBounds(format!(
                "Indices ({x}, {y}, {z}) out of bounds for dimensions {:#?}",
                self.dimensions
            )));
        }
        Ok(())
    }

    /// Fill the entire grid with the given value.
    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
    }

    /// Reset the grid to the default value.
    pub fn reset(&mut self) {
        self.fill(self.default_value);
    }
}

impl<T: Copy + Debug> Index<(usize, usize, usize)> for Grid3D<T> {
    type Output = T;

    fn index(&self, indices: (usize, usize, usize)) -> &Self::Output {
        let (x, y, z) = indices;
        match self.linear_index(x, y, z) {
            Ok(index) => &self.data[index],
            Err(_) => {
                panic!(
                    "Index out of bounds: ({x}, {y}, {z}) for dimensions {:#?}",
                    self.dimensions
                );
            }
        }
    }
}

impl<T: Copy + Debug> IndexMut<(usize, usize, usize)> for Grid3D<T> {
    fn index_mut(&mut self, indices: (usize, usize, usize)) -> &mut Self::Output {
        let (x, y, z) = indices;
        match self.linear_index(x, y, z) {
            Ok(index) => &mut self.data[index],
            Err(_) => {
                panic!(
                    "Index out of bounds: ({x}, {y}, {z}) for dimensions {:#?}",
                    self.dimensions
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid3D::new((3, 4, 5), 0);
        assert_eq!(grid.dims(), (3, 4, 5));

        // Test that all values are initialized to default
        for x in 0..3 {
            for y in 0..4 {
                for z in 0..5 {
                    assert_eq!(grid.get(x, y, z).unwrap(), 0);
                }
            }
        }
    }

    #[test]
    fn test_grid_set_get() {
        let mut grid = Grid3D::new((3, 4, 5), 0);

        // Set some values
        grid.set(1, 2, 3, 42).unwrap();
        grid.set(0, 0, 0, 10).unwrap();

        // Check the values
        assert_eq!(grid.get(1, 2, 3).unwrap(), 42);
        assert_eq!(grid.get(0, 0, 0).unwrap(), 10);
        assert_eq!(grid.get(2, 3, 4).unwrap(), 0); // Default value

        // Test out of bounds
        assert!(grid.get(3, 0, 0).is_err());
        assert!(grid.set(0, 4, 0, 100).is_err());
    }

    #[test]
    fn test_grid_indexing() {
        let mut grid = Grid3D::new((3, 4, 5), 0);

        // Set using indexing
        grid[(1, 2, 3)] = 42;
        grid[(0, 0, 0)] = 10;

        // Get using indexing
        assert_eq!(grid[(1, 2, 3)], 42);
        assert_eq!(grid[(0, 0, 0)], 10);
        assert_eq!(grid[(2, 3, 4)], 0); // Default value
    }

    #[test]
    fn test_valid_indices() {
        let grid = Grid3D::new((3, 4, 5), 0);

        // Test valid indices
        assert!(grid.valid_indices(0, 0, 0).is_ok());
        assert!(grid.valid_indices(2, 3, 4).is_ok());
        assert!(grid.valid_indices(1, 2, 3).is_ok());
    }

    #[test]
    fn test_invalid_indices() {
        let grid = Grid3D::new((3, 4, 5), 0);

        // Test out-of-bounds indices
        assert!(grid.valid_indices(3, 0, 0).is_err());
        assert!(grid.valid_indices(0, 4, 0).is_err());
        assert!(grid.valid_indices(0, 0, 5).is_err());
        assert!(grid.valid_indices(10, 10, 10).is_err());
    }
}
