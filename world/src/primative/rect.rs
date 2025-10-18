use crate::geom_traits::{Depth, Width, Height};
use crate::primative::point::{Point2D, Point3D};

#[derive(Copy, Clone, Debug, Default, Hash)]
pub struct Rect2D<T> {
    pub min: Point2D<T>,
    pub max: Point2D<T>,
}

#[derive(Copy, Clone, Debug, Default, Hash)]
pub struct Rect3D<T> {
    pub min: Point3D<T>,
    pub max: Point3D<T>,
}

macro_rules! impl_width {
    ($t:ty) => {
        impl Width<$t> for Rect2D<$t> {
            fn width(&self) -> $t {
                self.max.0 - self.min.0
            }
        }
        impl Width<$t> for Rect3D<$t> {
            fn width(&self) -> $t {
                self.max.0 - self.min.0
            }
        }
    };
}

macro_rules! impl_height {
    ($t:ty) => {
        impl Height<$t> for Rect2D<$t> {
            fn height(&self) -> $t {
                self.max.1 - self.min.1
            }
        }
        impl Height<$t> for Rect3D<$t> {
            fn height(&self) -> $t {
                self.max.1 - self.min.1
            }
        }
    };
}

macro_rules! impl_depth {
    ($t:ty) => {
        impl Depth<$t> for Rect3D<$t> {
            fn depth(&self) -> $t {
                self.max.2 - self.min.2
            }
        }
    };
}

impl_width!(u8);
impl_width!(u16);
impl_width!(u32);
impl_width!(u64);
impl_width!(i8);
impl_width!(i16);
impl_width!(i32);
impl_width!(i64);
impl_width!(f32);
impl_width!(f64);

impl_height!(u8);
impl_height!(u16);
impl_height!(u32);
impl_height!(u64);
impl_height!(i8);
impl_height!(i16);
impl_height!(i32);
impl_height!(i64);
impl_height!(f32);
impl_height!(f64);

impl_depth!(u8);
impl_depth!(u16);
impl_depth!(u32);
impl_depth!(u64);
impl_depth!(i8);
impl_depth!(i16);
impl_depth!(i32);
impl_depth!(i64);
impl_depth!(f32);
impl_depth!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect2d_width() {
        let rect = Rect2D {
            min: Point2D(1, 2),
            max: Point2D(5, 3),
        };
        assert_eq!(rect.width(), 4);

        let rect_f32 = Rect2D {
            min: Point2D(0.5f32, 0.0f32),
            max: Point2D(5.5f32, 3.3f32),
        };
        assert_eq!(rect_f32.width(), 5.0f32);
    }

    #[test]
    fn test_rect3d_width() {
        let rect = Rect3D {
            min: Point3D(1, 0, 0),
            max: Point3D(5, 3, 2),
        };
        assert_eq!(rect.width(), 4);

        let rect_f32 = Rect3D {
            min: Point3D(0.5f32, 0.0f32, 0.0f32),
            max: Point3D(5.5f32, 3.3f32, 2.2f32),
        };
        assert_eq!(rect_f32.width(), 5.0f32);
    }

    #[test]
    fn test_rect2d_height() {
        let rect = Rect2D {
            min: Point2D(0, 1),
            max: Point2D(5, 3),
        };
        assert_eq!(rect.height(), 2);

        let rect_f32 = Rect2D {
            min: Point2D(0.0f32, 0.3f32),
            max: Point2D(5.5f32, 3.3f32),
        };
        assert_eq!(rect_f32.height(), 3.0f32);
    }

    #[test]
    fn test_rect3d_height() {
        let rect = Rect3D {
            min: Point3D(0, 1, 0),
            max: Point3D(5, 3, 2),
        };
        assert_eq!(rect.height(), 2);

        let rect_f32 = Rect3D {
            min: Point3D(0.0f32, 0.3f32, 0.0f32),
            max: Point3D(5.5f32, 3.3f32, 2.2f32),
        };
        assert_eq!(rect_f32.height(), 3.0f32);
    }

    #[test]
    fn test_rect3d_depth() {
        let rect = Rect3D {
            min: Point3D(0, 0, 1),
            max: Point3D(5, 3, 2),
        };
        assert_eq!(rect.depth(), 1);

        let rect_f32 = Rect3D {
            min: Point3D(0.0f32, 0.0f32, 0.2f32),
            max: Point3D(5.5f32, 3.3f32, 2.2f32),
        };
        assert_eq!(rect_f32.depth(), 2.0f32);
    }
}

