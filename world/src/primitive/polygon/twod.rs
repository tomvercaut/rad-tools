use crate::geom_traits::Reverse;
use crate::geom_traits::{Area, BoundingBox};
use crate::primitive::direction::RotationDirection;
use crate::primitive::point::Point2D;
use crate::primitive::rect::Rect2D;

/// Represents a 2D polygon with cached bounding box and area calculations.
/// The polygon is defined by a sequence of vertices stored in counterclockwise order.
#[derive(Clone, Debug, Default)]
pub struct Polygon2D<T> {
    /// Vertices of the polygon stored in counterclockwise order.
    /// Must contain at least 3 points to form a valid polygon.
    points: Vec<Point2D<T>>,

    /// Cached axis-aligned bounding box that fully contains the polygon.
    /// Updated when the polygon is created.
    bbox: Rect2D<T>,

    /// Cached area of the polygon.
    /// Always stored as a positive value regardless of vertex winding order.
    area: f64,

    /// The direction / order of the points in the polygon.
    direction: RotationDirection,
}

macro_rules! impl_polygon2d {
     ($t:ty) => {
        impl Polygon2D<$t> {
            pub fn new(points: Vec<Point2D<$t>>) -> crate::primitive::Result<Self> {
                if points.len() < 3 {
                    return Err(crate::primitive::Error::PolygonRequiresAtLeast3Points);
                }
                let area_2d = Area2D { points: &points };
                let area = area_2d.area();

                let direction = if area < 0.0 {
                    RotationDirection::Clockwise
                } else {
                    RotationDirection::CounterClockwise
                };

                let bbox = Bbox2D { points: &points };
                let bbox = bbox.bounding_box();

                Ok(Self {
                    points,
                    bbox,
                    area: area.abs(),
                    direction,
                })
            }
            
            pub fn points(&self) -> &[Point2D<$t>] {
                &self.points
            }
            
            pub fn len(&self) -> usize {
                self.points.len()
            }

            pub fn direction(&self) -> RotationDirection {
                self.direction
            }
        }
         
         impl Area for Polygon2D<$t> {
            type AreaType = f64;
            
            fn area(&self) -> Self::AreaType {
                self.area
            }
         }
         
         impl BoundingBox for Polygon2D<$t> {
            type BoundingBoxType = Rect2D<$t>;

            fn bounding_box(&self) -> Self::BoundingBoxType {
                self.bbox
            }
         }

         impl Reverse for Polygon2D<$t> {
            fn reverse(&self) -> Self {
                let mut points = self.points.clone();
                points.reverse();
                Self {
                    points,
                    bbox: self.bbox,
                    area: self.area,
                    direction: self.direction.reverse(),
                }
            }

            fn reverse_mut(&mut self) {
                self.points.reverse();
                self.direction.reverse_mut();
            }
         }
     };
    ($($t:ty),*) => {
        $(impl_polygon2d!($t);)*
    };
}

impl_polygon2d!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

struct Area2D<'a, T> {
    points: &'a [Point2D<T>],
}

macro_rules! impl_area {
    ($t:ty) => {
        impl<'a> Area for Area2D<'a, $t> {
            type AreaType = f64;

            /// Based on https://en.wikipedia.org/wiki/Shoelace_formula
            /// counterclockwise = positive area, clockwise = negative area
            fn area(&self) -> Self::AreaType {
                let mut area: Self::AreaType = Default::default();
                let np = self.points.len();
                for i in 0..np {
                    let j = (i + 1) % np;

                    let p1 = self.points[i];
                    let p2 = self.points[j];
                    area += (p1.0 as Self::AreaType + p2.1 as Self::AreaType) * (p1.0 as Self::AreaType - p2.0 as Self::AreaType);
                }
                area *= 0.5;
                area
            }
        }
    };
    ($($t:ty),*) => {
        $(impl_area!($t);)*
    };
}

impl_area!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

