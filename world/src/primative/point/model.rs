/// A 2-dimensional point represented by x and y coordinates of type T
///
/// The type parameter T can be any numeric type (integer or floating point)
/// that supports the required traits for comparison and hashing
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2D<T>(pub T, pub T);

/// A 3-dimensional point represented by x, y and z coordinates of type T
///
/// The type parameter T can be any numeric type (integer or floating point)
/// that supports the required traits for comparison and hashing
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point3D<T>(pub T, pub T, pub T);

macro_rules! from_2d {
    ($t1:ty, $t2:ty) => {
        impl From<Point2D<$t1>> for Point2D<$t2> {
            fn from(p: Point2D<$t1>) -> Self {
                Point2D(p.0 as $t2, p.1 as $t2)
            }
        }
    };
}

from_2d!(u8, i16);
from_2d!(u8, i32);
from_2d!(u8, i64);
from_2d!(u8, f32);
from_2d!(u8, f64);

from_2d!(i8, i16);
from_2d!(i8, i32);
from_2d!(i8, i64);
from_2d!(i8, f32);
from_2d!(i8, f64);

from_2d!(u16, i32);
from_2d!(u16, i64);
from_2d!(u16, f32);
from_2d!(u16, f64);

from_2d!(i16, i32);
from_2d!(i16, i64);
from_2d!(i16, f32);
from_2d!(i16, f64);

from_2d!(u32, i64);
from_2d!(u32, f32);
from_2d!(u32, f64);

from_2d!(i32, i64);
from_2d!(i32, f32);
from_2d!(i32, f64);

from_2d!(u64, f32);
from_2d!(u64, f64);

from_2d!(i64, f32);
from_2d!(i64, f64);

macro_rules! from_3d {
    ($t1:ty, $t2:ty) => {
        impl From<Point3D<$t1>> for Point3D<$t2> {
            fn from(p: Point3D<$t1>) -> Self {
                Point3D(p.0 as $t2, p.1 as $t2, p.2 as $t2)
            }
        }
    };
}

from_3d!(u8, i16);
from_3d!(u8, i32);
from_3d!(u8, i64);
from_3d!(u8, f32);
from_3d!(u8, f64);

from_3d!(i8, i16);
from_3d!(i8, i32);
from_3d!(i8, i64);
from_3d!(i8, f32);
from_3d!(i8, f64);

from_3d!(u16, i32);
from_3d!(u16, i64);
from_3d!(u16, f32);
from_3d!(u16, f64);

from_3d!(i16, i32);
from_3d!(i16, i64);
from_3d!(i16, f32);
from_3d!(i16, f64);

from_3d!(u32, i64);
from_3d!(u32, f32);
from_3d!(u32, f64);

from_3d!(i32, i64);
from_3d!(i32, f32);
from_3d!(i32, f64);

from_3d!(u64, f32);
from_3d!(u64, f64);

