use crate::primative::point::{Point2D, Point3D};

pub struct Edge2D<T> {
    pub start: Point2D<T>,
    pub end: Point2D<T>,
}

pub struct Edge3D<T> {
    pub start: Point3D<T>,
    pub end: Point3D<T>,
}
