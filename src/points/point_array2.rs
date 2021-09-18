/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! # PointArray2
//! Structs of arrays to store x and values. This is more efficienct than creating
//! an array of structs.
//!
//! As arrays are stored on the stack, The number of elements per axis is limited
//! to 10,000. For more elements, the PointVec2 struct can be used instead.
//!
//!
//! ## WARNING this file is no longer included as a module!

use crate::points::internal_norm;
use crate::points::rotation_2d::rotate_tuple2;
use crate::points::Points;
use integer_sqrt::IntegerSquareRoot;
use rayon::prelude::*;

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
    /// Returns elementwise dot product of two PointArrays
    fn dot(&self, other: &Self) -> [f64; N];

    /// Rotates point anti-clockwise about an angle alpha
    fn rotate(&self, alpha: &f64) -> Self::Output;

    /// Returns normalised vector from input vector
    fn unit(&self) -> Self::Output;
    /// Returns a point/vector of zeros
    fn zero() -> Self::Output;
    /// Returns a point/vector of ones
    fn identity() -> Self::Output;
    /// Returns a point/vector with x = 1.0, y = 0.0
    fn i_hat() -> Self::Output;
    /// Returns a point/vector with x = 0.0, y = 1.0
    fn j_hat() -> Self::Output;
}

use std::convert::TryInto;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// Converts a Vec to array
fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

/// Point2 Array struct
#[derive(Debug, PartialEq)]
pub struct PointArray2<const N: usize> {
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
    /// Returns elementwise dot product of two PointArrays
    fn dot(&self, other: &Self) -> [f64; N] {
        vec_to_array(
            self.x
                .par_iter()
                .zip(self.y.par_iter())
                .zip(other.x.par_iter())
                .zip(other.y.par_iter())
                .map(|(((x1, y1), x2), y2)| (x1 * x2 + y1 * y2))
                .collect::<Vec<f64>>(),
        )
    }

    /// Rotates point anti-clockwise about an angle alpha
    fn rotate(&self, alpha: &f64) -> Self::Output {
        let (x_rot, y_rot) = self
            .x
            .par_iter()
            .zip(self.y.par_iter())
            .map(|(x, y)| rotate_tuple2(&(*x, *y), alpha))
            .collect::<(Vec<f64>, Vec<f64>)>();
        PointArray2::new(vec_to_array(x_rot), vec_to_array(y_rot))
    }

