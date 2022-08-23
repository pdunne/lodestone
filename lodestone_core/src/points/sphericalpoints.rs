/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! SphericalPoints
//! 3D struct for handling points in spherical coordinates and their associated methods
//!
use crate::points::{Point3, Points};
use crate::utils::conversions::sph2cart;

use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// SphericalPoint
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SphericalPoint {
    /// radial coordinate
    pub rho: f64,
    /// azimuthal coordinate
    pub phi: f64,
    /// polar coordinate
    pub theta: f64,
}

impl SphericalPoint {
    /// Constructor method that generates a SphericalPoint struct by casting the
    /// generic input parameters to float64
    pub fn new<T: Into<f64>>(rho: T, phi: T, theta: T) -> SphericalPoint {
        SphericalPoint {
            rho: rho.into(),
            phi: phi.into(),
            theta: theta.into(),
        }
    }

    /// Returns a point struct as a tuple (using radians)
    pub fn as_tuple(&self) -> (f64, f64, f64) {
        (self.rho, self.phi, self.theta)
    }

    /// Returns a point struct as an array (using radians)
    pub fn as_array(&self) -> [f64; 3] {
        [self.rho, self.phi, self.theta]
    }
}

impl fmt::Display for SphericalPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(rho: {}, phi: {}, theta: {})",
            self.rho, self.phi, self.theta
        )
    }
}

/// Traits specific to SphericalPoint
pub trait SphericalPoints {
    type Output;
    fn rho(&self) -> f64;
    fn phi(&self) -> f64;
    fn theta(&self) -> f64;

    fn with_rho(&self, rho: f64) -> Self::Output;
    fn with_phi(&self, phi: f64) -> Self::Output;
    fn with_theta(&self, theta: f64) -> Self::Output;

    fn to_cartesian(&self) -> Point3;

    fn magnitude(&self) -> f64;
    fn magnitude_squared(&self) -> f64;
    fn distance_from_origin(&self) -> f64;
    fn distance_from_point(&self, other: &Self) -> f64;
    // fn dot(&self, other: &Self) -> f64;
    fn unit(&self) -> Self::Output;
    fn zero() -> Self::Output;
    fn identity() -> Self::Output;
}

impl Points for SphericalPoint {
    type Output = SphericalPoint;

    fn add_p(&self, other: &Self) -> SphericalPoint {
        SphericalPoint {
            rho: self.rho + other.rho,
            phi: self.phi + other.phi,
            theta: self.theta + other.theta,
        }
    }

    fn sub_p(&self, other: &Self) -> SphericalPoint {
        SphericalPoint {
            rho: self.rho - other.rho,
            phi: self.phi - other.phi,
            theta: self.theta - other.theta,
        }
    }

    fn mul_p(&self, other: &Self) -> SphericalPoint {
        SphericalPoint {
            rho: self.rho * other.rho,
            phi: self.phi * other.phi,
            theta: self.theta * other.theta,
        }
    }

    fn div_p(&self, other: &Self) -> SphericalPoint {
        SphericalPoint {
            rho: self.rho / other.rho,
            phi: self.phi / other.phi,
            theta: self.theta / other.theta,
        }
    }

    fn neg_p(&self) -> SphericalPoint {
        SphericalPoint {
            rho: -self.rho,
            phi: -self.phi,
            theta: -self.theta,
        }
    }

    fn scale(&self, s: f64) -> SphericalPoint {
        SphericalPoint {
            rho: self.rho * s,
            phi: self.phi * s,
            theta: self.theta * s,
        }
    }

    fn round(&self) -> SphericalPoint {
        SphericalPoint {
            rho: self.rho.round(),
            phi: self.phi.round(),
            theta: self.theta.round(),
        }
    }
}

impl SphericalPoints for SphericalPoint {
    type Output = SphericalPoint;

    fn rho(&self) -> f64 {
        self.rho
    }
    fn phi(&self) -> f64 {
        self.phi
    }
    fn theta(&self) -> f64 {
        self.theta
    }
    fn with_rho(&self, rho: f64) -> Self::Output {
        SphericalPoint {
            rho,
            phi: self.phi,
            theta: self.theta,
        }
    }
    fn with_phi(&self, phi: f64) -> Self::Output {
        SphericalPoint {
            rho: self.rho,
            phi,
            theta: self.theta,
        }
    }

    fn with_theta(&self, theta: f64) -> Self::Output {
        SphericalPoint {
            rho: self.rho,
            phi: self.phi,
            theta,
        }
    }

    fn to_cartesian(&self) -> Point3 {
        sph2cart(self)
    }

    fn magnitude(&self) -> f64 {
        self.rho
    }

    fn magnitude_squared(&self) -> f64 {
        self.rho.powi(2)
    }

    fn distance_from_origin(&self) -> f64 {
        self.rho
    }

    fn distance_from_point(&self, other: &Self) -> f64 {
        let sin_theta_1 = self.theta.sin();
        let sin_theta_2 = other.theta.sin();
        let cos_d_theta = (self.theta - other.theta).cos();
        let cos_d_phi = (self.phi - other.phi).cos();
        let part_1 = self.rho.powi(2) + other.rho.powi(2);
        let part_2 = 2.0 * self.rho * other.rho * cos_d_theta;
        let part_3 = 2.0 * self.rho * other.rho * sin_theta_1 * sin_theta_2 * (cos_d_phi - 1.0);

        (part_1 - part_2 - part_3).sqrt()
    }

    fn unit(&self) -> Self::Output {
        SphericalPoint {
            rho: 1.0_f64,
            phi: self.phi,
            theta: self.theta,
        }
    }
    fn zero() -> Self::Output {
        SphericalPoint {
            rho: 0.0_f64,
            phi: 0.0,
            theta: 0.0,
        }
    }
    fn identity() -> Self::Output {
        SphericalPoint {
            rho: 1.0_f64,
            phi: 1.0,
            theta: 1.0,
        }
    }
}

impl Add for SphericalPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_p(&other)
    }
}

impl AddAssign for SphericalPoint {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            rho: self.rho + other.rho,
            phi: self.phi + other.phi,
            theta: self.theta + other.theta,
        };
    }
}

impl Sub for SphericalPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.sub_p(&other)
    }
}

impl Mul for SphericalPoint {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_p(&other)
    }
}

impl Div for SphericalPoint {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.div_p(&other)
    }
}

impl Neg for SphericalPoint {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.neg_p()
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::points::{Point3, Points3};
//     use crate::utils::comparison::nearly_equal;
//     // TODO:
//     // - SphericalPoint tests
//     #[test]
//     fn sum_points() {
//         let sum = Point3::i_hat() + Point3::j_hat() + Point3::k_hat();
//         assert!(nearly_equal(
//             Point3 {
//                 x: 1.0,
//                 y: 1.0,
//                 z: 1.0,
//             },
//             sum
//         ))
//     }
// }
