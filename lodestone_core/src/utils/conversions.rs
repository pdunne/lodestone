/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

//! Conversions
//! The utils module contains utilities to convert between different coordinates
//! systems and between degrees and radians
//!

use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use crate::points::{Point2, Point3, Points2, Points3, PolarPoint, SphericalPoint};
use serde_derive::{Deserialize, Serialize};

/// Angle enum for converting between radians and degrees
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum Angle {
    /// Angle stored in degrees
    Degrees(f64),
    /// Angle stored in radians
    Radians(f64),
}

impl Angle {
    /// Converts radian angle to degrees as a float. If the angle is already in degrees,
    /// it returns itself
    pub fn to_degrees(self) -> f64 {
        match self {
            Angle::Radians(val) => val.to_degrees(),
            Angle::Degrees(val) => val,
        }
    }

    /// Converts degree angle to radians as a float. If the angle is already in radians,
    /// it returns itself
    pub fn to_radians(self) -> f64 {
        match self {
            Angle::Degrees(val) => val.to_radians(),
            Angle::Radians(val) => val,
        }
    }

    fn add_p(&self, other: &Self) -> Self {
        Angle::Radians(self.to_radians() + other.to_radians())
    }

    fn sub_p(&self, other: &Self) -> Self {
        Angle::Radians(self.to_radians() - other.to_radians())
    }

    fn mul_p(&self, other: &Self) -> Self {
        Angle::Radians(self.to_radians() * other.to_radians())
    }

    fn div_p(&self, other: &Self) -> Self {
        Angle::Radians(self.to_radians() / other.to_radians())
    }

    fn neg_p(&self) -> Self {
        Angle::Radians(-self.to_radians())
    }

    pub fn scale(&self, s: f64) -> Self {
        Angle::Radians(self.to_radians() * s)
    }

    pub fn round(&self) -> Self {
        Angle::Radians(self.to_radians().round())
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_p(&other)
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, other: Self) {
        *self = Angle::Radians(self.to_radians() + other.to_radians());
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.sub_p(&other)
    }
}

impl Mul for Angle {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_p(&other)
    }
}

impl Div for Angle {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.div_p(&other)
    }
}

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.neg_p()
    }
}

impl Default for Angle {
    /// Default method for Prism.
    ///
    /// Returns 0.0 radians
    fn default() -> Self {
        Angle::Radians(0.0)
    }
}

/// Converts a cartesian coordinate to polar
pub fn cart2pol(point: &Point2) -> PolarPoint {
    let rho = point.magnitude();
    let phi = point.y.atan2(point.x);
    PolarPoint::new(rho, phi)
}

/// Converts a polar coordinate to cartesian
pub fn pol2cart(point: &PolarPoint) -> Point2 {
    let x = point.rho * point.phi.cos();
    let y = point.rho * point.phi.sin();
    Point2 { x, y }
}

/// Converts a polar vector to a cartesian vector
pub fn vector_pol2cart(vector: &PolarPoint, phi: &f64) -> Point2 {
    let cos_phi = phi.cos();
    let sin_phi = phi.sin();

    let vector_x = vector.rho * cos_phi - vector.phi * sin_phi;
    let vector_y = vector.rho * sin_phi + vector.phi * cos_phi;

    Point2 {
        x: vector_x,
        y: vector_y,
    }
}

/// Rotates a 2D point, `Point2` about a pivot point
pub fn rotate_around_pivot_2d(&point: &Point2, phi: &f64, pivot: &Point2) -> Point2 {
    let cos_val = phi.cos();
    let sin_val = phi.sin();
    let x = point.x - pivot.x;
    let y = point.y - pivot.y;

    let x_rot = (x * cos_val - y * sin_val) + pivot.x;
    let y_rot = (x * sin_val + y * cos_val) + pivot.y;

    Point2 { x: x_rot, y: y_rot }
}

/// Rotates a 2D point, `Point2` about the origin
pub fn rotate_around_origin_2d(&point: &Point2, phi: &f64) -> Point2 {
    let cos_val = phi.cos();
    let sin_val = phi.sin();
    let x = point.x;
    let y = point.y;

    let x_rot = x * cos_val - y * sin_val;
    let y_rot = x * sin_val + y * cos_val;

    Point2 { x: x_rot, y: y_rot }
}

/// Converts a cartesian coordinate to spherical
pub fn cart2sph(point: &Point3) -> SphericalPoint {
    let rho = point.magnitude();
    let phi = point.y.atan2(point.x);
    let theta = (point.z / rho).acos();
    SphericalPoint::new(rho, phi, theta)
}

/// Converts a polar coordinate to cartesian
pub fn sph2cart(point: &SphericalPoint) -> Point3 {
    let x = point.rho * point.phi.cos() * point.theta.sin();
    let y = point.rho * point.phi.sin() * point.theta.sin();
    let z = point.rho * point.theta.cos();
    Point3 { x, y, z }
}

/// Converts a spherical vector to a cartesian vector
pub fn vector_sph2cart(vector: &SphericalPoint, phi: &f64, theta: &f64) -> Point3 {
    let cos_phi = phi.cos();
    let sin_phi = phi.sin();
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();

    let vector_x = vector.rho * cos_theta * cos_phi + vector.theta * cos_theta * cos_phi
        - vector.phi * sin_phi;

    let vector_y = vector.rho * sin_theta * sin_phi
        + vector.theta * cos_theta * sin_phi
        + vector.phi * cos_phi;

    let vector_z = vector.rho * cos_theta - vector.theta * sin_theta;

    Point3 {
        x: vector_x,
        y: vector_y,
        z: vector_z,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{utils::comparison::nearly_equal, PI, PI_2, PI_4};

    #[test]
    fn test_degrees_to_radians() {
        let angle = Angle::Degrees(90.0);
        assert!(nearly_equal(angle.to_radians(), PI_2));
    }

    #[test]
    fn test_degrees_to_degrees() {
        let angle = Angle::Degrees(32.0);
        assert!(nearly_equal(angle.to_degrees(), 32.0));
    }

    #[test]
    fn test_radians_to_degrees() {
        let angle = Angle::Radians(PI);
        assert!(nearly_equal(angle.to_degrees(), 180.0));
    }

    #[test]
    fn test_radians_to_radians() {
        let angle = Angle::Radians(PI_4);
        assert!(nearly_equal(angle.to_radians(), PI_4));
    }
}
