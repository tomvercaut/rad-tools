use crate::primative::point::{Point2D, Point3D};

macro_rules! div_2d {
    ($t1:ty, $output:ty) => {
        impl std::ops::Div<Point2D<$t1>> for Point2D<$t1> {
            type Output = Point2D<$output>;

            fn div(self, rhs: Point2D<$t1>) -> Self::Output {
                Point2D(
                    self.0 as $output / rhs.0 as $output,
                    self.1 as $output / rhs.1 as $output,
                )
            }
        }

        impl std::ops::Div<$t1> for Point2D<$t1> {
            type Output = Point2D<$output>;

            fn div(self, rhs: $t1) -> Self::Output {
                Point2D(
                    self.0 as $output / rhs as $output,
                    self.1 as $output / rhs as $output,
                )
            }
        }

        impl std::ops::Div<Point2D<$t1>> for $t1 {
            type Output = Point2D<$output>;

            fn div(self, rhs: Point2D<$t1>) -> Self::Output {
                Point2D(
                    self as $output / rhs.0 as $output,
                    self as $output / rhs.1 as $output,
                )
            }
        }
    };
}

macro_rules! div_assign_2d {
    ($t1:ty, $output:ty) => {
        impl std::ops::DivAssign<Point2D<$t1>> for Point2D<$t1> {
            fn div_assign(&mut self, rhs: Point2D<$t1>) {
                self.0 /= rhs.0 as $t1;
                self.1 /= rhs.1 as $t1;
            }
        }

        impl std::ops::DivAssign<$t1> for Point2D<$t1> {
            fn div_assign(&mut self, rhs: $t1) {
                self.0 /= rhs as $t1;
                self.1 /= rhs as $t1;
            }
        }
    };
}

div_2d!(u8, f64);
div_2d!(u16, f64);
div_2d!(u32, f64);
div_2d!(u64, f64);

div_2d!(i8, f64);
div_2d!(i16, f64);
div_2d!(i32, f64);
div_2d!(i64, f64);
div_2d!(f32, f64);
div_2d!(f64, f64);
div_assign_2d!(f64, f64);

macro_rules! div_3d {
    ($t1:ty, $output:ty) => {
        impl std::ops::Div<Point3D<$t1>> for Point3D<$t1> {
            type Output = Point3D<$output>;

            fn div(self, rhs: Point3D<$t1>) -> Self::Output {
                Point3D(
                    self.0 as $output / rhs.0 as $output,
                    self.1 as $output / rhs.1 as $output,
                    self.2 as $output / rhs.2 as $output,
                )
            }
        }

        impl std::ops::Div<$t1> for Point3D<$t1> {
            type Output = Point3D<$output>;

            fn div(self, rhs: $t1) -> Self::Output {
                Point3D(
                    self.0 as $output / rhs as $output,
                    self.1 as $output / rhs as $output,
                    self.2 as $output / rhs as $output,
                )
            }
        }

        impl std::ops::Div<Point3D<$t1>> for $t1 {
            type Output = Point3D<$output>;

            fn div(self, rhs: Point3D<$t1>) -> Self::Output {
                Point3D(
                    self as $output / rhs.0 as $output,
                    self as $output / rhs.1 as $output,
                    self as $output / rhs.2 as $output,
                )
            }
        }
    };
}

macro_rules! div_assign_3d {
    ($t1:ty, $output:ty) => {
        impl std::ops::DivAssign<Point3D<$t1>> for Point3D<$t1> {
            fn div_assign(&mut self, rhs: Point3D<$t1>) {
                self.0 /= rhs.0 as $t1;
                self.1 /= rhs.1 as $t1;
                self.2 /= rhs.2 as $t1;
            }
        }

        impl std::ops::DivAssign<$t1> for Point3D<$t1> {
            fn div_assign(&mut self, rhs: $t1) {
                self.0 /= rhs as $t1;
                self.1 /= rhs as $t1;
                self.2 /= rhs as $t1;
            }
        }
    };
}

div_3d!(u8, f64);
div_3d!(u16, f64);
div_3d!(u32, f64);
div_3d!(u64, f64);

div_3d!(i8, f64);
div_3d!(i16, f64);
div_3d!(i32, f64);
div_3d!(i64, f64);
div_3d!(f32, f64);
div_3d!(f64, f64);
div_assign_3d!(f64, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_div() {
        // Test integer division
        let p1 = Point2D(6u8, 4u8);
        let p2 = Point2D(2u8, 2u8);
        assert_eq!(p1 / p2, Point2D(3.0f64, 2.0f64));

        let p1 = Point2D(6i32, 4i32);
        let p2 = Point2D(2i32, 2i32);
        assert_eq!(p1 / p2, Point2D(3.0f64, 2.0f64));

        // Test integer point div_assign
        let p1 = Point2D(6i32, 9i32);
        assert_eq!(p1 / 3, Point2D(2f64, 3f64));

        // Test floating point division
        let p1 = Point2D(6.0f32, 12.0f32);
        let p2 = Point2D(3.0f32, 4.0f32);
        assert_eq!(p1 / p2, Point2D(2.0f64, 3.0f64));

        // Test floating point div_assign
        let p1 = Point2D(6.0f32, 9.0f32);
        assert_eq!(p1 / 3.0, Point2D(2.0f64, 3.0f64));

        // Test div_assign
        let mut p1 = Point2D(6f64, 9f64);
        p1 /= Point2D(2f64, 3f64);
        assert_eq!(p1, Point2D(3f64, 3f64));

        let mut p1 = Point2D(6f64, 9f64);
        p1 /= 3f64;
        assert_eq!(p1, Point2D(2f64, 3f64));
    }

    #[test]
    fn test_point3d_div() {
        // Test integer devision
        let p1 = Point3D(6u8, 9u8, 24u8);
        let p2 = Point3D(2u8, 3u8, 6u8);
        assert_eq!(p1 / p2, Point3D(3f64, 3f64, 4f64));

        let p1 = Point3D(6i32, 9i32, 24i32);
        let p2 = Point3D(2i32, 3i32, 6i32);
        assert_eq!(p1 / p2, Point3D(3f64, 3f64, 4f64));

        // Test integer point div_assign
        let p1 = Point3D(6f64, 9f64, 24f64);
        assert_eq!(p1 / 3f64, Point3D(2f64, 3f64, 8f64));

        // Test floating point division
        let p1 = Point3D(6.0f64, 9.0f64, 24.0f64);
        let p2 = Point3D(2.0f64, 3.0f64, 6.0f64);
        assert_eq!(p1 / p2, Point3D(3.0f64, 3.0f64, 4.0f64));

        // Test floating point div_assign
        let p1 = Point3D(6f64, 9f64, 24f64);
        assert_eq!(p1 / 3.0, Point3D(2f64, 3f64, 8f64));

        // Test div_assign
        let mut p1 = Point3D(6f64, 9f64, 24f64);
        p1 /= Point3D(2f64, 3f64, 6f64);
        assert_eq!(p1, Point3D(3f64, 3f64, 4f64));

        // Test floating point div_assign
        let mut p1 = Point3D(6f64, 9f64, 24f64);
        p1 /= 3f64;
        assert_eq!(p1, Point3D(2f64, 3f64, 8f64));
    }
}
