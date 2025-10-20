use crate::geom_traits::Intersection;
use crate::intersection::IntersectionResult;
use crate::intersection::{IntersectionError, RangedIntersection};
use crate::primitive::line::Line2D;
use crate::primitive::point::Point2D;

macro_rules! impl_intersection {
    ($t:ty) => {
        impl Intersection<Line2D<$t>> for Line2D<$t> {
            type IntersectionResultType = IntersectionResult<RangedIntersection<Point2D<$t>>>;
            fn intersect(&self, lb: &Line2D<$t>) -> Self::IntersectionResultType {
                let la = self;
                let denom = (lb.end.1 - lb.start.1) * (la.end.0 - la.start.0) -
                            (lb.end.0 - lb.start.0) * (la.end.1 - la.start.1);
                if denom == 0.0 {
                    return Err(IntersectionError::ParallelLines);
                }
                let ua = ((lb.end.0 - lb.start.0) * (la.start.1 - lb.start.1) -
                        (lb.end.1 - lb.start.1) * (la.start.0 - lb.start.0)) / denom;
                let ub = ((la.end.0 - la.start.0) * (la.start.1 - lb.start.1) -
                        (la.end.1 - la.start.1) * (la.start.0 - lb.start.0)) / denom;


                let ip = Point2D(
                    la.start.0 + ua * (la.end.0 - la.start.0),
                    la.start.1 + ua * (la.end.1 - la.start.1)
                );

                if ua < 0.0 || ua > 1.0 || ub < 0.0 || ub > 1.0 {
                    Ok(RangedIntersection::Outside(ip))
                } else {
                    Ok(RangedIntersection::Inside(ip))
                }
            }
        }
    };
    ($($t:ty),*) => {
        $(impl_intersection!($t);)*
    };
}

impl_intersection!(f32, f64);
