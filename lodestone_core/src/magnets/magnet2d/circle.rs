/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

pub use super::circle_field::*;
use serde_derive::{Deserialize, Serialize};

use crate::utils::conversions::Angle;

use crate::magnets::{GetCenter, GetField, MagnetTrait};
use crate::points::{Point2, PolarPoint};
use crate::MagnetError;
use std::fmt;

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
/// use lodestone_core::magnets::Circle;
/// use lodestone_core::utils::conversions::Angle;
/// let magnet_1 = Circle::default();
/// println!("Magnet 1:{}", magnet_1);
/// let magnet_2 = Circle::new(1.0, (0.0, -1.0 / 2.0), Angle::Degrees(0.0), 1.0, Angle::Degrees(90.0));
/// println!("Magnet 2:{}", magnet_2);
/// ```
///
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Circle {
    pub radius: f64,
    pub center: Point2,
    pub alpha: Angle,
    pub jr: f64,
    pub phi: Angle,
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
            alpha: Angle::Degrees(0.0),
            jr: 1.0,
            phi: Angle::Degrees(90.0),
            jx: 0.0,
            jy: 1.0,
        }
    }
}

impl Circle {
    /// Constructor for 2D Circle magnet
    pub fn new<C>(radius: f64, center: C, alpha: Angle, jr: f64, phi: Angle) -> Circle
    where
        // R: Into<f64> + Mul<Output = R> + Add<Output = R> + Copy,
        C: GetCenter<Point2>,
        // J: Into<f64> + Mul<Output = J> + Add<Output = J> + Copy,
        // P: Into<f64> + Mul<Output = P> + Add<Output = P> + Copy,
    {
        let phi_rad = phi.to_radians();
        Circle {
            radius,
            center: center.center(),
            alpha,
            jr,
            phi,
            jx: jr * phi_rad.cos(),
            jy: jr * phi_rad.sin(),
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
            self.alpha.to_degrees(),
            self.jr,
            self.phi.to_degrees()
        )
    }
}

impl MagnetTrait<[f64; 2], Point2, f64, PolarPoint> for Circle {
    /// Returns the field due to a Circle
    // fn field(&self, point: &[f64; 2]) -> Result<[f64; 2], MagnetError> {
    //     // get_field_circle(&self, point)
    //     Ok(*point)
    // }

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
        PolarPoint::new(self.jr, self.phi.to_radians())
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
        self.phi = Angle::Radians(magnetisation.phi);
        self.jx = self.jr * (magnetisation.phi.cos());
        self.jy = self.jr * (magnetisation.phi.sin());
    }
}

impl GetField<&Point2, Result<Point2, MagnetError>> for Circle {
    /// Returns the magnetic field of a circular magnet at a Point2 struct {x,y}
    fn field(&self, point: &Point2) -> Result<Point2, MagnetError> {
        get_field_circle(self, point)
    }
}

impl GetField<&(f64, f64), Result<(f64, f64), MagnetError>> for Circle {
    /// Returns the magnetic field of a circular magnet at a 2-element tuple (x,y)
    fn field(&self, point: &(f64, f64)) -> Result<(f64, f64), MagnetError> {
        let field_vec = get_field_circle(
            self,
            &Point2 {
                x: point.0,
                y: point.1,
            },
        )?;
        Ok((field_vec.x, field_vec.y))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::comparison::nearly_equal;

    use super::*;

    //     #[test]
    //     fn test_circle_field() {
    //         let m1 = Circle::default();
    //         println!("m: {}", m1);
    //         let point1 = Point2::new(0.0, 0.5);
    //
    //         let field = m1.field(&point1).unwrap();
    //         // assert!(true);
    //     }

    #[test]
    fn test_circle_surface_field_y_rot_90() {
        let magnet = Circle::new(
            1.0,
            (0.0, 0.0),
            Angle::Degrees(90.0),
            1.0,
            Angle::Degrees(90.0),
        );
        let point1 = Point2::new(0.0, 1.0);

        let field = magnet.field(&point1).unwrap();
        let comp_field = Point2::new(0.5, 0.0);

        assert!(nearly_equal(field.x, comp_field.x));
        assert!(nearly_equal(field.y, comp_field.y));
    }
}
