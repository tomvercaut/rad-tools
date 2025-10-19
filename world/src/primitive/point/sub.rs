use crate::primitive::point::{Point2D, Point3D};

macro_rules! sub_2d {
    ($t1:ty, $output:ty) => {
        impl std::ops::Sub<Point2D<$t1>> for Point2D<$t1> {
            type Output = Point2D<$output>;

            fn sub(self, rhs: Point2D<$t1>) -> Self::Output {
                Point2D(
                    self.0 as $output - rhs.0 as $output,
                    self.1 as $output - rhs.1 as $output,
                )
            }
        }

        impl std::ops::Sub<$t1> for Point2D<$t1> {
            type Output = Point2D<$output>;

            fn sub(self, rhs: $t1) -> Self::Output {
                Point2D(
                    self.0 as $output - rhs as $output,
                    self.1 as $output - rhs as $output,
                )
            }
        }

        impl std::ops::Sub<Point2D<$t1>> for $t1 {
            type Output = Point2D<$output>;

            fn sub(self, rhs: Point2D<$t1>) -> Self::Output {
                Point2D(
                    self as $output - rhs.0 as $output,
                    self as $output - rhs.1 as $output,
                )
            }
        }

        impl std::ops::SubAssign<Point2D<$t1>> for Point2D<$t1> {
            fn sub_assign(&mut self, rhs: Point2D<$t1>) {
                self.0 -= rhs.0 as $t1;
                self.1 -= rhs.1 as $t1;
            }
        }

        impl std::ops::SubAssign<$t1> for Point2D<$t1> {
            fn sub_assign(&mut self, rhs: $t1) {
                self.0 -= rhs as $t1;
                self.1 -= rhs as $t1;
            }
        }
    };
}

sub_2d!(u8, u8);
sub_2d!(u16, u16);
sub_2d!(u32, u32);
sub_2d!(u64, u64);
sub_2d!(u128, u128);

sub_2d!(i8, i8);
sub_2d!(i16, i16);
sub_2d!(i32, i32);
sub_2d!(i64, i64);
sub_2d!(i128, i128);
sub_2d!(f32, f32);
sub_2d!(f64, f64);

macro_rules! sub_3d {
    ($t1:ty, $output:ty) => {
        impl std::ops::Sub<Point3D<$t1>> for Point3D<$t1> {
            type Output = Point3D<$output>;

            fn sub(self, rhs: Point3D<$t1>) -> Self::Output {
                Point3D(
                    self.0 as $output - rhs.0 as $output,
                    self.1 as $output - rhs.1 as $output,
                    self.2 as $output - rhs.2 as $output,
                )
            }
        }

        impl std::ops::Sub<$t1> for Point3D<$t1> {
            type Output = Point3D<$output>;

            fn sub(self, rhs: $t1) -> Self::Output {
                Point3D(
                    self.0 as $output - rhs as $output,
                    self.1 as $output - rhs as $output,
                    self.2 as $output - rhs as $output,
                )
            }
        }

        impl std::ops::Sub<Point3D<$t1>> for $t1 {
            type Output = Point3D<$output>;

            fn sub(self, rhs: Point3D<$t1>) -> Self::Output {
                Point3D(
                    self as $output - rhs.0 as $output,
                    self as $output - rhs.1 as $output,
                    self as $output - rhs.2 as $output,
                )
            }
        }

        impl std::ops::SubAssign<Point3D<$t1>> for Point3D<$t1> {
            fn sub_assign(&mut self, rhs: Point3D<$t1>) {
                self.0 -= rhs.0 as $t1;
                self.1 -= rhs.1 as $t1;
                self.2 -= rhs.2 as $t1;
            }
        }

        impl std::ops::SubAssign<$t1> for Point3D<$t1> {
            fn sub_assign(&mut self, rhs: $t1) {
                self.0 -= rhs as $t1;
                self.1 -= rhs as $t1;
                self.2 -= rhs as $t1;
            }
        }
    };
}

sub_3d!(u8, u8);
sub_3d!(u16, u16);
sub_3d!(u32, u32);
sub_3d!(u64, u64);
sub_3d!(u128, u128);

sub_3d!(i8, i8);
sub_3d!(i16, i16);
sub_3d!(i32, i32);
sub_3d!(i64, i64);
sub_3d!(i128, i128);
sub_3d!(f32, f32);
sub_3d!(f64, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_subtraction() {
        // Test integer subtraction
        let p1 = Point2D(3u8, 5u8);
        let p2 = Point2D(1u8, 2u8);
        assert_eq!(p1 - p2, Point2D(2u8, 3u8));

        let p1 = Point2D(3i32, 5i32);
        let p2 = Point2D(1i32, 2i32);
        assert_eq!(p1 - p2, Point2D(2i32, 3i32));

        // Test integer point sub_assign
        let p1 = Point2D(1i32, 2i32);
        assert_eq!(p1 - 3, Point2D(-2i32, -1i32));

        // Test floating point subtraction
        let p1 = Point2D(1.0f32, 2.0f32);
        let p2 = Point2D(3.0f32, 5.0f32);
        assert_eq!(p1 - p2, Point2D(-2.0f32, -3.0f32));

        // Test floating point sub_assign
        let p1 = Point2D(1.0f32, 2.0f32);
        assert_eq!(p1 - 3.0, Point2D(-2.0f32, -1.0f32));

        // Test sub_assign
        let mut p1 = Point2D(3u8, 5u8);
        p1 -= Point2D(1u8, 4u8);
        assert_eq!(p1, Point2D(2u8, 1u8));

        let mut p1 = Point2D(5u8, 6u8);
        p1 -= 3;
        assert_eq!(p1, Point2D(2u8, 3u8));
    }

    #[test]
    fn test_point3d_subtraction() {
        // Test integer subtraction
        let p1 = Point3D(5u8, 6u8, 7u8);
        let p2 = Point3D(1u8, 3u8, 6u8);
        assert_eq!(p1 - p2, Point3D(4u8, 3u8, 1u8));

        let p1 = Point3D(1i32, 2i32, 3i32);
        let p2 = Point3D(3i32, 5i32, 4i32);
        assert_eq!(p1 - p2, Point3D(-2i32, -3i32, -1i32));

        // Test integer point sub_assign
        let p1 = Point3D(1i32, 2i32, 3i32);
        assert_eq!(p1 - 3, Point3D(-2i32, -1i32, 0i32));

        // Test floating point subtraction
        let p1 = Point3D(1.0f32, 2.0f32, 3.0f32);
        let p2 = Point3D(3.0f32, 5.0f32, 4.0f32);
        assert_eq!(p1 - p2, Point3D(-2.0f32, -3.0f32, -1.0f32));

        // Test floating point sub_assign
        let p1 = Point3D(1f32, 2f32, 3f32);
        assert_eq!(p1 - 3.0, Point3D(-2f32, -1f32, 0f32));

        // Test sub_assign
        let mut p1 = Point3D(4u8, 7u8, 7u8);
        p1 -= Point3D(3u8, 5u8, 4u8);
        assert_eq!(p1, Point3D(1u8, 2u8, 3u8));

        // Test floating point sub_assign
        let mut p1 = Point3D(5u8, 6u8, 3u8);
        p1 -= 3;
        assert_eq!(p1, Point3D(2u8, 3u8, 0u8));
    }
}
