/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

pub use super::circle_field::*;

use std::fmt;
use std::ops::{Add, Mul};

use crate::magnets::{GetCenter, Magnet};
use crate::points::{Point2, PolarPoint};
use crate::MagnetError;

/// A 2D circular magnet
///
/// Arguments using `new()` method:
///     - radius (f64): magnet radius
///     - center (Point2): magnet center
///     - alpha (f64): NOT IMPLEMENTED
///     - jr (f64): remnant magnetisation in T
///     - phi (f64): angle of magnetisation w.r.t. x-axis
///
/// The default method generates a circular magnet of radius 1.0, centred at
/// (0.0, 0.0), magnetised in y with a remnant magnetisation of 1.0 T
/// # Example
/// ```rust
/// use magnet_rs::magnets::Circle;
/// let magnet_1 = Circle::default();
/// println!("Magnet 1:{}", magnet_1);
/// let magnet_2 = Circle::new(1.0, (0.0, -1.0 / 2.0), 0.0, 1.0, 45);
/// println!("Magnet 2:{}", magnet_2);
/// ```
///
#[derive(Copy, Clone)]
pub struct Circle {
    pub radius: f64,
    pub center: Point2,
    pub alpha: f64,
    pub jr: f64,
    pub phi: f64,
    pub jx: f64,
    pub jy: f64,
}

impl Default for Circle {
    /// Default method for Circle.
    ///
    /// Generates a circular magnet  of radius 1.0, centred at (0,0),
    /// with a magnetisation of 1 tesla in y
    fn default() -> Self {
        Circle {
            radius: 1.0,
            center: Point2::new(0.0, 0.0),
            alpha: 0.0,
            jr: 1.0,
            phi: 90.0,
            jx: 0.0,
            jy: 1.0,
        }
    }
}

impl Circle {
    pub fn new<R, C, A, J, P>(radius: R, center: C, alpha: A, jr: J, phi: P) -> Circle
    where
        R: Into<f64> + Mul<Output = R> + Add<Output = R> + Copy,
        C: GetCenter<Point2>,
        A: Into<f64> + Mul<Output = A> + Add<Output = A> + Copy,
        J: Into<f64> + Mul<Output = J> + Add<Output = J> + Copy,
        P: Into<f64> + Mul<Output = P> + Add<Output = P> + Copy,
    {
        Circle {
            radius: radius.into(),
            center: center.center(),
            alpha: alpha.into(),
            jr: jr.into(),
            phi: phi.into(),
            jx: jr.into() * phi.into().cos(),
            jy: jr.into() * phi.into().sin(),
        }
    }
}

/// Implements Display for Rectangle magnets.
///
/// Example:
///
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[r: {}\tc: {},\talpha:{}\tJ ({:.3}, {:.3})]",
            self.radius,
            self.center,
            self.alpha,
            self.jr,
            self.phi.to_degrees()
        )
    }
}

// impl Magnet for Circle {}
impl Magnet<[f64; 2], Point2, f64, PolarPoint> for Circle {
    /// Returns the field due to a Circle
    fn field(&self, point: &[f64; 2]) -> Result<[f64; 2], MagnetError> {
        // get_field_circle(&self, point)
        Ok(*point)
    }

    /// Returns the center of a Circle
    fn center(&self) -> Point2 {
        self.center
    }

    /// Returns the radius of a Circle
    fn size(&self) -> f64 {
        self.radius
    }

    /// Returns the magnetisation of a Circle: PolarPoint (Jr, phi)
    fn magnetisation(self) -> PolarPoint {
        PolarPoint::new(self.jr, self.phi)
    }

    /// Sets the magnet center
    fn set_center(&mut self, point: Point2) {
        self.center = point;
    }

    /// Sets the magnet radius
    fn set_size(&mut self, radius: f64) {
        self.radius = radius;
    }

    /// Set the magnetisation  of the magnet using a Polar vector.
    /// i.e. magnitude and angle phi.
    ///
    /// This method also updates self.jx and self.jy
    fn set_magnetisation(&mut self, magnetisation: PolarPoint) {
        self.jr = magnetisation.rho;
        self.phi = magnetisation.phi;
        self.jx = self.jr * (self.phi).cos();
        self.jy = self.jr * (self.phi).sin();
    }
}
