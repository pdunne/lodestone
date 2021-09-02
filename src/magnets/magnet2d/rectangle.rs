/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

pub use super::rectangle_field::*;

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
// use std::ops::{Add, Mul};

use crate::magnets::{GetCenter, GetField, MagnetTrait};
use crate::points::{Point2, PolarPoint};
use crate::utils::conversions::Angle;
use crate::MagnetError;

/// A 2D rectangular magnet
///
/// Arguments using `new()` method:
///     - width (f64): magnet width
///     - height (f64): magnet height
///     - center (Point2): magnet center
///     - alpha (f64):
///     - jr (f64): remnant magnetisation in T
///     - phi (f64): angle of magnetisation w.r.t. x-axis
///
/// The default method generates a square magnet of size 1.0 x 1.0, centred at
/// (0.0, 0.0), magnetised in y with a remnant magnetisation of 1.0 T
/// # Example
/// ```rust
/// use magnet_rs::magnets::Rectangle;
/// use magnet_rs::utils::conversions::Angle;
/// let magnet_1 = Rectangle::default();
/// println!("Magnet 1:{}", magnet_1);
/// let magnet_2 = Rectangle::new(1.0, 1.0, (0.0, -1.0 / 2.0), Angle::Degrees(0.0), 1.0, Angle::Degrees(90.0));
/// println!("Magnet 2:{}", magnet_2);
/// ```
///
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub center: Point2,
    pub alpha: Angle,
    pub jr: f64,
    pub phi: Angle,
    pub a: f64,
    pub b: f64,
    pub jx: f64,
    pub jy: f64,
}

impl GetCenter<Point2> for Point2 {
    fn center(&self) -> Point2 {
        *self
    }
}

impl<T: Into<f64> + Copy, U: Into<f64> + Copy> GetCenter<Point2> for (T, U) {
    /// Converts a 2 element tuple to a Point2
    fn center(&self) -> Point2 {
        Point2 {
            x: self.0.into(),
            y: self.1.into(),
        }
    }
}

impl Default for Rectangle {
    /// Default method for Rectangle.
    ///
    /// Generates a rectangular magnet  of size 1.0 x 1.0, centred at (0,0),
    /// with a magnetisation of 1 tesla in y
    fn default() -> Self {
        Rectangle {
            width: 1.0,
            height: 1.0,
            center: Point2::new(0.0, 0.0),
            alpha: Angle::Degrees(0.0),
            jr: 1.0,
            phi: Angle::Degrees(90.0),
            jx: 0.0,
            jy: 1.0,
            a: 0.5,
            b: 0.5,
        }
    }
}

impl Rectangle {
    pub fn new<C>(
        width: f64,
        height: f64,
        center: C,
        alpha: Angle,
        jr: f64,
        phi: Angle,
    ) -> Rectangle
    where
        C: GetCenter<Point2>,
    {
        let phi_rad = phi.to_radians();

        Rectangle {
            width,
            height,
            center: center.center(),
            alpha,
            jr,
            phi,
            jx: jr * phi_rad.cos(),
            jy: jr * phi_rad.sin(),
            a: width / 2.0,
            b: height / 2.0,
        }
    }
}

/// Implements Display for Rectangle magnets.
///
/// Example:
///
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[w: {},\th: {},\tc: {},\talpha:{}\tJ ({:.3}, {:.3})]",
            self.width,
            self.height,
            self.center,
            self.alpha.to_degrees(),
            self.jr,
            self.phi.to_degrees()
        )
    }
}

impl MagnetTrait<[f64; 2], Point2, [f64; 2], PolarPoint> for Rectangle {
    /// Returns the field due to a Rectangle
    // fn field(&self, point: &[f64; 2]) -> anyhow::Result<[f64; 2], MagnetError> {
    //     // get_field_rectangle(&self, point, x)
    //     // Ok([0.0_f64; 2])
    //     Ok(*point)
    // }

    /// Returns the center of a Rectangle
    fn center(&self) -> Point2 {
        self.center
    }

    /// Returns the size of a Rectangle: Point2 (x:width, y:height)
    fn size(&self) -> [f64; 2] {
        [self.width, self.height]
    }

    /// Returns the magnetisation of a Rectangle: PolarPoint (rho:Jr, phi:angle)
    fn magnetisation(self) -> PolarPoint {
        PolarPoint::new(self.jr, self.phi.to_radians())
    }

    /// Sets the magnet center
    fn set_center(&mut self, point: Point2) {
        self.center = point;
    }

    /// Sets the width and height of a rectangular magnet, and the internal
    /// values self.a and self.b
    fn set_size(&mut self, point: [f64; 2]) {
        self.width = point[0];
        self.height = point[1];
        self.a = self.width / 2.0;
        self.b = self.height / 2.0;
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

impl GetField<&Point2, Result<Point2, MagnetError>> for Rectangle {
    /// Returns the magnetic field of a rectangular magnet at a Point2 struct {x,y}
    fn field(&self, point: &Point2) -> Result<Point2, MagnetError> {
        get_field_rectangle(self, point)
    }
}

impl GetField<&(f64, f64), Result<(f64, f64), MagnetError>> for Rectangle {
    /// Returns the magnetic field of a rectangular magnet at a 2-element tuple (x,y)
    fn field(&self, point: &(f64, f64)) -> Result<(f64, f64), MagnetError> {
        let field_vec = get_field_rectangle(
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

    use super::{Angle, GetField, Point2, Rectangle};

    #[test]
    fn test_tuple_rectangle_field() {
        let magnet = Rectangle::new(
            1.0,
            1.0,
            Point2::new(0., 0.0),
            Angle::Degrees(0.0),
            1.0,
            Angle::Degrees(90.0),
        );

        let point = (0.0, 0.0);
        let field = magnet.field(&point).unwrap();
        assert_eq!(field.0, 0.0_f64);
        assert_eq!(field.1, 0.5);
    }

    #[test]
    fn test_point2_rectangle_field() {
        let magnet = Rectangle::new(
            1.0,
            1.0,
            Point2::new(0., 0.0),
            Angle::Degrees(0.0),
            1.0,
            Angle::Degrees(90.0),
        );

        let point = Point2::new(0.0, 0.0);
        let field = magnet.field(&point).unwrap();
        assert_eq!(field.x, 0.0_f64);
        assert_eq!(field.y, 0.5_f64);
    }

    #[test]
    fn test_point2_rectangle_field_45() {
        let magnet = Rectangle::new(
            1.0,
            1.0,
            Point2::new(0., 0.0),
            Angle::Degrees(0.0),
            1.0,
            Angle::Degrees(45.0),
        );

        let point = Point2::new(0.0, 0.0);
        let field = magnet.field(&point).unwrap();
        assert!(nearly_equal(field.x, 0.5 / 2.0_f64.sqrt()));
        assert!(nearly_equal(field.y, 0.5 / 2.0_f64.sqrt()));
    }
}
