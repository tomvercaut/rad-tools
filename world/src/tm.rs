use nalgebra::{Matrix4, Point3};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransformError {
    #[error("Transform is not invertible")]
    NonInvertibleTransform,
}

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
    pub fn new(matrix: Matrix4<f64>) -> Result<Self, TransformError> {
        let inverse = matrix
            .try_inverse()
            .ok_or(TransformError::NonInvertibleTransform)?;

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
    pub fn with_scale(x: f64, y: f64, z: f64) -> Result<Self, TransformError> {
        if x.abs() < f64::EPSILON || y.abs() < f64::EPSILON || z.abs() < f64::EPSILON {
            return Err(TransformError::NonInvertibleTransform);
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

    /// Apply transform
    pub fn apply(&self, point: &Point3<f64>) -> Point3<f64> {
        self.matrix.transform_point(point)
    }

    /// Apply inverse transform
    pub fn apply_inverse(&self, x: f64, y: f64, z: f64) -> Point3<f64> {
        let coords = Point3::new(x, y, z);
        self.inverse.transform_point(&coords)
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
        let coords = transform.apply(&Point3::new(5f64, 6f64, 7f64));
        assert_eq!(coords.x, 15.0); // 5 + 10
        assert_eq!(coords.y, 26.0); // 6 + 20
        assert_eq!(coords.z, 37.0); // 7 + 30

        // Test coordinates to indices
        let indices = transform.apply_inverse(coords.x, coords.y, coords.z);
        assert_eq!(indices, Point3::new(5f64, 6f64, 7f64));
    }

    #[test]
    fn test_i2c_identity() {
        let transform = Transform::identity();
        let coords = transform.apply(&Point3::new(1.0, 2.0, 3.0));
        assert_eq!(coords.x, 1.0);
        assert_eq!(coords.y, 2.0);
        assert_eq!(coords.z, 3.0);
    }

    #[test]
    fn test_i2c_with_scale() {
        let transform = Transform::with_scale(2.0, 3.0, 4.0).unwrap();
        let coords = transform.apply(&Point3::new(1.0, 2.0, 3.0));
        assert_eq!(coords.x, 2.0); // 1 * 2
        assert_eq!(coords.y, 6.0); // 2 * 3
        assert_eq!(coords.z, 12.0); // 3 * 4
    }

    #[test]
    fn test_i2c_with_translation() {
        let transform = Transform::with_translation(10.0, 20.0, 30.0);
        let coords = transform.apply(&Point3::new(1.0, 2.0, 3.0));
        assert_eq!(coords.x, 11.0); // 1 + 10
        assert_eq!(coords.y, 22.0); // 2 + 20
        assert_eq!(coords.z, 33.0); // 3 + 30
    }
}
