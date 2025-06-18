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
///
/// * `x0` - The lower x-coordinate of the grid
/// * `x1` - The upper x-coordinate of the grid
///
/// * `y0` - The lower y-coordinate of the grid
/// * `y1` - The upper y-coordinate of the grid
///
/// * `q00` - The value at point (x0,y0)
/// * `q10` - The value at point (x1,y0)
/// * `q01` - The value at point (x0,y1)
/// * `q11` - The value at point (x1,y1)
///
/// # Returns
///
/// Returns the interpolated value at point (x,y).
/// If x0 equals x1 or y0 equals y1, it returns the linear interpolation along the non-degenerate axis.
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

/// Performs trilinear interpolation in a 3D rectangular grid.
///
/// Given a point (x,y,z), this function calculates the interpolated value using
/// the values at eight corners of a rectangular cuboid.
///
/// # Arguments
///
/// * `x` - The x-coordinate at which to interpolate
/// * `y` - The y-coordinate at which to interpolate
/// * `z` - The z-coordinate at which to interpolate
///
/// * `x0` - The lower x-coordinate of the grid
/// * `x1` - The upper x-coordinate of the grid
///
/// * `y0` - The lower y-coordinate of the grid
/// * `y1` - The upper y-coordinate of the grid
///
/// * `z0` - The lower z-coordinate of the grid
/// * `z1` - The upper z-coordinate of the grid
///
/// * `q000` - The value at point (x0,y0,z0)
/// * `q100` - The value at point (x1,y0,z0)
/// * `q010` - The value at point (x0,y1,z0)
/// * `q110` - The value at point (x1,y1,z0)
/// * `q001` - The value at point (x0,y0,z1)
/// * `q101` - The value at point (x1,y0,z1)
/// * `q011` - The value at point (x0,y1,z1)
/// * `q111` - The value at point (x1,y1,z1)
///
/// # Returns
///
/// Returns the interpolated value at point (x,y,z).
/// If any axis is degenerate, it falls back to bilinear or linear interpolation as appropriate.
///
/// # Example
///
/// ```
/// use rad_tools_world::interp::trilinear;
/// let value: f64 = trilinear(1.5, 1.5, 1.5, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 10.0, 20.0, 20.0, 40.0, 20.0, 40.0, 40.0, 80.0);
/// assert!((value - 33.75).abs() < 0.001);
///
/// let value: f64 = trilinear(1.2, 1.2, 1.2, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 10.0, 20.0, 20.0, 40.0, 20.0, 40.0, 40.0, 80.0);
/// assert!((value - 17.28).abs() < 0.001);
///
/// let value: f64 = trilinear(1.8, 1.8, 1.8, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 10.0, 20.0, 20.0, 40.0, 20.0, 40.0, 40.0, 80.0);
/// assert!((value - 58.32).abs() < 0.001);
/// ```
#[allow(clippy::too_many_arguments)]
pub fn trilinear<T>(
    x: T,
    y: T,
    z: T,
    x0: T,
    x1: T,
    y0: T,
    y1: T,
    z0: T,
    z1: T,
    q000: T,
    q100: T,
    q010: T,
    q110: T,
    q001: T,
    q101: T,
    q011: T,
    q111: T,
) -> T
where
    T: Float,
{
    let dx = x1 - x0;
    let dy = y1 - y0;
    let dz = z1 - z0;

    if dx == T::zero() && dy == T::zero() && dz == T::zero() {
        return q000;
    }

    if dx == T::zero() && dy == T::zero() {
        return linear(z, z0, z1, q000, q001);
    }

    if dx == T::zero() && dz == T::zero() {
        return linear(y, y0, y1, q000, q010);
    }

    if dy == T::zero() && dz == T::zero() {
        return linear(x, x0, x1, q000, q100);
    }

    if dx == T::zero() {
        return bilinear(y, z, y0, y1, z0, z1, q000, q010, q001, q011);
    }

    if dy == T::zero() {
        return bilinear(x, z, x0, x1, z0, z1, q000, q100, q001, q101);
    }

    if dz == T::zero() {
        return bilinear(x, y, x0, x1, y0, y1, q000, q100, q010, q110);
    }

    let tx = (x - x0) / dx;
    let ty = (y - y0) / dy;
    let tz = (z - z0) / dz;

    let c00 = q000 + (q100 - q000) * tx;
    let c10 = q010 + (q110 - q010) * tx;
    let c01 = q001 + (q101 - q001) * tx;
    let c11 = q011 + (q111 - q011) * tx;

    let c0 = c00 + (c10 - c00) * ty;
    let c1 = c01 + (c11 - c01) * ty;

    c0 + (c1 - c0) * tz
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

    #[test]
    fn test_trilinear_normal() {
        let value = trilinear(
            1.5, 1.5, 1.5, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 10.0, 20.0, 20.0, 40.0, 20.0, 40.0, 40.0,
            80.0,
        );

        assert!((value - 33.75).abs() < 0.001);
        let value = trilinear(
            1.2, 1.2, 1.2, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 10.0, 20.0, 20.0, 40.0, 20.0, 40.0, 40.0,
            80.0,
        );
        assert!((value - 17.28).abs() < 0.001);

        let value = trilinear(
            1.8, 1.8, 1.8, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 10.0, 20.0, 20.0, 40.0, 20.0, 40.0, 40.0,
            80.0,
        );
        assert!((value - 58.32).abs() < 0.001);
    }

    #[test]
    fn test_trilinear_degenerate_cases() {
        // Test when all dimensions are degenerate
        assert_eq!(
            trilinear(
                1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0,
                10.0, 10.0
            ),
            10.0
        );

        // Test when x dimension is degenerate
        assert_eq!(
            trilinear(
                1.0, 1.5, 1.5, 1.0, 1.0, 1.0, 2.0, 1.0, 2.0, 10.0, 10.0, 20.0, 20.0, 20.0, 20.0,
                40.0, 40.0
            ),
            22.5
        );

        // Test when y dimension is degenerate
        assert_eq!(
            trilinear(
                1.5, 1.0, 1.5, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 10.0, 20.0, 10.0, 20.0, 20.0, 40.0,
                20.0, 40.0
            ),
            22.5
        );

        // Test when z dimension is degenerate
        assert_eq!(
            trilinear(
                1.5, 1.5, 1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 1.0, 10.0, 20.0, 20.0, 40.0, 10.0, 20.0,
                20.0, 40.0
            ),
            22.5
        );
    }

    #[test]
    fn test_trilinear_corners() {
        let x0 = 1.0;
        let x1 = 2.0;
        let y0 = 1.0;
        let y1 = 2.0;
        let z0 = 1.0;
        let z1 = 2.0;
        let q000 = 10.0;
        let q100 = 20.0;
        let q010 = 20.0;
        let q110 = 40.0;
        let q001 = 20.0;
        let q101 = 40.0;
        let q011 = 40.0;
        let q111 = 80.0;

        // Test all eight corners
        assert_eq!(
            trilinear(
                x0, y0, z0, x0, x1, y0, y1, z0, z1, q000, q100, q010, q110, q001, q101, q011, q111
            ),
            q000
        );
        assert_eq!(
            trilinear(
                x1, y0, z0, x0, x1, y0, y1, z0, z1, q000, q100, q010, q110, q001, q101, q011, q111
            ),
            q100
        );
        assert_eq!(
            trilinear(
                x0, y1, z0, x0, x1, y0, y1, z0, z1, q000, q100, q010, q110, q001, q101, q011, q111
            ),
            q010
        );
        assert_eq!(
            trilinear(
                x1, y1, z0, x0, x1, y0, y1, z0, z1, q000, q100, q010, q110, q001, q101, q011, q111
            ),
            q110
        );
        assert_eq!(
            trilinear(
                x0, y0, z1, x0, x1, y0, y1, z0, z1, q000, q100, q010, q110, q001, q101, q011, q111
            ),
            q001
        );
        assert_eq!(
            trilinear(
                x1, y0, z1, x0, x1, y0, y1, z0, z1, q000, q100, q010, q110, q001, q101, q011, q111
            ),
            q101
        );
        assert_eq!(
            trilinear(
                x0, y1, z1, x0, x1, y0, y1, z0, z1, q000, q100, q010, q110, q001, q101, q011, q111
            ),
            q011
        );
        assert_eq!(
            trilinear(
                x1, y1, z1, x0, x1, y0, y1, z0, z1, q000, q100, q010, q110, q001, q101, q011, q111
            ),
            q111
        );
    }
}
