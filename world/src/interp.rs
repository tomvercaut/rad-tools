use num_traits::Float;

/// Performs linear interpolation between two points (x0,y0) and (x1,y1).
///
/// Given an x-coordinate, this function calculates the corresponding y-coordinate
/// on the line segment between two points using linear interpolation.
///
/// # Arguments
///
/// * `x` - The x-coordinate at which to interpolate
/// * `x0` - The x-coordinate of the first point
/// * `x1` - The x-coordinate of the second point
/// * `y0` - The y-coordinate of the first point
/// * `y1` - The y-coordinate of the second point
///
/// # Returns
///
/// Returns the interpolated y-value at point x.
/// If x0 equals x1, returns y0 to avoid division by zero.
///
/// # Example
///
/// ```
/// use rad_tools_world::interp::linear;
///
/// let result = linear(1.5f64, 1.0, 2.0, 10.0, 20.0);
/// assert_eq!(result, 15.0); // Midpoint between 10.0 and 20.0
///
/// // When x equals one of the endpoints
/// let result = linear(1.0f64, 1.0, 2.0, 10.0, 20.0);
/// assert_eq!(result, 10.0);
/// ```
pub fn linear<T>(x: T, x0: T, x1: T, y0: T, y1: T) -> T
where
    T: Float,
{
    let dx = x1 - x0;
    if dx == T::zero() {
        return y0;
    }
    y0 + (x - x0) * (y1 - y0) / dx
}

/// Performs bilinear interpolation on a rectangular grid.
///
/// Given a point (x,y), this function calculates the interpolated value using
/// the values at four corners of a rectangular grid.
///
/// # Arguments
///
/// * `x` - The x-coordinate at which to interpolate
/// * `y` - The y-coordinate at which to interpolate
/// * `x0` - The lower x-coordinate of the grid
/// * `x1` - The upper x-coordinate of the grid
/// * `y0` - The lower y-coordinate of the grid
/// * `y1` - The upper y-coordinate of the grid
/// * `q00` - The value at point (x0,y0)
/// * `q10` - The value at point (x1,y0)
/// * `q01` - The value at point (x0,y1)
/// * `q11` - The value at point (x1,y1)
///
/// # Returns
///
/// Returns the interpolated value at point (x,y).
/// If x0 equals x1 or y0 equals y1, returns the linear interpolation along the non-degenerate axis.
///
/// # Example
///
/// ```
/// use rad_tools_world::interp::bilinear;
///
/// let result = bilinear(1.5f64, 1.5, 1.0, 2.0, 1.0, 2.0, 10.0, 20.0, 20.0, 40.0);
/// assert_eq!(result, 22.5); // Interpolated value at (1.5, 1.5)
/// ```
#[allow(clippy::too_many_arguments)]
pub fn bilinear<T>(x: T, y: T, x0: T, x1: T, y0: T, y1: T, q00: T, q10: T, q01: T, q11: T) -> T
where
    T: Float,
{
    let dx = x1 - x0;
    let dy = y1 - y0;

    if dx == T::zero() && dy == T::zero() {
        return q00;
    }
    if dx == T::zero() {
        return linear(y, y0, y1, q00, q01);
    }
    if dy == T::zero() {
        return linear(x, x0, x1, q00, q10);
    }

    let tx = (x - x0) / dx;
    let ty = (y - y0) / dy;

    let p0 = q00 + (q10 - q00) * tx;
    let p1 = q01 + (q11 - q01) * tx;

    p0 + (p1 - p0) * ty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_interp() {
        assert_eq!(linear(1.5f64, 1.0, 2.0, 10.0, 20.0), 15.0);
        assert_eq!(linear(0.0f64, -1.0, 1.0, -10.0, 10.0), 0.0);
        assert_eq!(linear(2.0f64, 2.0, 2.0, 5.0, 10.0), 5.0);
    }

    #[test]
    fn test_bilinear_normal() {
        assert_eq!(
            bilinear(1.5f64, 1.5, 1.0, 2.0, 1.0, 2.0, 10.0, 20.0, 20.0, 40.0),
            22.5
        );
    }

    #[test]
    fn test_bilinear_degenerate_x() {
        assert_eq!(
            bilinear(1.0f64, 1.5, 1.0, 1.0, 1.0, 2.0, 10.0, 10.0, 20.0, 20.0),
            15.0
        );
    }

    #[test]
    fn test_bilinear_degenerate_y() {
        assert_eq!(
            bilinear(1.5f64, 1.0, 1.0, 2.0, 1.0, 1.0, 10.0, 20.0, 10.0, 20.0),
            15.0
        );
    }

    #[test]
    fn test_bilinear_degenerate_both() {
        assert_eq!(
            bilinear(1.0f64, 1.0, 1.0, 1.0, 1.0, 1.0, 10.0, 10.0, 10.0, 10.0),
            10.0
        );
    }

    #[test]
    fn test_bilinear_corners() {
        let x0 = 1.0f64;
        let x1 = 2.0f64;
        let y0 = 1.0f64;
        let y1 = 2.0f64;
        let q00 = 10.0f64;
        let q10 = 20.0f64;
        let q01 = 20.0f64;
        let q11 = 40.0f64;

        assert_eq!(bilinear(x0, y0, x0, x1, y0, y1, q00, q10, q01, q11), q00);
        assert_eq!(bilinear(x1, y0, x0, x1, y0, y1, q00, q10, q01, q11), q10);
        assert_eq!(bilinear(x0, y1, x0, x1, y0, y1, q00, q10, q01, q11), q01);
        assert_eq!(bilinear(x1, y1, x0, x1, y0, y1, q00, q10, q01, q11), q11);
    }

    #[test]
    fn test_bilinear() {
        assert!(
            (bilinear(14.5, 20.2, 14.0, 15.0, 20.0, 21.0, 91.0, 210.0, 162.0, 95.0) - 146.1)
                < 0.001
        );
        assert!(
            (bilinear(14.5, 20.0, 14.0, 15.0, 20.0, 20.0, 91.0, 210.0, 91.0, 210.0) - 150.5)
                < 0.001
        );
        assert!(
            (bilinear(14.0, 20.2, 14.0, 14.0, 20.0, 21.0, 91.0, 91.0, 162.0, 162.0) - 105.2)
                < 0.001
        );
    }
}
