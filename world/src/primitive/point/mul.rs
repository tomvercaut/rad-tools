use crate::primitive::point::{Point2D, Point3D};

macro_rules! mul_2d {
    ($t1:ty, $output:ty) => {
        impl std::ops::Mul<Point2D<$t1>> for Point2D<$t1> {
            type Output = Point2D<$output>;

            fn mul(self, rhs: Point2D<$t1>) -> Self::Output {
                Point2D(
                    self.0 as $output * rhs.0 as $output,
                    self.1 as $output * rhs.1 as $output,
                )
            }
        }

        impl std::ops::Mul<$t1> for Point2D<$t1> {
            type Output = Point2D<$output>;

            fn mul(self, rhs: $t1) -> Self::Output {
                Point2D(
                    self.0 as $output * rhs as $output,
                    self.1 as $output * rhs as $output,
                )
            }
        }

        impl std::ops::Mul<Point2D<$t1>> for $t1 {
            type Output = Point2D<$output>;

            fn mul(self, rhs: Point2D<$t1>) -> Self::Output {
                Point2D(
                    self as $output * rhs.0 as $output,
                    self as $output * rhs.1 as $output,
                )
            }
        }

        impl std::ops::MulAssign<Point2D<$t1>> for Point2D<$t1> {
            fn mul_assign(&mut self, rhs: Point2D<$t1>) {
                self.0 *= rhs.0 as $t1;
                self.1 *= rhs.1 as $t1;
            }
        }

        impl std::ops::MulAssign<$t1> for Point2D<$t1> {
            fn mul_assign(&mut self, rhs: $t1) {
                self.0 *= rhs as $t1;
                self.1 *= rhs as $t1;
            }
        }
    };
}

mul_2d!(u8, u8);
mul_2d!(u16, u16);
mul_2d!(u32, u32);
mul_2d!(u64, u64);
mul_2d!(u128, u128);

mul_2d!(i8, i8);
mul_2d!(i16, i16);
mul_2d!(i32, i32);
mul_2d!(i64, i64);
mul_2d!(i128, i128);
mul_2d!(f32, f32);
mul_2d!(f64, f64);

macro_rules! mul_3d {
    ($t1:ty, $output:ty) => {
        impl std::ops::Mul<Point3D<$t1>> for Point3D<$t1> {
            type Output = Point3D<$output>;

            fn mul(self, rhs: Point3D<$t1>) -> Self::Output {
                Point3D(
                    self.0 as $output * rhs.0 as $output,
                    self.1 as $output * rhs.1 as $output,
                    self.2 as $output * rhs.2 as $output,
                )
            }
        }

        impl std::ops::Mul<$t1> for Point3D<$t1> {
            type Output = Point3D<$output>;

            fn mul(self, rhs: $t1) -> Self::Output {
                Point3D(
                    self.0 as $output * rhs as $output,
                    self.1 as $output * rhs as $output,
                    self.2 as $output * rhs as $output,
                )
            }
        }

        impl std::ops::Mul<Point3D<$t1>> for $t1 {
            type Output = Point3D<$output>;

            fn mul(self, rhs: Point3D<$t1>) -> Self::Output {
                Point3D(
                    self as $output * rhs.0 as $output,
                    self as $output * rhs.1 as $output,
                    self as $output * rhs.2 as $output,
                )
            }
        }

        impl std::ops::MulAssign<Point3D<$t1>> for Point3D<$t1> {
            fn mul_assign(&mut self, rhs: Point3D<$t1>) {
                self.0 *= rhs.0 as $t1;
                self.1 *= rhs.1 as $t1;
                self.2 *= rhs.2 as $t1;
            }
        }

        impl std::ops::MulAssign<$t1> for Point3D<$t1> {
            fn mul_assign(&mut self, rhs: $t1) {
                self.0 *= rhs as $t1;
                self.1 *= rhs as $t1;
                self.2 *= rhs as $t1;
            }
        }
    };
}

mul_3d!(u8, u8);
mul_3d!(u16, u16);
mul_3d!(u32, u32);
mul_3d!(u64, u64);
mul_3d!(u128, u128);

mul_3d!(i8, i8);
mul_3d!(i16, i16);
mul_3d!(i32, i32);
mul_3d!(i64, i64);
mul_3d!(i128, i128);
mul_3d!(f32, f32);
mul_3d!(f64, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_mul() {
        // Test integer multiplication
        let p1 = Point2D(1u8, 2u8);
        let p2 = Point2D(3u8, 4u8);
        assert_eq!(p1 * p2, Point2D(3u8, 8u8));

        let p1 = Point2D(2i32, 2i32);
        let p2 = Point2D(3i32, 4i32);
        assert_eq!(p1 * p2, Point2D(6i32, 8i32));

        // Test integer point mul
        let p1 = Point2D(2i32, 3i32);
        assert_eq!(p1 * 3, Point2D(6i32, 9i32));

        // Test floating point multiply
        let p1 = Point2D(2.0f32, 3.0f32);
        let p2 = Point2D(3.0f32, 4.0f32);
        assert_eq!(p1 * p2, Point2D(6.0f32, 12.0f32));

        // Test floating point mul_assign
        let p1 = Point2D(2.0f32, 3.0f32);
        assert_eq!(p1 * 3.0, Point2D(6.0f32, 9.0f32));

        // Test mul_assign
        let mut p1 = Point2D(2u8, 3u8);
        p1 *= Point2D(3u8, 4u8);
        assert_eq!(p1, Point2D(6u8, 12u8));

        let mut p1 = Point2D(2u8, 3u8);
        p1 *= 3;
        assert_eq!(p1, Point2D(6u8, 9u8));
    }

    #[test]
    fn test_point3d_mul() {
        // Test integer multiplication
        let p1 = Point3D(2u8, 3u8, 4u8);
        let p2 = Point3D(4u8, 5u8, 6u8);
        assert_eq!(p1 * p2, Point3D(8u8, 15u8, 24u8));

        let p1 = Point3D(2i32, 3i32, 4i32);
        let p2 = Point3D(4i32, 5i32, 6i32);
        assert_eq!(p1 * p2, Point3D(8i32, 15i32, 24i32));

        // Test integer point mul_assign
        let p1 = Point3D(2i32, 3i32, 4i32);
        assert_eq!(p1 * 3, Point3D(6i32, 9i32, 12i32));

        // Test floating point multipli
        let p1 = Point3D(2.0f32, 3.0f32, 4.0f32);
        let p2 = Point3D(4.0f32, 5.0f32, 6.0f32);
        assert_eq!(p1 * p2, Point3D(8.0f32, 15.0f32, 24.0f32));

        // Test floating point mul_assign
        let p1 = Point3D(2f32, 3f32, 4f32);
        assert_eq!(p1 * 3.0, Point3D(6f32, 9f32, 12f32));

        // Test mul_assign
        let mut p1 = Point3D(2u8, 3u8, 4u8);
        p1 *= Point3D(4u8, 5u8, 6u8);
        assert_eq!(p1, Point3D(8u8, 15u8, 24u8));

        // Test floating point mul_assign
        let mut p1 = Point3D(2u8, 3u8, 4u8);
        p1 *= 3;
        assert_eq!(p1, Point3D(6u8, 9u8, 12u8));
    }
}