struct Bbox2D<'a, T> {
    points: &'a [Point2D<T>],
}

macro_rules! impl_bounding_box {
    ($t:ty) => {
        impl<'a> BoundingBox for Bbox2D<'a, $t> {
            type BoundingBoxType = Rect2D<$t>;

            fn bounding_box(&self) -> Self::BoundingBoxType {
                let n = self.points.len();
                if n == 0 {
                    return Self::BoundingBoxType::default();
                }
                let mut min = self.points[0];
                let mut max = min;
                for p in self.points.iter() {
                    if p.0 < min.0 {
                        min.0 = p.0;
                    }
                    if p.0 > max.0 {
                        max.0 = p.0;
                    }
                    if p.1 < min.1 {
                        min.1 = p.1;
                    }
                    if p.1 > max.1 {
                        max.1 = p.1;
                    }
                }
                Self::BoundingBoxType {
                    min,
                    max,
                }
            }
        }
    };
    ($($t:ty),*) => {
        $(impl_bounding_box!($t);)*
    };
}

impl_bounding_box!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_polygon_creation() {
        let points = vec![
            Point2D(1.0, 1.0),
            Point2D(0.0, 0.0),
            Point2D(1.0, 0.0),
        ];
        let polygon = Polygon2D::<f64>::new(points).unwrap();
        assert_eq!(polygon.len(), 3);
        assert_eq!(polygon.area(), 0.5);
    }

    #[test]
    fn test_invalid_polygon_creation() {
        let points = vec![
            Point2D(0.0, 0.0),
            Point2D(1.0, 0.0),
        ];
        let result = Polygon2D::<f64>::new(points);
        assert!(result.is_err());
    }

    #[test]
    fn test_area_calculation() {
        let points = vec![
            Point2D(0.0, 0.0),
            Point2D(2.0, 0.0),
            Point2D(2.0, 2.0),
            Point2D(0.0, 2.0),
        ];
        let polygon = Polygon2D::<f64>::new(points).unwrap();
        assert_eq!(polygon.area(), 4.0);
    }

    #[test]
    fn test_bounding_box() {
        let points = vec![
            Point2D(1.0, 3.0),
            Point2D(1.0, 1.0),
            Point2D(3.0, 1.0),
            Point2D(3.0, 3.0),
        ];
        let polygon = Polygon2D::<f64>::new(points).unwrap();
        let bbox = polygon.bounding_box();
        assert_eq!(bbox.min, Point2D(1.0, 1.0));
        assert_eq!(bbox.max, Point2D(3.0, 3.0));
    }

    #[test]
    fn test_reverse() {
        let points = vec![
            Point2D(0.0, 0.0),
            Point2D(1.0, 0.0),
            Point2D(1.0, 1.0),
        ];
        let polygon = Polygon2D::<f64>::new(points.clone()).unwrap();
        let reversed = polygon.reverse();

        // Check points are in reverse order
        for (i, point) in points.iter().rev().enumerate() {
            assert_eq!(&reversed.points()[i], point);
        }

        // Area and bbox should remain the same
        assert_eq!(reversed.area(), polygon.area());
        assert_eq!(reversed.bounding_box(), polygon.bounding_box());

        // Direction should be reversed
        assert_eq!(reversed.direction(), polygon.direction().reverse());
    }

    #[test]
    fn test_reverse_mut() {
        let points = vec![
            Point2D(0.0, 0.0),
            Point2D(1.0, 0.0),
            Point2D(1.0, 1.0),
        ];
        let mut polygon = Polygon2D::<f64>::new(points.clone()).unwrap();
        let original_direction = polygon.direction();

        polygon.reverse_mut();

        // Check points are in reverse order
        for (i, point) in points.iter().rev().enumerate() {
            assert_eq!(&polygon.points()[i], point);
        }

        // Direction should be reversed
        assert_eq!(polygon.direction(), original_direction.reverse());
    }
}

