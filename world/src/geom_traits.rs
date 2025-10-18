/// Trait for types that can calculate distance between two objects.
pub trait DistanceTo {
    /// The type used to represent distance
    type DistanceType;
    /// Calculates the distance to another object
    fn distance_to(&self, other: &Self) -> Self::DistanceType;
    /// Calculates the squared distance to another object
    fn sq_distance_to(&self, other: &Self) -> Self::DistanceType;
}

/// Trait for types that can be bounded by a box
pub trait BoundingBox {
    /// The type used to represent the bounding box
    type BoundingBoxType;
    /// Returns the bounding box that contains this object
    fn bounding_box(&self) -> Self::BoundingBoxType;
}

/// Trait for types that have a width dimension
pub trait Width<T> {
    /// Returns the width of the object
    fn width(&self) -> T;
}

/// Trait for types that have a height dimension
pub trait Height<T> {
    /// Returns the height of the object
    fn height(&self) -> T;
}

/// Trait for types that have a depth dimension
pub trait Depth<T> {
    /// Returns the depth of the object
    fn depth(&self) -> T;
}

/// Trait for types that can calculate their area
pub trait Area {
    /// The type used to represent area
    type AreaType;
    /// Returns the area of the object
    fn area(&self) -> Self::AreaType;
}