    /// Returns normalised vector from input vector
    fn unit(&self) -> Self::Output {
        let (x_local, y_local) = self
            .x
            .par_iter()
            .zip(self.y.par_iter())
            .map(|(x, y)| internal_norm(x, y))
            .collect::<(Vec<f64>, Vec<f64>)>();
        PointArray2::new(vec_to_array(x_local), vec_to_array(y_local))
    }
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

fn hard_function(x: &f64, y: &f64) -> (f64, f64) {
    (x.powi(3).sqrt(), y.powi(3).sqrt())
}

impl<const N: usize> PointArray2<N> {
    /// Method to calculate a 'hard' function on a PointArray2 struct
    ///  - Iterates over the point array
    ///  - passes the internal x,y pairs to a hard function
    ///  - The hard function returns a tuple (f64, f64)
    ///  - This is collected in a tuple of Vec
    ///  - Each Vec is converted into an array
    ///  - And finally the arrays are wrapped in a PointArray2 struct
    pub fn gen_point_closure(&self) -> PointArray2<N> {
        let (x_local, y_local) = self
            .x
            .par_iter()
            .zip(self.y.par_iter())
            .map(|(x, y)| hard_function(x, y))
            .collect::<(Vec<f64>, Vec<f64>)>();
        PointArray2::new(vec_to_array(x_local), vec_to_array(y_local))
    }
}

impl<const N: usize> PointArray2<N> {
    pub fn grid2d(x_min: &f64, x_max: &f64, y_min: &f64, y_max: &f64) -> PointArray2<N> {
        // const ARRAY_NUM: usize = 9;
        let num_points: usize = N.integer_sqrt();
        let mut x = [0.0; N];
        let mut y = [0.0; N];
        let mut k: usize = 0;

        let step_x = (x_max - x_min) / (num_points - 1) as f64;
        let step_y = (y_max - y_min) / (num_points - 1) as f64;

        for i in 0..num_points {
            for j in 0..num_points {
                x[k] = x_min + i as f64 * step_x;
                y[k] = y_min + j as f64 * step_y;
                k += 1;
            }
        }
        PointArray2::new(x, y)
    }
}

/// Generates a 2D grid of points (x_min:x_max, y_min:y_max) in the form of
/// a tuple of Vectors (x, y) where x,y are Vec<f64>
///
/// This version generates the points using an iterator where the range is converted
/// into a parallel iter (Rayon), a flat map
pub fn cart_prod_2d<const N: usize>(
    x_min: &f64,
    x_max: &f64,
    y_min: &f64,
    y_max: &f64,
) -> (Vec<f64>, Vec<f64>) {
    let xs = (0_usize..N).into_par_iter();
    let ys = (0_usize..N).into_par_iter();
    // let xs = &xs;
    // let ys = &ys;

    let num_points: usize = N;

    let step_x = (x_max - x_min) / (num_points - 1) as f64;
    let step_y = (y_max - y_min) / (num_points - 1) as f64;

    let (xx, yy): (Vec<f64>, Vec<f64>) = xs
        .clone()
        .flat_map(|x| {
            ys.clone()
                .map(move |y| (x_min + (x as f64) * step_x, y_min + y as f64 * step_y))
        })
        .unzip();

    (xx, yy)

    // various tests below commented out because they are not needed
    // let (values, (squares, cubes)): (Vec<_>, (Vec<_>, Vec<_>)) = (0..4).into_par_iter()
    // .map(|i| (i, (i * i, i * i * i)))
    // .unzip();

    // let (squares, cubes): (Vec<_>, Vec<_>) = (0..N)
    //     .into_par_iter()
    //     .map(|i| (x_min + (i as f64) * step_x, y_min + i as f64 * step_y))
    //     .unzip();

    // assert_eq!(squares, [0, 1, 4, 9]);
    // assert_eq!(cubes, [0, 1, 8, 27]);

    // let (xx, yy): = xs.flat_map(|x| {
    // ys.clone().flat_map({
    //         .map(move |y| (x_min + (x as f64) * step_x, y_min + y as f64 * step_y))
    // })}).unzip();

    // let (xx, yy): (Vec<f64>, Vec<f64>) = xs
    //     .clone()
    //     .flat_map(|x| {
    //         ys.clone()
    //             .map(move |y| (x_min + (x as f64) * step_x, y_min + y as f64 * step_y))
    //     })
    //     .unzip(); // .collect::<Vec<(i32, i32, i32)>>();

    // PointArray2::<N> {
    //     x: vec_to_array(xx),
    //     y: vec_to_array(yy),
    //     // x: xx.try_into().unwrap(),
    //     // y: yy.try_into().unwrap(),
    // }
}

pub fn cart_prod_3d<const N: usize>() -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let xs = (0..N).into_par_iter();
    let ys = (0..N).into_par_iter();
    let zs = (0..N).into_par_iter();

    let ys = &ys;
    let zs = &zs;

    let (xx, (yy, zz)): (Vec<f64>, (Vec<f64>, Vec<f64>)) = xs
        .flat_map(move |x| {
            ys.clone()
                .flat_map(move |y| zs.clone().map(move |z| (x as f64, (y as f64, z as f64))))
        })
        .unzip();

    (xx, yy, zz)
}

#[cfg(test)]
mod tests {
    // use std::usize;

    use super::{cart_prod_2d, cart_prod_3d, hard_function, PointArray2, PointArrays2};
    use crate::STACK_MAX;
    const NUM_ELEM: usize = 100;
    // use crate::points::internal_norm;

