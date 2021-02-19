/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! Points 2
//! 2D structs for handling points and their associated methods
//! 
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// Point2 
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Point2 {
    ///X - coordinate
    pub x: f64,
    ///y - coordinate
    pub y: f64,
}

impl Point2 {
    /// Constructor method that generates a Point2 struct by casting the 
    /// generic input parameters to float64
    pub fn new<T: Into<f64>>(x: T, y: T) -> Point2 {
        Point2 {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// # Points2 Traits
/// Overloading of +-*/, as well as helper functions
pub trait Points2 {
    /// Set output to be Points2
    type Output;
    /// Implelment addition
    fn add_p(&self, other: &Self) -> Self::Output;
    /// Subtraction
    fn sub_p(&self, other: &Self) -> Self::Output;
    fn mul_p(&self, other: &Self) -> Self::Output;
    fn div_p(&self, other: &Self) -> Self::Output;
    fn neg_p(&self) -> Self::Output;
    fn scale(&self, s: f64) -> Self::Output;
    fn round(&self) -> Self::Output;

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn with_x(&self, x: f64) -> Self::Output;
    fn with_y(&self, y: f64) -> Self::Output;

    fn magnitude(&self) -> f64;
    fn magnitude_squared(&self) -> f64;
    fn distance_from_origin(&self) -> f64;
    fn distance_from_point(&self, other: &Self) -> f64;

    fn dot(&self, other: &Self) -> f64;
    fn unit(&self) -> Self::Output;
    fn zero() -> Self::Output;
    fn identity() -> Self::Output;
    fn i_hat() -> Self::Output;
    fn j_hat() -> Self::Output;
}

impl Points2 for Point2 {
    type Output = Point2;

    fn add_p(&self, other: &Self) -> Point2 {
        Point2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub_p(&self, other: &Self) -> Point2 {
        Point2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn mul_p(&self, other: &Self) -> Point2 {
        Point2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

    fn div_p(&self, other: &Self) -> Point2 {
        Point2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }

    fn neg_p(&self) -> Point2 {
        Point2 {
            x: -self.x,
            y: -self.y,
        }
    }

    fn scale(&self, s: f64) -> Point2 {
        Point2 {
            x: self.x * s,
            y: self.y * s,
        }
    }

    fn round(&self) -> Point2 {
        Point2 {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn with_x(&self, x: f64) -> Point2 {
        Point2 { x, y: self.y }
    }

    fn with_y(&self, y: f64) -> Point2 {
        Point2 { x: self.x, y }
    }

    fn magnitude_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    fn distance_from_origin(&self) -> f64 {
        self.magnitude()
    }

    fn distance_from_point(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn unit(&self) -> Point2 {
        self.scale(1.0 / self.magnitude())
    }

    fn zero() -> Point2 {
        Point2 {
            x: 0.0_f64,
            y: 0.0_f64,
        }
    }

    fn identity() -> Point2 {
        Point2 {
            x: 1.0_f64,
            y: 1.0_f64,
        }
    }

    fn i_hat() -> Point2 {
        Point2 { x: 1.0, y: 0.0 }
    }

    fn j_hat() -> Point2 {
        Point2 { x: 0.0, y: 1.0 }
    }
}

impl Add for Point2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_p(&other)
    }
}

impl AddAssign for Point2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Point2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.sub_p(&other)
    }
}

impl Mul for Point2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_p(&other)
    }
}

impl Div for Point2 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.div_p(&other)
    }
}

impl Neg for Point2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.neg_p()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::nearly_equal;
    use crate::utils::points2::{Point2, Points2};

    #[test]
    fn sum_points() {
        let sum = Point2::i_hat() + Point2::j_hat();
        assert_eq!(Point2 { x: 1.0, y: 1.0 }, sum);
    }

    #[test]
    fn sub_points() {
        let p1 = Point2 { x: 2.0, y: 4.0 };
        let p2 = Point2 { x: 0.3, y: 1.5 };
        let sub = p1 - p2;
        assert_eq!(Point2 { x: 1.7, y: 2.5 }, sub);
    }

    #[test]
    fn mul_points() {
        let p1 = Point2 { x: 2.0, y: 4.0 };
        let p2 = Point2 { x: 0.3, y: 1.5 };
        let result = p1 * p2;
        assert_eq!(Point2 { x: 0.6, y: 6.0 }, result);
    }

    #[test]
    fn div_points() {
        let p1 = Point2 { x: 2.0, y: 4.0 };
        let p2 = Point2 { x: 0.2, y: 1.6 };
        let result = p1 / p2;
        assert_eq!(Point2 { x: 10.0, y: 2.5 }, result);
    }

    #[test]
    fn neg_point() {
        let p1 = Point2 { x: 2.0, y: 4.0 };
        let result = -p1;
        assert_eq!(Point2 { x: -2.0, y: -4.0 }, result);
    }

    #[test]
    fn scale_point() {
        let p1 = Point2 { x: 2.0, y: 4.0 };
        let result = p1.scale(3.0);
        assert_eq!(Point2 { x: 6.0, y: 12.0 }, result);
    }

    #[test]
    fn round_point() {
        let p1 = Point2 { x: 2.33, y: 4.55 };
        let result = p1.round();
        assert_eq!(Point2 { x: 2.0, y: 5.0 }, result);
    }

    #[test]
    fn unit_magnitude_squared() {
        let mag_squared = Point2::i_hat().magnitude_squared();
        assert_eq!(1., mag_squared);
    }

    #[test]
    fn magnitude_point() {
        let p1 = Point2 { x: 3.0, y: 4.0 };
        let result = p1.magnitude();
        assert_eq!(5., result);
    }

    #[test]
    fn distance_from_origin_point() {
        let p1 = Point2 { x: 3.0, y: 4.0 };
        let result = p1.distance_from_origin();
        assert_eq!(5., result);
    }

    #[test]
    fn distance_from_point_test() {
        let p1 = Point2 { x: 3.0, y: 4.0 };
        let p2 = Point2 { x: 4.5, y: -3.2 };
        let result = p1.distance_from_point(&p2);
        assert_eq!(54.09_f64.sqrt(), result);
    }

    #[test]
    fn dot_product() {
        let p1 = Point2 { x: 1.0, y: 2.0 };
        let p2 = Point2 { x: 3.0, y: 4.0 };
        let result = p1.dot(&p2);
        assert_eq!(11.0, result);
    }

    #[test]
    fn unit_vector() {
        let p1 = Point2 { x: 3.0, y: 4.0 };
        let norm_p1 = p1.unit();

        let result = nearly_equal(norm_p1.x, 3.0 / 5.0) && nearly_equal(norm_p1.y, 4.0 / 5.0);

        assert!(result);
    }
}
