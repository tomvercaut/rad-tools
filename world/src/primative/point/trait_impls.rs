use crate::geom_traits::DistanceTo;
use crate::primative::point::{Point2D, Point3D};

macro_rules! impl_distance_to {
    ($t:ty) => {
        impl DistanceTo for Point2D<$t> {
            type DistanceType = f64;

            fn distance_to(&self, other: &Self) -> Self::DistanceType {
                self.sq_distance_to(other).sqrt()
            }

            fn sq_distance_to(&self, other: &Self) -> Self::DistanceType {
                (self.0 as Self::DistanceType - other.0 as Self::DistanceType).powi(2)
                    + (self.1 as Self::DistanceType - other.1 as Self::DistanceType).powi(2)
            }
        }
        
        impl DistanceTo for Point3D<$t> {
            type DistanceType = f64;

            fn distance_to(&self, other: &Self) -> Self::DistanceType {
                self.sq_distance_to(other).sqrt()
            }

            fn sq_distance_to(&self, other: &Self) -> Self::DistanceType {
                (self.0 as Self::DistanceType - other.0 as Self::DistanceType).powi(2)
                    + (self.1 as Self::DistanceType - other.1 as Self::DistanceType).powi(2)
                    + (self.2 as Self::DistanceType - other.2 as Self::DistanceType).powi(2)
            }
        }
    };
}

impl_distance_to!(u8);
impl_distance_to!(u16);
impl_distance_to!(u32);
impl_distance_to!(u64);
impl_distance_to!(i8);
impl_distance_to!(i16);
impl_distance_to!(i32);
impl_distance_to!(i64);
impl_distance_to!(f32);
impl_distance_to!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_integer_distance() {
        let p1: Point2D<i32> = Point2D(0, 0);
        let p2: Point2D<i32> = Point2D(3, 4);
        let p3: Point2D<i32> = Point2D(-3, -4);
        assert_eq!(p1.distance_to(&p2), 5.0);
        assert_eq!(p1.distance_to(&p3), 5.0);
        assert_eq!(p2.distance_to(&p3), 10.0);
    }

    #[test]
    fn test_point3d_integer_distance() {
        let p1: Point3D<i32> = Point3D(0, 0, 0);
        let p2: Point3D<i32> = Point3D(3, 4, 12);
        let p3: Point3D<i32> = Point3D(-3, -4, -12);
        assert_eq!(p1.distance_to(&p2), 13.0);
        assert_eq!(p1.distance_to(&p3), 13.0);
        assert_eq!(p2.distance_to(&p3), 26.0);
    }

    #[test]
    fn test_point2d_float_distance() {
        let p1: Point2D<f64> = Point2D(0.0, 0.0);
        let p2: Point2D<f64> = Point2D(1.5, 2.5);
        assert!((p1.distance_to(&p2) - 8.5f64.sqrt()).abs() < f64::EPSILON);
    }

    #[test]
    fn test_point3d_float_distance() {
        let p1: Point3D<f32> = Point3D(0.0, 0.0, 0.0);
        let p2: Point3D<f32> = Point3D(1.5, 2.5, 3.5);
        assert!((p1.distance_to(&p2) - 20.75f64.sqrt()).abs() < f64::EPSILON);
    }
}

