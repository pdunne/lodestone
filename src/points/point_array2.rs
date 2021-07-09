/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! PointsArray2
//! 2D structs of arrays to
//! This is more efficienct that arrays of structs.
//!
// use crate::utils::conversions::{cart2pol, pol2cart};
use crate::utils::points::Points;
use rayon::prelude::*;
// use crate::PI;
// use crate::STACK_MAX;

/// Traits for manipulating and accessing PointArray2 types
pub trait PointArrays2<const N: usize> {
    /// Set output to be PointArays2
    type Output;
    /// Returns x
    fn x(&self) -> [f64; N];
    /// Returns y
    fn y(&self) -> [f64; N];
    /// Returns Point with original y, but new x
    fn with_x(&self, x: [f64; N]) -> Self::Output;
    /// Returns Point with original y, but new x
    fn with_y(&self, y: [f64; N]) -> Self::Output;
    /// Converts Point2 to PolarPoint
    // fn to_polar(&self) -> PolarPoint;
    /// Returns squared magnitude of PointArray
    fn magnitude_squared(&self) -> [f64; N];
    /// Returns magnitude of PointArray
    fn magnitude(&self) -> [f64; N];

    // /// Returns elementwise distance from PointArray to origin
    // fn distance_from_origin(&self) -> [f64; N];
    // /// Returns elementwise distance from one PointArray to another
    // fn distance_from_point(&self, other: &Self) -> [f64; N];
    // /// Returns elementwise dot product of two PointArrays
    // fn dot(&self, other: &Self) -> [f64; N];
    //
    //     /// Returns normalised vector from input vector
    //     fn unit(&self) -> Self::Output;
    /// Returns a point/vector of zeros
    fn zero() -> Self::Output;
    /// Returns a point/vector of ones
    fn identity() -> Self::Output;
    /// Returns a point/vector with x = 1.0, y = 0.0
    fn i_hat() -> Self::Output;
    /// Returns a point/vector with x = 0.0, y = 1.0
    fn j_hat() -> Self::Output;
}

// use std::fmt;
use std::convert::TryInto;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// Converts a Vec to array
fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

/// Point2 Array struct
struct PointArray2<const N: usize> {
    pub x: [f64; N],
    pub y: [f64; N],
}

impl<const N: usize> Default for PointArray2<N> {
    fn default() -> Self {
        Self {
            x: [f64::default(); N],
            y: [f64::default(); N],
        }
    }
}

impl<const N: usize> PointArray2<N> {
    pub fn new(x: [f64; N], y: [f64; N]) -> Self {
        Self { x: x, y: y }
    }
}

impl<const N: usize> Points for PointArray2<N> {
    type Output = Self;

    fn add_p(&self, other: &Self) -> Self {
        let mut op_x: [f64; N] = [0.0; N];
        let mut op_y: [f64; N] = [0.0; N];

        for i in 0..N {
            op_x[i] = self.x[i] + other.x[i];
            op_y[i] = self.y[i] + other.y[i];
        }

        PointArray2 { x: op_x, y: op_y }
    }

    fn sub_p(&self, other: &Self) -> Self {
        let mut op_x: [f64; N] = [0.0; N];
        let mut op_y: [f64; N] = [0.0; N];

        for i in 0..N {
            op_x[i] = self.x[i] - other.x[i];
            op_y[i] = self.y[i] - other.y[i];
        }

        PointArray2 { x: op_x, y: op_y }
    }

    fn mul_p(&self, other: &Self) -> Self {
        let mut op_x: [f64; N] = [0.0; N];
        let mut op_y: [f64; N] = [0.0; N];

        for i in 0..N {
            op_x[i] = self.x[i] * other.x[i];
            op_y[i] = self.y[i] * other.y[i];
        }

        PointArray2 { x: op_x, y: op_y }
    }

