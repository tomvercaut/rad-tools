// pub trait Point2D<C, T>: Copy + Clone + Debug + PartialEq {
//     fn x(&self) -> C;
//     fn set_x(&mut self, x: C);
//     fn y(&self) -> C;
//     fn set_y(&mut self, y: C);
//     fn value(&self) -> T;
//     fn set_value(&mut self, value: T);
// }
//
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct OctaviusPoint {
//     x: f64,
//     y: f64,
//     value: f64,
// }
//
// impl Point2D<f64, f64> for OctaviusPoint {
//     fn x(&self) -> f64 {
//         self.x
//     }
//
//     fn set_x(&mut self, x: f64) {
//         self.x = x;
//     }
//
//     fn y(&self) -> f64 {
//         self.y
//     }
//
//     fn set_y(&mut self, y: f64) {
//         self.y = y;
//     }
//
//     fn value(&self) -> f64 {
//         self.value
//     }
//
//     fn set_value(&mut self, value: f64) {
//         self.value = value;
//     }
// }
//
// pub fn bilinear_p<P>(p11: P, p12: P, p21: P, p22: P, p: P) -> P where P: Point2D<f64, f64> {
//     let x1 = p11.x();
//     let x2 = p21.x();
//     let y1 = p11.y();
//     let y2 = p12.y();
//
//     let dx = x2 - x1;
//     let dy = y2 - y1;
//
//     let x = p.x();
//     let y = p.y();
//
//     let d = 1.0 / (dx * dy);
//     let x2_minus_x = x2 - x;
//     let y2_minus_y = y2 - y;
//     let x_minus_x1 = x - x1;
//     let y_minus_y1 = y - y1;
//
//     let w11 = x2_minus_x * y2_minus_y * d;
//     let w12 = x2_minus_x * y_minus_y1 * d;
//     let w21 = x_minus_x1 * y2_minus_y * d;
//     let w22 = x_minus_x1 * y_minus_y1 * d;
//
//     let value = w11 * p11.value() + w12 * p12.value() + w21 * p21.value() + w22 * p22.value();
//     let mut r = p;
//     r.set_value(value);
//     r
// }

/// Function to interpolate between two variables x and y using linear interpolations.
///
/// More details on the method can be found on: https://en.wikipedia.org/wiki/Bilinear_interpolation
///
/// Edge case:
/// * x2-x1 < std::f64::EPSICON and y2-y1 < std::f64::EPSICON: q11 is returned
/// * x2-x1 < std::f64::EPSICON: linear interpolation is done in y
/// * y2-y1 < std::f64::EPSICON: linear interpolation is done in x
///
/// # Arguments
///
/// * `x`: variable
/// * `y`: variable
/// * `x1`: first coordinate in x
/// * `x2`: second coordinate in x
/// * `y1`: first coordinate in y
/// * `y2`: second coordinate in y
/// * `q11`: value at (x1, y1)
/// * `q21`: value at (x2, y1)
/// * `q12`: value at (x1, y2)
/// * `q22`: value at (x2, y2)
///
/// returns: f64 value at (x,y)
pub fn bilinear(
    x: f64,
    y: f64,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    q11: f64,
    q21: f64,
    q12: f64,
    q22: f64,
) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;

    let c1 = dx < 2.0 * f64::EPSILON;
    let c2 = dy < 2.0 * f64::EPSILON;
    if c1 && c2 {
        q11
    } else if c1 {
        linear(y, y1, y2, q11, q12)
    } else if c2 {
        linear(x, x1, x2, q11, q21)
    } else {
        let d = 1.0 / (dx * dy);
        let x2_minus_x = x2 - x;
        let y2_minus_y = y2 - y;
        let x_minus_x1 = x - x1;
        let y_minus_y1 = y - y1;

        let w11 = x2_minus_x * y2_minus_y * d;
        let w12 = x2_minus_x * y_minus_y1 * d;
        let w21 = x_minus_x1 * y2_minus_y * d;
        let w22 = x_minus_x1 * y_minus_y1 * d;

        let value = w11 * q11 + w12 * q12 + w21 * q21 + w22 * q22;
        value
    }
}

/// Linear interpolation of one variable.
///
/// # Arguments
///
/// * `x`: variable
/// * `x1`: first coordinate in x
/// * `x2`: second coordinate in y
/// * `y1`: value at x1
/// * `y2`: value at x2
///
/// returns: f64 value at x
pub fn linear(x: f64, x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    let d = x2 - x1;
    if d < 2.0 * f64::EPSILON {
        y1
    } else {
        y1 + (x - x1) * (y2 - y1) / (x2 - x1)
    }
}
