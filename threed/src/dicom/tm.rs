use std::ops::Mul;

use nalgebra::{Matrix4, SMatrix, Vector3, Vector4};

pub trait IndexToCoordate<I, C> {
    fn i2c(&self, index: &[I; 3]) -> [C; 3];
}

pub trait CoordinateToIndex<I, C> {
    fn c2i(&self, coordinate: &[C; 3]) -> [I; 3];
}

pub struct Rcs {
    i2c: SMatrix<f64, 4, 4>,
    _c2i: SMatrix<f64, 4, 4>,
}

impl Rcs {
    pub fn from_dcm(image_orientation: &[f64; 6], image_position: &[f64; 3], spacing: &[f64; 3]) -> Option<Self> {
        let x = Vector3::new(image_orientation[0], image_orientation[1], image_orientation[2]);
        let y = Vector3::new(image_orientation[3], image_orientation[4], image_orientation[5]);
        let z = x.cross(&y);

        let c0 = Vector4::new(
            image_orientation[0] * spacing[0],
            image_orientation[1] * spacing[0],
            image_orientation[2] * spacing[0],
            0.0,
        );

        let c1 = Vector4::new(
            image_orientation[3] * spacing[1],
            image_orientation[4] * spacing[1],
            image_orientation[5] * spacing[1],
            0.0,
        );

        let c2 = Vector4::new(
            z[0] * spacing[2],
            z[1] * spacing[2],
            z[2] * spacing[2],
            0.0,
        );

        let c3 = Vector4::new(
            image_position[0],
            image_position[1],
            image_position[2],
            1.0,
        );

        let i2c = Matrix4::from_columns(&[
            c0,
            c1,
            c2,
            c3
        ]);

        let mut c2i = i2c.clone();
        if !c2i.try_inverse_mut() {
            None
        } else {
            Some(Self {
                i2c,
                _c2i: c2i,
            })
        }
    }
}

impl IndexToCoordate<usize, f64> for Rcs {
    fn i2c(&self, index: &[usize; 3]) -> [f64; 3] {
        let pixel_index = Vector4::new(index[0] as f64, index[1] as f64, index[2] as f64, 1.0);
        let coordinate = self.i2c.mul(&pixel_index);
        [coordinate[0], coordinate[1], coordinate[2]]
    }
}

// 
// impl CoordinateToIndex<usize, f64> for Rcs {
//     fn c2i(&self, coordinate: &[f64; 3]) -> [usize; 3] {
//         let coordinate = Vector4::new(coordinate[0], coordinate[1], coordinate[2], 0.0);
//         let index = self.c2i.mul(&coordinate);
//         [index[0] as usize, index[1] as usize, index[2] as usize]
//     }
// }
// 
// impl CoordinateToIndex<f64, f64> for Rcs {
//     fn c2i(&self, coordinate: &[f64; 3]) -> [f64; 3] {
//         let coordinate = Vector4::new(coordinate[0], coordinate[1], coordinate[2], 0.0);
//         let index = self.c2i.mul(&coordinate);
//         [index[0], index[1], index[2]]
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_product() {
        let x = Vector3::new(1.0, 0.0, 0.0);
        let y = Vector3::new(0.0, 1.0, 0.0);

        let z = x.cross(&y);
        assert_eq!(0.0, z[(0, 0)]);
        assert_eq!(0.0, z[(1, 0)]);
        assert_eq!(1.0, z[(2, 0)]);
        assert_eq!(z, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn i2c() {
        let image_orientation = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let image_position = [-299.4140625, -545.9140625, 402.0];
        let spacing = [1.171875, 1.171875, 2.0];
        let rcs = Rcs::from_dcm(&image_orientation, &image_position, &spacing).unwrap();
        let indices = [
            [0, 0, 0usize],
            [1, 0, 0usize],
            [0, 1, 0usize],
            [0, 0, 1usize],
            [1, 1, 1usize],
        ];
        let results = [
            [-299.4140625, -545.9140625, 402.0],
            [-299.4140625 + spacing[0], -545.9140625, 402.0],
            [-299.4140625, -545.9140625 + spacing[1], 402.0],
            [-299.4140625, -545.9140625, 402.0 + spacing[2]],
            [-299.4140625 + spacing[0], -545.9140625 + spacing[1], 402.0 + spacing[2]],
        ];
        assert_eq!(indices.len(), results.len());
        for (index, expected) in indices.iter().zip(results.iter()) {
            let calculated = rcs.i2c(index);
            assert_eq!(calculated, *expected);
        }
    }
}