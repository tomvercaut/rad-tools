
pub trait DistanceTo {
    type DistanceType;
    fn distance_to(&self, other: &Self) -> Self::DistanceType;
    fn sq_distance_to(&self, other: &Self) -> Self::DistanceType;
}