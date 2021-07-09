/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! Points 2
//! 2D structs for handling points and their associated methods
//!
use crate::points::Points;
use crate::utils::conversions::{cart2pol, pol2cart};
use crate::PI;

use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

/// PolarPoint
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PolarPoint {
    /// radial coordinate
    pub rho: f64,
    /// azimuthal coordinate
    pub phi: f64,
}

impl PolarPoint {
    /// Constructor method that generates a PolarPoint struct by casting the
    /// generic input parameters to float64
    pub fn new<T: Into<f64>>(rho: T, phi: T) -> PolarPoint {
        PolarPoint {
            rho: rho.into(),
            phi: phi.into(),
        }
    }
}

impl fmt::Display for PolarPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(rho: {}, phi: {}˚)", self.rho, self.phi * 180.0 / PI)
    }
}

impl PolarPoints for PolarPoint {
    type Output = PolarPoint;

    fn rho(&self) -> f64 {
        self.rho
    }
    fn phi(&self) -> f64 {
        self.phi
    }
    fn with_rho(&self, rho: f64) -> Self::Output {
        PolarPoint { rho, phi: self.phi }
    }
    fn with_phi(&self, phi: f64) -> Self::Output {
        PolarPoint { rho: self.rho, phi }
    }

    fn to_cartesian(&self) -> Point2 {
        pol2cart(&self)
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
        (self.rho.powi(2) + other.rho.powi(2)
            - 2.0 * self.rho * other.rho * (other.phi - self.phi).cos())
        .sqrt()
    }

    fn unit(&self) -> Self::Output {
        PolarPoint {
            rho: 1.0_f64,
            phi: self.phi,
        }
    }
    fn zero() -> Self::Output {
        PolarPoint {
            rho: 0.0_f64,
            phi: 0.0_f64,
        }
    }
    fn identity() -> Self::Output {
        PolarPoint {
            rho: 1.0_f64,
            phi: 1.0_f64,
        }
    }
}

impl Add for PolarPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_p(&other)
    }
}

impl AddAssign for PolarPoint {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            rho: self.rho + other.rho,
            phi: self.phi + other.phi,
        };
    }
}

impl Sub for PolarPoint {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.sub_p(&other)
    }
}

impl Mul for PolarPoint {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_p(&other)
    }
}

impl Div for PolarPoint {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.div_p(&other)
    }
}

impl Neg for PolarPoint {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.neg_p()
    }
}

#[cfg(test)]
mod tests {
    use crate::points::points2::{Point2, Points, Points2};
    use crate::utils::comparison::nearly_equal;

    #[test]
    fn sum_points() {
        let sum = Point2::i_hat() + Point2::j_hat();
        assert_eq!(Point2 { x: 1.0, y: 1.0 }, sum);
    }
}
