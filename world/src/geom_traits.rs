
pub trait DistanceTo {
    type DistanceType;
    fn distance_to(&self, other: &Self) -> Self::DistanceType;
    fn sq_distance_to(&self, other: &Self) -> Self::DistanceType;
}

pub trait BoundingBox {
    type BoundingBoxType;

    fn bounding_box(&self) -> Self::BoundingBoxType;
}

pub trait Width<T> {
    fn width(&self) -> T;
}

pub trait Height<T> {
    fn height(&self) -> T;
}

pub trait Depth<T> {
    fn depth(&self) -> T;
}