from_3d!(i64, f32);
from_3d!(i64, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_casts() {
        assert_eq!(
            Point2D::<i16>::from(Point2D(5u8, 10u8)),
            Point2D(5i16, 10i16)
        );
        assert_eq!(
            Point2D::<i32>::from(Point2D(5u8, 10u8)),
            Point2D(5i32, 10i32)
        );
        assert_eq!(
            Point2D::<i64>::from(Point2D(5u8, 10u8)),
            Point2D(5i64, 10i64)
        );
        assert_eq!(
            Point2D::<f32>::from(Point2D(5u8, 10u8)),
            Point2D(5.0f32, 10.0f32)
        );
        assert_eq!(
            Point2D::<f64>::from(Point2D(5u8, 10u8)),
            Point2D(5.0f64, 10.0f64)
        );

        assert_eq!(
            Point2D::<i16>::from(Point2D(-5i8, -10i8)),
            Point2D(-5i16, -10i16)
        );
        assert_eq!(
            Point2D::<i32>::from(Point2D(-5i8, -10i8)),
            Point2D(-5i32, -10i32)
        );
        assert_eq!(
            Point2D::<i64>::from(Point2D(-5i8, -10i8)),
            Point2D(-5i64, -10i64)
        );
        assert_eq!(
            Point2D::<f32>::from(Point2D(-5i8, -10i8)),
            Point2D(-5.0f32, -10.0f32)
        );
        assert_eq!(
            Point2D::<f64>::from(Point2D(-5i8, -10i8)),
            Point2D(-5.0f64, -10.0f64)
        );

        assert_eq!(
            Point2D::<i32>::from(Point2D(5u16, 10u16)),
            Point2D(5i32, 10i32)
        );
        assert_eq!(
            Point2D::<i64>::from(Point2D(5u16, 10u16)),
            Point2D(5i64, 10i64)
        );
        assert_eq!(
            Point2D::<f32>::from(Point2D(5u16, 10u16)),
            Point2D(5.0f32, 10.0f32)
        );
        assert_eq!(
            Point2D::<f64>::from(Point2D(5u16, 10u16)),
            Point2D(5.0f64, 10.0f64)
        );

        assert_eq!(
            Point2D::<i32>::from(Point2D(-5i16, -10i16)),
            Point2D(-5i32, -10i32)
        );
        assert_eq!(
            Point2D::<i64>::from(Point2D(-5i16, -10i16)),
            Point2D(-5i64, -10i64)
        );
        assert_eq!(
            Point2D::<f32>::from(Point2D(-5i16, -10i16)),
            Point2D(-5.0f32, -10.0f32)
        );
        assert_eq!(
            Point2D::<f64>::from(Point2D(-5i16, -10i16)),
            Point2D(-5.0f64, -10.0f64)
        );

        assert_eq!(
            Point2D::<i64>::from(Point2D(5u32, 10u32)),
            Point2D(5i64, 10i64)
        );
        assert_eq!(
            Point2D::<f32>::from(Point2D(5u32, 10u32)),
            Point2D(5.0f32, 10.0f32)
        );
        assert_eq!(
            Point2D::<f64>::from(Point2D(5u32, 10u32)),
            Point2D(5.0f64, 10.0f64)
        );

        assert_eq!(
            Point2D::<i64>::from(Point2D(-5i32, -10i32)),
            Point2D(-5i64, -10i64)
        );
        assert_eq!(
            Point2D::<f32>::from(Point2D(-5i32, -10i32)),
            Point2D(-5.0f32, -10.0f32)
        );
        assert_eq!(
            Point2D::<f64>::from(Point2D(-5i32, -10i32)),
            Point2D(-5.0f64, -10.0f64)
        );

        assert_eq!(
            Point2D::<f32>::from(Point2D(5u64, 10u64)),
            Point2D(5.0f32, 10.0f32)
        );
        assert_eq!(
            Point2D::<f64>::from(Point2D(5u64, 10u64)),
            Point2D(5.0f64, 10.0f64)
        );

        assert_eq!(
            Point2D::<f32>::from(Point2D(-5i64, -10i64)),
            Point2D(-5.0f32, -10.0f32)
        );
        assert_eq!(
            Point2D::<f64>::from(Point2D(-5i64, -10i64)),
            Point2D(-5.0f64, -10.0f64)
        );
    }

    #[test]
    fn test_point3d_casts() {
        assert_eq!(
            Point3D::<i16>::from(Point3D(5u8, 10u8, 15u8)),
            Point3D(5i16, 10i16, 15i16)
        );
        assert_eq!(
            Point3D::<i32>::from(Point3D(5u8, 10u8, 15u8)),
            Point3D(5i32, 10i32, 15i32)
        );
        assert_eq!(
            Point3D::<i64>::from(Point3D(5u8, 10u8, 15u8)),
            Point3D(5i64, 10i64, 15i64)
        );
        assert_eq!(
            Point3D::<f32>::from(Point3D(5u8, 10u8, 15u8)),
            Point3D(5.0f32, 10.0f32, 15.0f32)
        );
        assert_eq!(
            Point3D::<f64>::from(Point3D(5u8, 10u8, 15u8)),
            Point3D(5.0f64, 10.0f64, 15.0f64)
        );

        assert_eq!(
            Point3D::<i16>::from(Point3D(-5i8, -10i8, -15i8)),
            Point3D(-5i16, -10i16, -15i16)
        );
        assert_eq!(
            Point3D::<i32>::from(Point3D(-5i8, -10i8, -15i8)),
            Point3D(-5i32, -10i32, -15i32)
        );
        assert_eq!(
            Point3D::<i64>::from(Point3D(-5i8, -10i8, -15i8)),
            Point3D(-5i64, -10i64, -15i64)
        );
        assert_eq!(
            Point3D::<f32>::from(Point3D(-5i8, -10i8, -15i8)),
            Point3D(-5.0f32, -10.0f32, -15.0f32)
        );
        assert_eq!(
            Point3D::<f64>::from(Point3D(-5i8, -10i8, -15i8)),
            Point3D(-5.0f64, -10.0f64, -15.0f64)
        );

        assert_eq!(
            Point3D::<i32>::from(Point3D(5u16, 10u16, 15u16)),
            Point3D(5i32, 10i32, 15i32)
        );
        assert_eq!(
            Point3D::<i64>::from(Point3D(5u16, 10u16, 15u16)),
            Point3D(5i64, 10i64, 15i64)
        );
        assert_eq!(
            Point3D::<f32>::from(Point3D(5u16, 10u16, 15u16)),
            Point3D(5.0f32, 10.0f32, 15.0f32)
        );
        assert_eq!(
            Point3D::<f64>::from(Point3D(5u16, 10u16, 15u16)),
            Point3D(5.0f64, 10.0f64, 15.0f64)
        );

        assert_eq!(
            Point3D::<i32>::from(Point3D(-5i16, -10i16, -15i16)),
            Point3D(-5i32, -10i32, -15i32)
        );
        assert_eq!(
            Point3D::<i64>::from(Point3D(-5i16, -10i16, -15i16)),
            Point3D(-5i64, -10i64, -15i64)
        );
        assert_eq!(
            Point3D::<f32>::from(Point3D(-5i16, -10i16, -15i16)),
            Point3D(-5.0f32, -10.0f32, -15.0f32)
        );
        assert_eq!(
            Point3D::<f64>::from(Point3D(-5i16, -10i16, -15i16)),
            Point3D(-5.0f64, -10.0f64, -15.0f64)
        );

        assert_eq!(
            Point3D::<i64>::from(Point3D(5u32, 10u32, 15u32)),
            Point3D(5i64, 10i64, 15i64)
        );
        assert_eq!(
            Point3D::<f32>::from(Point3D(5u32, 10u32, 15u32)),
            Point3D(5.0f32, 10.0f32, 15.0f32)
        );
        assert_eq!(
            Point3D::<f64>::from(Point3D(5u32, 10u32, 15u32)),
            Point3D(5.0f64, 10.0f64, 15.0f64)
        );

        assert_eq!(
            Point3D::<i64>::from(Point3D(-5i32, -10i32, -15i32)),
            Point3D(-5i64, -10i64, -15i64)
        );
        assert_eq!(
            Point3D::<f32>::from(Point3D(-5i32, -10i32, -15i32)),
            Point3D(-5.0f32, -10.0f32, -15.0f32)
        );
        assert_eq!(
            Point3D::<f64>::from(Point3D(-5i32, -10i32, -15i32)),
            Point3D(-5.0f64, -10.0f64, -15.0f64)
        );

        assert_eq!(
            Point3D::<f32>::from(Point3D(5u64, 10u64, 15u64)),
            Point3D(5.0f32, 10.0f32, 15.0f32)
        );
        assert_eq!(
            Point3D::<f64>::from(Point3D(5u64, 10u64, 15u64)),
            Point3D(5.0f64, 10.0f64, 15.0f64)
        );

        assert_eq!(
            Point3D::<f32>::from(Point3D(-5i64, -10i64, -15i64)),
            Point3D(-5.0f32, -10.0f32, -15.0f32)
        );
        assert_eq!(
            Point3D::<f64>::from(Point3D(-5i64, -10i64, -15i64)),
            Point3D(-5.0f64, -10.0f64, -15.0f64)
        );
    }
}