    fn div_p(&self, other: &Self) -> Self {
        let mut op_x: [f64; N] = [0.0; N];
        let mut op_y: [f64; N] = [0.0; N];

        for i in 0..N {
            op_x[i] = self.x[i] / other.x[i];
            op_y[i] = self.y[i] / other.y[i];
        }

        PointArray2 { x: op_x, y: op_y }
    }

    fn neg_p(&self) -> Self {
        let mut op_x: [f64; N] = [0.0; N];
        let mut op_y: [f64; N] = [0.0; N];

        for i in 0..N {
            op_x[i] = -self.x[i];
            op_y[i] = -self.y[i];
        }

        PointArray2 { x: op_x, y: op_y }
    }

    fn scale(&self, s: f64) -> Self {
        let mut op_x: [f64; N] = [0.0; N];
        let mut op_y: [f64; N] = [0.0; N];

        for i in 0..N {
            op_x[i] = self.x[i] * s;
            op_y[i] = self.y[i] * s;
        }

        PointArray2 { x: op_x, y: op_y }
    }

    fn round(&self) -> Self {
        let mut op_x: [f64; N] = [0.0; N];
        let mut op_y: [f64; N] = [0.0; N];

        for i in 0..N {
            op_x[i] = self.x[i].round();
            op_y[i] = self.y[i].round();
        }

        PointArray2 { x: op_x, y: op_y }
    }
}

// fn check_elem_size(num_elem: &usize) -> anyhow::Result<()> {
//     Ok(assert!(
//         *num_elem <= STACK_MAX,
//         "Warning: Possible stack overflow. Maximum number of elements is {}, {} were allocated",
//         STACK_MAX,
//         num_elem
//     ))
// }

impl<const N: usize> Add for PointArray2<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_p(&other)
    }
}

impl<const N: usize> AddAssign for PointArray2<N> {
    fn add_assign(&mut self, other: Self) {
        for i in 0..N {
            self.x[i] += other.x[i];
            self.y[i] += other.y[i];
        }
    }
}

impl<const N: usize> Sub for PointArray2<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.sub_p(&other)
    }
}

impl<const N: usize> Mul for PointArray2<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_p(&other)
    }
}

impl<const N: usize> Div for PointArray2<N> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.div_p(&other)
    }
}

impl<const N: usize> Neg for PointArray2<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.neg_p()
    }
}

impl<const N: usize> PointArrays2<N> for PointArray2<N> {
    type Output = Self;

    /// Returns x
    fn x(&self) -> [f64; N] {
        self.x
    }

    /// Returns y
    fn y(&self) -> [f64; N] {
        self.y
    }

    /// Returns PointArray with original y, but new x
    fn with_x(&self, x: [f64; N]) -> Self::Output {
        assert_eq!(x.len(), self.y.len());
        PointArray2::new(x, self.y)
    }
    /// Returns PointArray with original x, but new y
    fn with_y(&self, y: [f64; N]) -> Self::Output {
        assert_eq!(y.len(), self.x.len());
        PointArray2::new(self.x, y)
    }
    /// Converts Point2 to PolarPoint
    // fn to_polar(&self) -> PolarPoint;
    /// Returns squared magnitude of PointArray
    fn magnitude_squared(&self) -> [f64; N] {
        vec_to_array(
            self.x
                .par_iter()
                .zip(self.y.par_iter())
                .map(|(x, y)| x * x + y * y)
                .collect::<Vec<f64>>(),
        )
    }

