/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! # PointVec2
//! Structs of vectors to store x and values. This is more efficienct than creating
//! an array of structs, but not as efficient as struct of arrays
//!//!

use crate::magnets::{get_field_2d, Magnet2D};
use crate::points::rotation_2d::rotate_tuple2;
use crate::points::{internal_norm, Point2, Points};
use rayon::prelude::*;

// use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde_derive::{Deserialize, Serialize};

/// Traits for PointVec2
pub trait PointVecs2 {
    /// Set output to be PointVec2
    type Output;
    /// Returns x
    fn x(&self) -> Vec<f64>;
    /// Returns y
    fn y(&self) -> Vec<f64>;
    /// Returns Point with original y, but new x
    fn with_x(&self, x: Vec<f64>) -> Self::Output;
    /// Returns Point with original y, but new x
    fn with_y(&self, y: Vec<f64>) -> Self::Output;
    /// Converts Point2 to PolarPoint
    // fn to_polar(&self) -> PolarPoint;
    /// Returns squared magnitude of PointArray
    fn magnitude_squared(&self) -> Vec<f64>;
    /// Returns magnitude of PointArray
    fn magnitude(&self) -> Vec<f64>;

    // /// Returns elementwise distance from PointArray to origin
    // fn distance_from_origin(&self) -> Vec<f64>;
    // /// Returns elementwise distance from one PointArray to another
    // fn distance_from_point(&self, other: &Self) -> Vec<f64>;
    /// Returns elementwise dot product of two PointArrays
    fn dot(&self, other: &Self) -> Vec<f64>;

    /// Rotates point anti-clockwise about an angle alpha
    fn rotate(&self, alpha: &f64) -> Self::Output;

    /// Returns normalised vector from input vector
    fn unit(&self) -> Self::Output;
    /// Returns a point/vector of zeros
    fn zero(num_elements: &usize) -> Self::Output;
    /// Returns a point/vector of ones
    fn identity(num_elements: &usize) -> Self::Output;
    /// Returns a point/vector with x = 1.0, y = 0.0
    fn i_hat(num_elements: &usize) -> Self::Output;
    /// Returns a point/vector with x = 0.0, y = 1.0
    fn j_hat(num_elements: &usize) -> Self::Output;
}

use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// Point2 Vector struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PointVec2 {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

impl Default for PointVec2 {
    fn default() -> Self {
        Self {
            x: vec![0.0],
            y: vec![0.0],
        }
    }
}

impl PointVec2 {
    /// Constructor for PointVec2
    pub fn new(x: Vec<f64>, y: Vec<f64>) -> Self {
        // TODO: Change assert to error!
        assert!(x.len() == y.len(), "Input vectors must be the same length!");
        Self { x, y }
    }
}

impl Points for PointVec2 {
    type Output = Self;

    fn add_p(&self, other: &Self) -> Self {
        assert_eq!(
            self.x.len(),
            self.y.len(),
            "Error x & y vec of first Point2Vec are not the same length"
        );
        assert_eq!(
            other.x.len(),
            other.y.len(),
            "Error x & y vec of second Point2Vec are not the same length"
        );
        assert_eq!(
            self.x.len(),
            other.x.len(),
            "Error both structs should have the same number of elements in x and y"
        );

        let mut result: Self = PointVec2::new(
            Vec::with_capacity(self.x.len()),
            Vec::with_capacity(self.x.len()),
        );

        result.x = self
            .x
            .par_iter()
            .zip(&other.x)
            .map(|(x, y)| x + y)
            .collect();
        result.y = self
            .y
            .par_iter()
            .zip(&other.y)
            .map(|(x, y)| x + y)
            .collect();
        result
    }

    fn sub_p(&self, other: &Self) -> Self {
        assert_eq!(
            self.x.len(),
            self.y.len(),
            "Error x & y vec of first Point2Vec are not the same length"
        );
        assert_eq!(
            other.x.len(),
            other.y.len(),
            "Error x & y vec of second Point2Vec are not the same length"
        );
        assert_eq!(
            self.x.len(),
            other.x.len(),
            "Error both structs should have the same number of elements in x and y"
        );

        let mut result: Self = PointVec2::new(
            Vec::with_capacity(self.x.len()),
            Vec::with_capacity(self.x.len()),
        );

        result.x = self
            .x
            .par_iter()
            .zip(&other.x)
            .map(|(x, y)| x - y)
            .collect();
        result.y = self
            .y
            .par_iter()
            .zip(&other.y)
            .map(|(x, y)| x - y)
            .collect();
        result
    }