    #[test]
    fn test_default() {
        let array = PointArray2::<2>::default();
        let const_array: [f64; 2] = [0.0, 0.0];
        assert_eq!(array.x, const_array, "Test x");
        assert_eq!(array.y, const_array, "Test y");
    }
    #[test]
    fn test_zeros() {
        // const NUM_ELEM: usize = 10;
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
        // const NUM_ELEM: usize = 10000;
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

    #[test]
    fn test_unit() {
        // const NUM_ELEM: usize = 10;
        assert!(
            NUM_ELEM <= STACK_MAX,
            "Warning: Possible stack overflow. Maximum number of elements is {}, {} were allocated",
            STACK_MAX,
            NUM_ELEM
        );
        let array = PointArray2::<NUM_ELEM>::new([1.0; NUM_ELEM], [2.0; NUM_ELEM]).unit();
        let mag_array = PointArray2::<NUM_ELEM>::new(
            [1.0 / 5.0_f64.sqrt(); NUM_ELEM],
            [2.0 / 5.0_f64.sqrt(); NUM_ELEM],
        );
        assert_eq!(array, mag_array);
    }

    #[test]
    fn test_dot_product() {
        // const NUM_ELEM: usize = 10000;
        assert!(
            NUM_ELEM <= STACK_MAX,
            "Warning: Possible stack overflow. Maximum number of elements is {}, {} were allocated",
            STACK_MAX,
            NUM_ELEM
        );
        let array_1 = PointArray2::<NUM_ELEM>::new([1.0; NUM_ELEM], [2.0; NUM_ELEM]);
        let array_2 = PointArray2::<NUM_ELEM>::new([3.0; NUM_ELEM], [4.0; NUM_ELEM]);
        let result = array_1.dot(&array_2);
        let mag_array: [f64; NUM_ELEM] = [11.0; NUM_ELEM];
        assert_eq!(result, mag_array);
    }

    #[test]
    fn test_rotate_90_small() {
        let x = [1.0, -2.0, -3.0, -1.0];
        let y = [1.0, 3.0, -2.0, -4.0];
        let array = PointArray2::new(x, y).rotate(&90.0_f64.to_radians());
        let comp_array = PointArray2::new(
            [-0.9999999999999999, -3.0, 1.9999999999999998, 4.0],
            [0.9999999999999999, -2.0, -3.0, -0.9999999999999998],
        );
        assert_eq!(array, comp_array);
    }

    #[test]
    fn test_rotate_90_full() {
        const NUM_ELEM: usize = 1000;
        assert!(
            NUM_ELEM <= STACK_MAX,
            "Warning: Possible stack overflow. Maximum number of elements is {}, {} were allocated",
            STACK_MAX,
            NUM_ELEM
        );
        let array = PointArray2::<NUM_ELEM>::new([1.0; NUM_ELEM], [1.0; NUM_ELEM])
            .rotate(&90.0_f64.to_radians());
        let comp_array = PointArray2::<NUM_ELEM>::new(
            [-0.9999999999999999; NUM_ELEM],
            [0.9999999999999999; NUM_ELEM],
        );
        assert_eq!(array, comp_array);
    }

    #[test]
    fn test_closure() {
        // const NUM_ELEM: usize = 10000;
        assert!(
            NUM_ELEM <= STACK_MAX,
            "Warning: Possible stack overflow. Maximum number of elements is {}, {} were allocated",
            STACK_MAX,
            NUM_ELEM
        );
        let input_x = 3.0;
        let input_y = 2.0;
        let (output_x, output_y) = hard_function(&input_x, &input_y);
        let closure_array = PointArray2::<NUM_ELEM>::new([input_x; NUM_ELEM], [input_y; NUM_ELEM])
            .gen_point_closure();
        let mag_array = PointArray2::new([output_x; NUM_ELEM], [output_y; NUM_ELEM]);
        assert_eq!(closure_array, mag_array);
    }

    #[test]
    fn test_grid2d() {
        let x_min = -1.0;
        let x_max = 1.0;
        let y_min = -2.0;
        let y_max = 2.0;
        let points = PointArray2::<9>::grid2d(&x_min, &x_max, &y_min, &y_max);
        let comp_points = PointArray2 {
            x: [-1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
            y: [-2.0, 0.0, 2.0, -2.0, 0.0, 2.0, -2.0, 0.0, 2.0],
        };
        assert_eq!(points, comp_points);
    }

    #[test]
    fn test_grid2d_large() {
        let x_min = -3.0;
        let x_max = 3.0;
        let y_min = -6.0;
        let y_max = 6.0;
        let points = PointArray2::<81>::grid2d(&x_min, &x_max, &y_min, &y_max);
        let comp_points = PointArray2 {
            x: [
                -3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -2.25, -2.25, -2.25, -2.25,
                -2.25, -2.25, -2.25, -2.25, -2.25, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5,
                -1.5, -0.75, -0.75, -0.75, -0.75, -0.75, -0.75, -0.75, -0.75, -0.75, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75,
                1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 2.25, 2.25, 2.25, 2.25, 2.25, 2.25,
                2.25, 2.25, 2.25, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,
            ],
            y: [
                -6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, -6.0, -4.5, -3.0, -1.5, 0.0, 1.5,
                3.0, 4.5, 6.0, -6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, -6.0, -4.5, -3.0,
                -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, -6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5, 6.0,
                -6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, -6.0, -4.5, -3.0, -1.5, 0.0, 1.5,
                3.0, 4.5, 6.0, -6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, -6.0, -4.5, -3.0,
                -1.5, 0.0, 1.5, 3.0, 4.5, 6.0,
            ],
        };
        assert_eq!(points, comp_points);
    }

    #[test]
    fn test_cart_prod_3() {
        let x_min = -1.0;
        let x_max = 1.0;
        let y_min = -2.0;
        let y_max = 2.0;
        let (x_vec, y_vec) = cart_prod_2d::<3>(&x_min, &x_max, &y_min, &y_max);
        let x_comp = vec![-1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let y_comp = vec![-2.0, 0.0, 2.0, -2.0, 0.0, 2.0, -2.0, 0.0, 2.0];
        assert_eq!(x_vec, x_comp);
        assert_eq!(y_vec, y_comp);
    }

    #[test]
    fn test_cart_prod_9() {
        let x_min = -3.0;
        let x_max = 3.0;
        let y_min = -6.0;
        let y_max = 6.0;
        let (x_vec, y_vec) = cart_prod_2d::<9>(&x_min, &x_max, &y_min, &y_max);
        let x_comp = vec![
            -3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -3.0, -2.25, -2.25, -2.25, -2.25,
            -2.25, -2.25, -2.25, -2.25, -2.25, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5,
            -1.5, -0.75, -0.75, -0.75, -0.75, -0.75, -0.75, -0.75, -0.75, -0.75, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75, 0.75,
            1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 2.25, 2.25, 2.25, 2.25, 2.25, 2.25, 2.25,
            2.25, 2.25, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0, 3.0,
        ];
        let y_comp = vec![
            -6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, -6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0,
            4.5, 6.0, -6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, -6.0, -4.5, -3.0, -1.5, 0.0,
            1.5, 3.0, 4.5, 6.0, -6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, -6.0, -4.5, -3.0,
            -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, -6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, -6.0,
            -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5, 6.0, -6.0, -4.5, -3.0, -1.5, 0.0, 1.5, 3.0, 4.5,
            6.0,
        ];
        assert_eq!(x_vec, x_comp);
        assert_eq!(y_vec, y_comp);
    }

    #[test]
    fn test_card_3d() {
        let (x, y, z) = cart_prod_3d::<3>();
        let x_comp = vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
        ];
        let y_comp = [
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0,
            2.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0,
        ];
        let z_comp = [
            0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0,
            2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0, 0.0, 1.0, 2.0,
        ];

        assert_eq!(x, x_comp);
        assert_eq!(y, y_comp);
        assert_eq!(z, z_comp);
    }
}