    /// Returns magnitude of PointArray
    fn magnitude(&self) -> [f64; N] {
        vec_to_array(
            self.x
                .par_iter()
                .zip(self.y.par_iter())
                .map(|(x, y)| (x * x + y * y).sqrt())
                .collect::<Vec<f64>>(),
        )
    }
    //     /// Returns elementwise distance from PointArray to origin
    //     fn distance_from_origin(&self) -> [f64; N];
    //     /// Returns elementwise distance from one PointArray to another
    //     fn distance_from_point(&self, other: &Self) -> [f64; N];
    //     /// Returns elementwise dot product of two PointArrays
    //     fn dot(&self, other: &Self) -> [f64; N];
    //
    //     /// Returns normalised vector from input vector
    //     fn unit(&self) -> Self::Output;
    //     /// Returns a point/vector of zeros
    fn zero() -> Self::Output {
        Self {
            x: [0.0; N],
            y: [0.0; N],
        }
    }
    /// Returns a point/vector of ones
    fn identity() -> Self::Output {
        Self {
            x: [1.0; N],
            y: [1.0; N],
        }
    }
    /// Returns a point/vector with x = 1.0, y = 0.0
    fn i_hat() -> Self::Output {
        Self {
            x: [1.0; N],
            y: [0.0; N],
        }
    }
    /// Returns a point/vector with x = 0.0, y = 1.0
    fn j_hat() -> Self::Output {
        Self {
            x: [0.0; N],
            y: [1.0; N],
        }
    }
}
#[cfg(test)]
mod tests {
    use std::usize;

    // use crate::utils::point_array2::{PointArray2, PointArrays2};
    use super::{PointArray2, PointArrays2};
    use crate::STACK_MAX;

    #[test]
    fn test_default() {
        let array = PointArray2::<2>::default();
        let const_array: [f64; 2] = [0.0, 0.0];
        assert_eq!(array.x, const_array, "Test x");
        assert_eq!(array.y, const_array, "Test y");
    }
    #[test]
    fn test_zeros() {
        const NUM_ELEM: usize = 10;
        let array = PointArray2::<NUM_ELEM>::default();
        let const_array: [f64; NUM_ELEM] = [0.0; NUM_ELEM];
        assert_eq!(array.x, const_array, "Test x");
        assert_eq!(array.y, const_array, "Test y");
    }

    #[test]
    fn test_new() {
        let array = PointArray2::<4>::new([1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]);
        let array_x: [f64; 4] = [1.0, 2.0, 3.0, 4.0];
        let array_y: [f64; 4] = [5.0, 6.0, 7.0, 8.0];
        assert_eq!(array.x, array_x, "Test x");
        assert_eq!(array.y, array_y, "Test y");
    }

    #[test]
    fn test_add() {
        let first_array = PointArray2::<2>::new([1.0, 2.0], [3.0, 4.0]);
        let second_array = PointArray2::<2>::new([5.0, 6.0], [7.0, 8.0]);
        let sum_array = first_array + second_array;
        let array_x: [f64; 2] = [6.0, 8.0];
        let array_y: [f64; 2] = [10.0, 12.0];
        assert_eq!(sum_array.x, array_x, "Test x");
        assert_eq!(sum_array.y, array_y, "Test y");
    }

    #[test]
    fn test_magnitude_squared() {
        let array = PointArray2::<2>::new([1.0, 2.0], [3.0, 4.0]).magnitude_squared();
        let mag_array: [f64; 2] = [10.0, 20.0];
        assert_eq!(array, mag_array);
    }

    #[test]
    fn test_magnitude() {
        let array = PointArray2::<2>::new([1.0, 2.0], [3.0, 4.0]).magnitude();
        let mag_array: [f64; 2] = [10.0_f64.sqrt(), 20.0_f64.sqrt()];
        assert_eq!(array, mag_array);
    }

    #[test]
    fn test_magnitude_1e4() {
        const NUM_ELEM: usize = 10000;
        assert!(
            NUM_ELEM <= STACK_MAX,
            "Warning: Possible stack overflow. Maximum number of elements is {}, {} were allocated",
            STACK_MAX,
            NUM_ELEM
        );
        let array = PointArray2::<NUM_ELEM>::new([1.0; NUM_ELEM], [2.0; NUM_ELEM]).magnitude();
        let mag_array: [f64; NUM_ELEM] = [5.0_f64.sqrt(); NUM_ELEM];
        assert_eq!(array, mag_array);
    }
}