    fn mul_p(&self, other: &Self) -> Self {
        assert_eq!(
            self.x.len(),
            self.y.len(),
            "Error x & y vec of first Point2Vec are not the same length"
        );
        assert_eq!(
            other.x.len(),
            other.y.len(),
            "Error x & y vec of second Point2Vec are not the same length"
        );
        assert_eq!(
            self.x.len(),
            other.x.len(),
            "Error both structs should have the same number of elements in x and y"
        );

        let mut result: Self = PointVec2::new(
            Vec::with_capacity(self.x.len()),
            Vec::with_capacity(self.x.len()),
        );

        result.x = self
            .x
            .par_iter()
            .zip(&other.x)
            .map(|(x, y)| x * y)
            .collect();
        result.y = self
            .y
            .par_iter()
            .zip(&other.y)
            .map(|(x, y)| x * y)
            .collect();
        result
    }

    fn div_p(&self, other: &Self) -> Self {
        assert_eq!(
            self.x.len(),
            self.y.len(),
            "Error x & y vec of first Point2Vec are not the same length"
        );
        assert_eq!(
            other.x.len(),
            other.y.len(),
            "Error x & y vec of second Point2Vec are not the same length"
        );
        assert_eq!(
            self.x.len(),
            other.x.len(),
            "Error both structs should have the same number of elements in x and y"
        );

        let mut result: Self = PointVec2::new(
            Vec::with_capacity(self.x.len()),
            Vec::with_capacity(self.x.len()),
        );

        result.x = self
            .x
            .par_iter()
            .zip(&other.x)
            .map(|(x, y)| x / y)
            .collect();
        result.y = self
            .y
            .par_iter()
            .zip(&other.y)
            .map(|(x, y)| x / y)
            .collect();
        result
    }

    fn neg_p(&self) -> Self {
        assert_eq!(
            self.x.len(),
            self.y.len(),
            "Error x & y vec of Point2Vec are not the same length"
        );

        let mut result: Self = PointVec2::new(
            Vec::with_capacity(self.x.len()),
            Vec::with_capacity(self.x.len()),
        );

        result.x = self.x.par_iter().map(|x| -1.0 * x).collect();
        result.y = self.y.par_iter().map(|y| -1.0 * y).collect();
        result
    }

    fn scale(&self, s: f64) -> Self {
        assert_eq!(
            self.x.len(),
            self.y.len(),
            "Error x & y vec of Point2Vec are not the same length"
        );

        let mut result: Self = PointVec2::new(
            Vec::with_capacity(self.x.len()),
            Vec::with_capacity(self.x.len()),
        );

        result.x = self.x.par_iter().map(|x| x * s).collect();
        result.y = self.y.par_iter().map(|y| y * s).collect();
        result
    }

    fn round(&self) -> Self {
        assert_eq!(
            self.x.len(),
            self.y.len(),
            "Error x & y vec of Point2Vec are not the same length"
        );

        let mut result: Self = PointVec2::new(
            Vec::with_capacity(self.x.len()),
            Vec::with_capacity(self.x.len()),
        );

        result.x = self.x.par_iter().map(|x| x.round()).collect();
        result.y = self.y.par_iter().map(|y| y.round()).collect();
        result
    }
}

impl Add for PointVec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_p(&other)
    }
}

impl AddAssign for PointVec2 {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(
            self.x.len(),
            self.y.len(),
            "Error x & y vec of first Point2Vec are not the same length"
        );
        assert_eq!(
            other.x.len(),
            other.y.len(),
            "Error x & y vec of second Point2Vec are not the same length"
        );
        assert_eq!(
            self.x.len(),
            other.x.len(),
            "Error both structs should have the same number of elements in x and y"
        );

        self.x = self
            .x
            .par_iter()
            .zip(&other.x)
            .map(|(x, y)| x + y)
            .collect();
        self.y = self
            .y
            .par_iter()
            .zip(&other.y)
            .map(|(x, y)| x + y)
            .collect();
    }
}

impl Sub for PointVec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.sub_p(&other)
    }
}

impl Mul for PointVec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_p(&other)
    }
}

impl Div for PointVec2 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.div_p(&other)
    }
}

impl Neg for PointVec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.neg_p()
    }
}

impl PointVecs2 for PointVec2 {
    type Output = Self;

    /// Returns x
    fn x(&self) -> Vec<f64> {
        self.x.clone()
    }

