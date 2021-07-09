/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! PolarPoints
//! 2D struct for handling points in polar coordinates and their associated methods
//!
use crate::points::{Point2, Points};
use crate::utils::conversions::pol2cart;

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
        write!(f, "(rho: {}, phi: {}˚)", self.rho, self.phi.to_degrees())
    }
}

/// Traits specific to PolarPoint
pub trait PolarPoints {
    type Output;
    fn rho(&self) -> f64;
    fn phi(&self) -> f64;
    fn with_rho(&self, rho: f64) -> Self::Output;
    fn with_phi(&self, phi: f64) -> Self::Output;

    fn to_cartesian(&self) -> Point2;

    fn magnitude(&self) -> f64;
    fn magnitude_squared(&self) -> f64;
    fn distance_from_origin(&self) -> f64;
    fn distance_from_point(&self, other: &Self) -> f64;
    // fn dot(&self, other: &Self) -> f64;
    fn unit(&self) -> Self::Output;
    fn zero() -> Self::Output;
    fn identity() -> Self::Output;
}

impl Points for PolarPoint {
    type Output = PolarPoint;

    fn add_p(&self, other: &Self) -> PolarPoint {
        PolarPoint {
            rho: self.rho + other.rho,
            phi: self.phi + other.phi,
        }
    }

    fn sub_p(&self, other: &Self) -> PolarPoint {
        PolarPoint {
            rho: self.rho - other.rho,
            phi: self.phi - other.phi,
        }
    }

    fn mul_p(&self, other: &Self) -> PolarPoint {
        PolarPoint {
            rho: self.rho * other.rho,
            phi: self.phi * other.phi,
        }
    }

    fn div_p(&self, other: &Self) -> PolarPoint {
        PolarPoint {
            rho: self.rho / other.rho,
            phi: self.phi / other.phi,
        }
    }

    fn neg_p(&self) -> PolarPoint {
        PolarPoint {
            rho: -self.rho,
            phi: -self.phi,
        }
    }

    fn scale(&self, s: f64) -> PolarPoint {
        PolarPoint {
            rho: self.rho * s,
            phi: self.phi * s,
        }
    }

    fn round(&self) -> PolarPoint {
        PolarPoint {
            rho: self.rho.round(),
            phi: self.phi.round(),
        }
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
        pol2cart(self)
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
    use crate::points::{Point2, Points2};
    // use crate::utils::comparison::nearly_equal;

    #[test]
    fn sum_points() {
        let sum = Point2::i_hat() + Point2::j_hat();
        assert_eq!(Point2 { x: 1.0, y: 1.0 }, sum);
    }
}
