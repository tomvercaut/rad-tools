use crate::primative::point::{Point2D, Point3D};

#[derive(Copy, Clone, Debug, Default, Hash)]
pub struct Line2D<T> {
    pub start: Point2D<T>,
    pub end: Point2D<T>,
}

#[derive(Copy, Clone, Debug, Default, Hash)]
pub struct Line3D<T> {
    pub start: Point3D<T>,
    pub end: Point3D<T>,
}