    /// Returns y
    fn y(&self) -> Vec<f64> {
        self.y.clone()
    }

    /// Returns PointArray with clone of original y, but new x
    fn with_x(&self, x: Vec<f64>) -> Self::Output {
        assert_eq!(x.len(), self.y.len());
        PointVec2::new(x, self.y.clone())
    }

    /// Returns PointArray with clone of original x, but new y
    fn with_y(&self, y: Vec<f64>) -> Self::Output {
        assert_eq!(y.len(), self.x.len());
        PointVec2::new(self.x.clone(), y)
    }
    /// Converts Point2 to PolarPoint
    // fn to_polar(&self) -> PolarPoint;
    /// Returns squared magnitude of PointArray
    fn magnitude_squared(&self) -> Vec<f64> {
        self.x
            .par_iter()
            .zip(self.y.par_iter())
            .map(|(x, y)| x * x + y * y)
            .collect::<Vec<f64>>()
    }

    /// Returns magnitude of PointArray
    fn magnitude(&self) -> Vec<f64> {
        self.x
            .par_iter()
            .zip(self.y.par_iter())
            .map(|(x, y)| (x * x + y * y).sqrt())
            .collect::<Vec<f64>>()
    }

    //     /// Returns elementwise distance from PointArray to origin
    //     fn distance_from_origin(&self) -> Vec<f64>;
    //     /// Returns elementwise distance from one PointArray to another
    //     fn distance_from_point(&self, other: &Self) -> Vec<f64>;
    //
    //

    /// Returns elementwise dot product of two PointArrays
    fn dot(&self, other: &Self) -> Vec<f64> {
        self.x
            .par_iter()
            .zip(self.y.par_iter())
            .zip(other.x.par_iter())
            .zip(other.y.par_iter())
            .map(|(((x1, y1), x2), y2)| (x1 * x2 + y1 * y2))
            .collect::<Vec<f64>>()
    }

    /// Rotates point anti-clockwise about an angle alpha
    fn rotate(&self, alpha: &f64) -> Self::Output {
        let (x_rot, y_rot) = self
            .x
            .par_iter()
            .zip(self.y.par_iter())
            .map(|(x, y)| rotate_tuple2((x, y), alpha))
            .collect::<(Vec<f64>, Vec<f64>)>();
        PointVec2::new(x_rot, y_rot)
    }

    /// Returns normalised vector from input vector
    fn unit(&self) -> Self::Output {
        let (x_local, y_local) = self
            .x
            .par_iter()
            .zip(self.y.par_iter())
            .map(|(x, y)| internal_norm(x, y))
            .collect::<(Vec<f64>, Vec<f64>)>();
        PointVec2::new(x_local, y_local)
    }
    //     /// Returns a point/vector of zeros
    fn zero(num_elements: &usize) -> Self::Output {
        Self {
            x: vec![0.0; *num_elements],
            y: vec![0.0; *num_elements],
        }
    }
    /// Returns a point/vector of ones
    fn identity(num_elements: &usize) -> Self::Output {
        Self {
            x: vec![1.0; *num_elements],
            y: vec![1.0; *num_elements],
        }
    }
    /// Returns a point/vector with x = 1.0, y = 0.0
    fn i_hat(num_elements: &usize) -> Self::Output {
        Self {
            x: vec![1.0; *num_elements],
            y: vec![0.0; *num_elements],
        }
    }
    /// Returns a point/vector with x = 0.0, y = 1.0
    fn j_hat(num_elements: &usize) -> Self::Output {
        Self {
            x: vec![0.0; *num_elements],
            y: vec![1.0; *num_elements],
        }
    }
}

impl PointVec2 {
    /// Method to calculate an arbitrary function on a PointVec2 struct
    ///  - Iterates over the point array
    ///  - passes the internal x,y pairs to a function `f` by reference
    ///  - The function returns a tuple (f64, f64)
    ///  - This is collected in a tuple of Vec
    ///  - And finally the vec are wrapped in a PointVec2 struct
    pub fn apply_function(&self, f: fn(&f64, &f64) -> (f64, f64)) -> PointVec2 {
        let (x_local, y_local) = self
            .x
            .par_iter()
            .zip(self.y.par_iter())
            .map(|(x, y)| f(x, y))
            .collect::<(Vec<f64>, Vec<f64>)>();
        PointVec2::new(x_local, y_local)
    }

