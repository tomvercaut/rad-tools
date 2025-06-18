use crate::GridError;
use nalgebra::{Matrix4, Point3};

/// A transform for converting between grid indices and world coordinates.
#[derive(Debug, Clone)]
pub struct Transform {
    /// The 4x4 transformation matrix
    matrix: Matrix4<f64>,
    /// The inverse of the transformation matrix
    inverse: Matrix4<f64>,
}

impl Transform {
    /// Create a new transform with the given 4x4 matrix.
    ///
    /// Returns an error if the matrix is not invertible.
    pub fn new(matrix: Matrix4<f64>) -> Result<Self, GridError> {
        let inverse = matrix
            .try_inverse()
            .ok_or(GridError::NonInvertibleTransform)?;

        Ok(Self { matrix, inverse })
    }

    /// Create a new identity transform.
    pub fn identity() -> Self {
        let matrix = Matrix4::identity();
        Self {
            matrix,
            inverse: matrix, // Identity matrix is its own inverse
        }
    }

    /// Create a new transform with a translation.
    pub fn with_translation(x: f64, y: f64, z: f64) -> Self {
        let mut matrix = Matrix4::identity();
        matrix[(0, 3)] = x;
        matrix[(1, 3)] = y;
        matrix[(2, 3)] = z;

        // For a pure translation, the inverse is just the negative translation
        let mut inverse = Matrix4::identity();
        inverse[(0, 3)] = -x;
        inverse[(1, 3)] = -y;
        inverse[(2, 3)] = -z;

        Self { matrix, inverse }
    }

    /// Create a new transform with a scale.
    pub fn with_scale(x: f64, y: f64, z: f64) -> Result<Self, GridError> {
        if x.abs() < f64::EPSILON || y.abs() < f64::EPSILON || z.abs() < f64::EPSILON {
            return Err(GridError::NonInvertibleTransform);
        }

        let mut matrix = Matrix4::identity();
        matrix[(0, 0)] = x;
        matrix[(1, 1)] = y;
        matrix[(2, 2)] = z;

        let mut inverse = Matrix4::identity();
        inverse[(0, 0)] = 1.0 / x;
        inverse[(1, 1)] = 1.0 / y;
        inverse[(2, 2)] = 1.0 / z;

        Ok(Self { matrix, inverse })
    }

    /// Convert grid indices to world coordinates.
    pub fn indices_to_coordinates(&self, indices: (usize, usize, usize)) -> Point3<f64> {
        let point = Point3::new(indices.0 as f64, indices.1 as f64, indices.2 as f64);

        self.matrix.transform_point(&point)
    }

    /// Convert world coordinates to grid indices.
    ///
    /// Returns an error if the resulting indices are out of bounds.
    pub fn coordinates_to_indices(
        &self,
        coords: Point3<f64>,
    ) -> Result<(usize, usize, usize), GridError> {
        let point = self.inverse.transform_point(&coords);

        // Round to the nearest integer and convert to usize
        let x = point.x.round();
        let y = point.y.round();
        let z = point.z.round();

        // Check if the values are negative
        if x < 0.0 || y < 0.0 || z < 0.0 {
            return Err(GridError::InvalidCoordinateConversion(format!(
                "Negative indices: ({}, {}, {})",
                x, y, z
            )));
        }

        Ok((x as usize, y as usize, z as usize))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        // Create a transform with translation
        let transform = Transform::with_translation(10.0, 20.0, 30.0);

        // Test indices to coordinates
        let coords = transform.indices_to_coordinates((5, 6, 7));
        assert_eq!(coords.x, 15.0); // 5 + 10
        assert_eq!(coords.y, 26.0); // 6 + 20
        assert_eq!(coords.z, 37.0); // 7 + 30

        // Test coordinates to indices
        let indices = transform.coordinates_to_indices(coords).unwrap();
        assert_eq!(indices, (5, 6, 7));
    }
}
