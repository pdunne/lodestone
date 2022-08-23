/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

// pub use super::rectangle_field::*;

// use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
// use std::ops::{Add, Mul};

use crate::magnets::{GetCenter, MagnetTrait};
use crate::points::{Point3, Points3, SphericalPoint};
use crate::utils::conversions::Angle;
// use crate::MagnetError;

//TODO: Finish docs
/// A 3D Prismatic magnet
///
/// Arguments using `new()` method:
///     - width (f64): magnet width
///     - height (f64): magnet height
///     - depth (f64): magnet depth
///     - center (Point3): magnet center
///     - alpha (f64):
///     - beta (f64):
///     - gamma (f64):
///     - jr (f64): remnant magnetisation in T
///     - phi (f64): angle of magnetisation w.r.t. x-axis
///     - theta (f64): angle of magnetisation w.r.t. z-axis
///
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Prism {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub center: Point3,
    pub alpha: Angle,
    pub beta: Angle,
    pub gamma: Angle,

    pub jr: f64,
    pub phi: Angle,
    pub theta: Angle,

    pub a: f64,
    pub b: f64,
    pub c: f64,

    pub jx: f64,
    pub jy: f64,
    pub jz: f64,
}

impl GetCenter<Point3> for Point3 {
    fn center(&self) -> Point3 {
        *self
    }
}

impl<T: Into<f64> + Copy, U: Into<f64> + Copy, V: Into<f64> + Copy> GetCenter<Point3>
    for (T, U, V)
{
    /// Converts a 3 element tuple to a Point3
    fn center(&self) -> Point3 {
        Point3 {
            x: self.0.into(),
            y: self.1.into(),
            z: self.2.into(),
        }
    }
}

impl Default for Prism {
    /// Default method for Prism.
    ///
    /// Generates a rectangular magnet  of size 1.0 x 1.0 x 1.0, centred at (0,0, 0),
    /// with a magnetisation of 1 tesla in z
    fn default() -> Self {
        Prism {
            width: 1.0,
            height: 1.0,
            depth: 1.0,
            center: Point3::zero(),
            alpha: Angle::Degrees(0.0),
            beta: Angle::Degrees(0.0),
            gamma: Angle::Degrees(0.0),

            jr: 1.0,
            phi: Angle::Degrees(90.0),
            theta: Angle::Degrees(0.0),

            a: 0.5,
            b: 0.5,
            c: 0.5,

            jx: 0.0,
            jy: 0.0,
            jz: 1.0,
        }
    }
}

impl Prism {
    /// Constructor for 3D Prism magnet
    pub fn new<C>(
        size: [f64; 3],
        center: C,
        orientation: [Angle; 3],
        magnetisation: SphericalPoint,
    ) -> Prism
    where
        C: GetCenter<Point3>,
    {
        let phi_rad = magnetisation.phi;
        let theta_rad = magnetisation.theta;
        let jr = magnetisation.rho;

        Prism {
            width: size[0],
            height: size[1],
            depth: size[2],
            center: center.center(),
            alpha: orientation[0],
            beta: orientation[1],
            gamma: orientation[2],
            jr,
            phi: Angle::Radians(magnetisation.phi),
            theta: Angle::Radians(magnetisation.theta),
            jx: jr * phi_rad.cos() * theta_rad.sin(),
            jy: jr * phi_rad.sin() * theta_rad.sin(),
            jz: jr * theta_rad.cos(),
            a: size[0] / 2.0,
            b: size[1] / 2.0,
            c: size[2] / 2.0,
        }
    }
}

/// Implements Display for Prism magnets.
///
/// Example:
///
impl fmt::Display for Prism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[w: {},\th: {},\td: {},\tc: {}][\nalpha:{}, beta:{}, gamma:{}\tJ ({:.3}, {:.3}, {:.3})]",
            self.width,
            self.height,
            self.depth,
            self.center,
            self.alpha.to_degrees(),
            self.beta.to_degrees(),
            self.gamma.to_degrees(),
            self.jr,
            self.phi.to_degrees(),
            self.theta.to_degrees()
        )
    }
}

impl MagnetTrait<[f64; 3], Point3, [f64; 3], SphericalPoint> for Prism {
    /// Returns the field due to a Prism
    // fn field(&self, point: &[f64; 3]) -> anyhow::Result<[f64; 3], MagnetError> {
    //     // get_field_rectangle(&self, point, x)
    //     // Ok([0.0_f64; 3])
    //     Ok(*point)
    // }

    /// Returns the center of a Prism
    fn center(&self) -> Point3 {
        self.center
    }

    /// Returns the size of a Prism: Point2 (x:width, y:height, z: depth)
    fn size(&self) -> [f64; 3] {
        [self.width, self.height, self.depth]
    }

    /// Returns the magnetisation of a Rectangle: SphericalPoint (rho:Jr, phi:angle, theta: angle)
    fn magnetisation(self) -> SphericalPoint {
        SphericalPoint::new(self.jr, self.phi.to_radians(), self.theta.to_radians())
    }

    // /// Returns the magnetisation of a Rectangle: SphericalPoint (rho:Jr, phi:angle)
    // fn magnetisation(self) -> Point3 {
    //     magnetisation_sph(self).to_cartesian()
    // }

    /// Sets the magnet center
    fn set_center(&mut self, point: Point3) {
        self.center = point;
    }

    /// Sets the width, height, and depth of a prismatic magnet, and the internal
    /// values self.a, self.b, and self.c
    fn set_size(&mut self, point: [f64; 3]) {
        self.width = point[0];
        self.height = point[1];
        self.depth = point[2];
        self.a = self.width / 2.0;
        self.b = self.height / 2.0;
        self.c = self.depth / 2.0;
    }

    /// Set the magnetisation  of the magnet using a Spherical vector.
    /// i.e. magnitude, angles phi and theta.
    ///
    /// This method also updates self.jx, self.jy, self.jz
    fn with_magnetisation(mut self, magnetisation: SphericalPoint) -> Self {
        self.jr = magnetisation.rho;
        self.phi = Angle::Radians(magnetisation.phi);
        self.theta = Angle::Radians(magnetisation.theta);
        let cos_phi = magnetisation.phi.cos();
        let sin_phi = magnetisation.phi.sin();
        let cos_theta = magnetisation.theta.cos();
        let sin_theta = magnetisation.theta.sin();

        self.jx = self.jr * cos_phi * sin_theta;
        self.jy = self.jr * sin_phi * sin_theta;
        self.jz = self.jr * cos_theta;
        self
    }
}