    /// Returns the magnetic field for a series of points due to all magnets
    pub fn get_field(&self, magnet_list: &[Magnet2D]) -> PointVec2 {
        // let pb = ProgressBar::new(self.x.len() as u64);
        let (x_local, y_local) = self
            .x
            .par_iter()
            .zip(self.y.par_iter())
            // .progress_with(pb)
            .map(|(x, y)| get_field_2d(magnet_list, (x, y)).unwrap())
            .collect::<(Vec<f64>, Vec<f64>)>();

        PointVec2::new(x_local, y_local)
    }

    /// Returns Point2 for a given index of a PointVec2 struct
    pub fn to_point(&self, index: usize) -> Point2 {
        Point2::new(self.x[index], self.y[index])
    }
}

// Generates a 2D grid of points (x_min:x_max, y_min:y_max) in the form of
/// a tuple of Vectors (x, y) where x,y are Vec<f64>
///
/// This version generates the points using an iterator where the range is converted
/// into a parallel iter (Rayon), a flat map
pub fn cart_prod_2d_vec(start: &Point2, stop: &Point2, num_points: &usize) -> PointVec2 {
    let xs = (0_usize..*num_points).into_par_iter();
    let ys = (0_usize..*num_points).into_par_iter();

    let distance = *stop - *start;
    let step_x = distance.x / (num_points - 1) as f64;
    let step_y = distance.y / (num_points - 1) as f64;

    let (xx, yy): (Vec<f64>, Vec<f64>) = xs
        .flat_map(|x| {
            ys.clone()
                .map(move |y| (start.x + (x as f64) * step_x, start.y + y as f64 * step_y))
        })
        .unzip();

    PointVec2 { x: xx, y: yy }
}

/// Generates a line of points in 2D from a Point2 start point to a Point2 end point
/// which is stored in a PointVec2 struct
pub fn gen_line_2d(start: &Point2, stop: &Point2, num_points: &usize) -> PointVec2 {
    let xs = (0_usize..*num_points).into_par_iter();
    let ys = (0_usize..*num_points).into_par_iter();

    let distance = *stop - *start;
    let step_x = distance.x / (num_points - 1) as f64;
    let step_y = distance.y / (num_points - 1) as f64;

    let (xx, yy): (Vec<f64>, Vec<f64>) = xs
        .zip(ys)
        .map(|(x, y)| (start.x + (x as f64) * step_x, start.y + y as f64 * step_y))
        .unzip();

    PointVec2 { x: xx, y: yy }
}

#[cfg(test)]
mod tests {
    use crate::points::{Point2, PointVecs2};

    use super::{gen_line_2d, PointVec2};
    fn hard_function(x: &f64, y: &f64) -> (f64, f64) {
        (x.powi(3).sqrt(), y.powi(3).sqrt())
    }

    #[test]
    fn test_default() {
        let vec = PointVec2::default();
        let comp_vec = vec![0.0];
        assert_eq!(vec.x, comp_vec, "Test x");
        assert_eq!(vec.y, comp_vec, "Test y");
    }

    #[test]
    fn test_new() {
        let vec = PointVec2::new(vec![1.0, 2.0], vec![3.0, 4.0]);
        assert_eq!(vec.x, vec![1.0, 2.0], "Test x");
        assert_eq!(vec.y, vec![3.0, 4.0], "Test y");
    }

    #[test]
    fn test_add() {
        let vec = PointVec2::new(vec![1.0, 2.0], vec![3.0, 4.0]);
        let vec_2 = PointVec2::new(vec![5.0, 6.0], vec![7.0, 8.0]);
        let comp_vec = PointVec2::new(vec![6.0, 8.0], vec![10.0, 12.0]);
        assert_eq!(vec + vec_2, comp_vec);
    }

    #[test]
    fn test_subtract() {
        let vec = PointVec2::new(vec![1.0, 2.0], vec![3.0, 4.0]);
        let vec_2 = PointVec2::new(vec![5.0, 6.0], vec![7.0, 8.0]);
        let comp_vec = PointVec2::new(vec![-4.0, -4.0], vec![-4.0, -4.0]);
        assert_eq!(vec - vec_2, comp_vec);
    }

    #[test]
    fn test_multiply() {
        let vec = PointVec2::new(vec![1.0, 2.0], vec![3.0, 4.0]);
        let vec_2 = PointVec2::new(vec![5.0, 6.0], vec![7.0, 8.0]);
        let comp_vec = PointVec2::new(vec![5.0, 12.0], vec![21.0, 32.0]);
        assert_eq!(vec * vec_2, comp_vec);
    }

