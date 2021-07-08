/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! # Magnet2D - Field calculations in 2D
//! This consists of modules for calculating magnetic fields due to magnetic
//! objects in 2D, including:
//!
pub mod circle_field;
pub mod line_field;
pub mod rectangle_field;

use std::error::Error;
use std::fmt;
use std::ops::{Add, Mul};

use crate::magnets::magnet2d::circle_field::get_field_circle;
use crate::magnets::magnet2d::rectangle_field::get_field_rectangle;
use crate::magnets::Magnet;
use crate::utils::points2::{Point2, PolarPoint};
use crate::PI;

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
/// use magnet_rs::magnets::magnet2d::Rectangle;
/// let magnet_1 = Rectangle::default();
/// println!("Magnet 1:{}", magnet_1);
/// let magnet_2 = Rectangle::new(1.0, 1.0, (0.0, -1.0 / 2.0), 0.0, 1.0, 90);
/// println!("Magnet 2:{}", magnet_2);
/// ```
///
#[derive(Copy, Clone)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub center: Point2,
    pub alpha: f64,
    pub jr: f64,
    pub phi: f64,
    pub a: f64,
    pub b: f64,
    pub jx: f64,
    pub jy: f64,
}

pub trait GetCenter2D {
    fn center(&self) -> Point2;
}

impl GetCenter2D for Point2 {
    fn center(&self) -> Point2 {
        *self
    }
}

impl<T: Into<f64> + Copy, U: Into<f64> + Copy> GetCenter2D for (T, U) {
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
            alpha: 0.0,
            jr: 1.0,
            phi: 90.0,
            jx: 0.0,
            jy: 1.0,
            a: 0.5,
            b: 0.5,
        }
    }
}

impl Rectangle {
    pub fn new<W, H, A, J, P, C>(
        width: W,
        height: H,
        center: C,
        alpha: A,
        jr: J,
        phi: P,
    ) -> Rectangle
    where
        W: Into<f64> + Mul<Output = W> + Add<Output = W> + Copy,
        H: Into<f64> + Mul<Output = H> + Add<Output = H> + Copy,
        C: GetCenter2D,
        A: Into<f64> + Mul<Output = A> + Add<Output = A> + Copy,
        J: Into<f64> + Mul<Output = J> + Add<Output = J> + Copy,
        P: Into<f64> + Mul<Output = P> + Add<Output = P> + Copy,
    {
        Rectangle {
            width: width.into(),
            height: height.into(),
            center: center.center(),
            alpha: alpha.into(),
            jr: jr.into(),
            phi: phi.into(),
            jx: jr.into() * (phi.into() * PI / 180.).cos(),
            jy: jr.into() * (phi.into() * PI / 180.).sin(),
            a: width.into() / 2.0,
            b: height.into() / 2.0,
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
            self.width, self.height, self.center, self.alpha, self.jx, self.jy
        )
    }
}

/// Magnet2D Traits
pub trait Magnet2D<T, U>: Magnet {
    /// Returns the magnetic field at a point
    fn field(&self, point: &T) -> Result<T, Box<dyn Error>>;

    /// Returns the magnet center
    fn center(&self) -> Point2;

    /// Returns the magnet dimensions.
    ///
    /// Note: This returns a generic, an array [f64;2] for Rectangles,
    /// and f64 for Circles
    fn size(&self) -> U;

    /// Returns the magnetisation vector
    fn magnetisation(self) -> Point2;

    /// Sets the magnet center to a point
    fn set_center(&mut self, point: Point2);

    /// Sets the size the of the magnet.
    /// Generic method which can also change internal struct values
    fn set_size(&mut self, point: U);

    /// Set the magnetisation  of the magnet using a Polar vector.
    /// i.e. magnitude and angle phi.
    ///
    /// This method also updates self.jx and self.jy
    fn set_magnetisation(&mut self, magnetisation: PolarPoint);
}

impl Magnet for Rectangle {}

impl Magnet2D<Point2, [f64; 2]> for Rectangle {
    /// Returns the field due to a Rectangle
    fn field(&self, point: &Point2) -> Result<Point2, Box<dyn Error>> {
        get_field_rectangle(&self, point)
    }

    /// Returns the center of a Rectangle
    fn center(&self) -> Point2 {
        self.center
    }

    /// Returns the size of a Rectangle: Point2 (x:width, y:height)
    fn size(&self) -> [f64; 2] {
        [self.width, self.height]
    }

    /// Returns the magnetisation of a Rectangle: Point2 (x:Jx, y:Jy)
    fn magnetisation(self) -> Point2 {
        Point2::new(self.jx, self.jy)
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
        self.phi = magnetisation.phi;
        self.jx = self.jr * (self.phi * PI / 180.).cos();
        self.jy = self.jr * (self.phi * PI / 180.).sin();
    }
}

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
/// use magnet_rs::magnets::magnet2d::Circle;
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
        C: GetCenter2D,
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
            jx: jr.into() * (phi.into() * PI / 180.).cos(),
            jy: jr.into() * (phi.into() * PI / 180.).sin(),
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
            self.radius, self.center, self.alpha, self.jx, self.jy
        )
    }
}

impl Magnet for Circle {}

impl Magnet2D<Point2, f64> for Circle {
    /// Returns the field due to a Circle
    fn field(&self, point: &Point2) -> Result<Point2, Box<dyn Error>> {
        get_field_circle(&self, point)
    }

    /// Returns the center of a Circle
    fn center(&self) -> Point2 {
        self.center
    }

    /// Returns the radius of a Circle
    fn size(&self) -> f64 {
        self.radius
    }

    /// Returns the magnetisation of a Circle: Point2 (x:Jx, y:Jy)
    fn magnetisation(self) -> Point2 {
        Point2::new(self.jx, self.jy)
    }

    /// Sets the magnet center
    fn set_center(&mut self, point: Point2) {
        self.center = point;
    }

    /// Sets the magnet radius
    fn set_size(&mut self, point: f64) {
        self.radius = point;
    }

    /// Set the magnetisation  of the magnet using a Polar vector.
    /// i.e. magnitude and angle phi.
    ///
    /// This method also updates self.jx and self.jy
    fn set_magnetisation(&mut self, magnetisation: PolarPoint) {
        self.jr = magnetisation.rho;
        self.phi = magnetisation.phi;
        self.jx = self.jr * (self.phi * PI / 180.).cos();
        self.jy = self.jr * (self.phi * PI / 180.).sin();
    }
}
