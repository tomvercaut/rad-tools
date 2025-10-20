mod line;

pub use line::*;

#[derive(thiserror::Error, Debug)]
pub enum IntersectionError {
    #[error("No intersection because the lines are parallel.")]
    ParallelLines,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub enum RangedIntersection<T> {
    Inside(T),
    Outside(T),
}

pub type IntersectionResult<T> = Result<T, IntersectionError>;