    #[test]
    fn test_divide() {
        let vec = PointVec2::new(vec![1.0, 2.0], vec![3.0, 4.0]);
        let vec_2 = PointVec2::new(vec![5.0, 6.0], vec![7.0, 8.0]);
        let comp_vec = PointVec2::new(vec![1.0 / 5.0, 2.0 / 6.0], vec![3.0 / 7.0, 4.0 / 8.0]);
        assert_eq!(vec / vec_2, comp_vec);
    }

    #[test]
    fn test_zeros() {
        let n_elem = 1000;
        let vec = PointVec2::zero(&n_elem);
        let comp_vec = PointVec2::new(vec![0.0; n_elem], vec![0.0; n_elem]);
        assert_eq!(vec, comp_vec);
    }

    #[test]
    fn test_magnitude_squared() {
        let vec = PointVec2::new(vec![1.0, 2.0], vec![3.0, 4.0]).magnitude_squared();
        let comp_vec = vec![10.0, 20.0];
        assert_eq!(vec, comp_vec);
    }

    #[test]
    fn test_magnitude() {
        let vec = PointVec2::new(vec![1.0, 2.0], vec![3.0, 4.0]).magnitude();
        let comp_vec = vec![10.0_f64.sqrt(), 20.0_f64.sqrt()];
        assert_eq!(vec, comp_vec);
    }

    #[test]
    fn test_magnitude_1e4() {
        let n_elem: usize = 10000;
        let vec = PointVec2::new(vec![1.0; n_elem], vec![2.0; n_elem]).magnitude();
        let comp_vec = vec![5.0_f64.sqrt(); n_elem];
        assert_eq!(vec, comp_vec);
    }

    #[test]
    fn test_unit() {
        let n_elem: usize = 1000;
        let vec = PointVec2::new(vec![3.0; n_elem], vec![4.0; n_elem]).unit();
        let comp_vec = PointVec2::new(vec![3.0 / 5.0; n_elem], vec![4.0 / 5.0; n_elem]).unit();
        assert_eq!(vec, comp_vec);
    }

    #[test]
    fn test_dot_product() {
        let n_elem: usize = 1000;
        let vec = PointVec2::new(vec![1.0; n_elem], vec![2.0; n_elem]);
        let vec_2 = PointVec2::new(vec![3.0; n_elem], vec![4.0; n_elem]);
        let result = vec.dot(&vec_2);
        let comp_vec = vec![11.0; n_elem];
        assert_eq!(result, comp_vec);
    }

    #[test]
    fn test_rotate_90_small() {
        let x = vec![1.0, -2.0, -3.0, -1.0];
        let y = vec![1.0, 3.0, -2.0, -4.0];
        let vec = PointVec2::new(x, y).rotate(&90.0_f64.to_radians());
        let comp_vec = PointVec2::new(
            vec![-0.9999999999999999, -3.0, 1.9999999999999998, 4.0],
            vec![1.0, -1.9999999999999998, -3.0, -1.0000000000000002],
        );
        assert_eq!(vec, comp_vec);
    }

    #[test]
    fn test_rotate_90_full() {
        let n_elem = 2;
        let vec = PointVec2::identity(&n_elem).rotate(&90.0_f64.to_radians());
        let comp_vec = PointVec2::new(vec![-0.9999999999999999; n_elem], vec![1.0; n_elem]);
        assert_eq!(vec, comp_vec);
    }

    #[test]
    fn test_closure() {
        let n_elem = 100;
        let input_x = 3.0;
        let input_y = 2.0;
        let (output_x, output_y) = hard_function(&input_x, &input_y);

        let closure_array = PointVec2::new(vec![input_x; n_elem], vec![input_y; n_elem])
            .apply_function(hard_function);
        let mag_array = PointVec2::new(vec![output_x; n_elem], vec![output_y; n_elem]);
        assert_eq!(closure_array, mag_array);
    }

    #[test]
    fn test_gen_line_2d() {
        let n_elem = 5;
        let start = Point2::new(0.0, 0.0);
        let stop = Point2::new(2.0, 4.0);

        let output = gen_line_2d(&start, &stop, &n_elem);
        let comp_array = PointVec2 {
            x: vec![0.0, 0.5, 1.0, 1.5, 2.0],
            y: vec![0.0, 1.0, 2.0, 3.0, 4.0],
        };
        assert_eq!(output, comp_array);
    }
}
