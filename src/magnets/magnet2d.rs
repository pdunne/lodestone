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
use crate::utils::points2::Point2;
// use crate::utils::points2::PolarPoint;
use crate::PI;

/// A 2D magnet
///
/// Arguments:
///
/// ::default method generates a square magnet of size 1 x 1, centred at
/// (0, 0), magnetised in y with a remnant magnetisation of 1 T
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
    pub theta: f64,
    pub a: f64,
    pub b: f64,
    pub jx: f64,
    pub jy: f64,
}

pub trait GetCenter2D {
    fn get_center(&self) -> Point2;
}

impl GetCenter2D for Point2 {
    fn get_center(&self) -> Point2 {
        *self
    }
}

impl<T: Into<f64> + Copy, U: Into<f64> + Copy> GetCenter2D for (T, U) {
    fn get_center(&self) -> Point2 {
        Point2 {
            x: self.0.into(),
            y: self.1.into(),
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            width: 1.0,
            height: 1.0,
            center: Point2::new(0.0, 0.0),
            alpha: 0.0,
            jr: 1.0,
            theta: 0.0,
            jx: 0.0,
            jy: 1.0,
            a: 0.5,
            b: 0.5,
        }
    }
}

impl Rectangle {
    pub fn new<W, H, A, J, T, C>(
        width: W,
        height: H,
        center: C,
        alpha: A,
        jr: J,
        theta: T,
    ) -> Rectangle
    where
        W: Into<f64> + Mul<Output = W> + Add<Output = W> + Copy,
        H: Into<f64> + Mul<Output = H> + Add<Output = H> + Copy,
        C: GetCenter2D,
        A: Into<f64> + Mul<Output = A> + Add<Output = A> + Copy,
        J: Into<f64> + Mul<Output = J> + Add<Output = J> + Copy,
        T: Into<f64> + Mul<Output = T> + Add<Output = T> + Copy,
    {
        Rectangle {
            width: width.into(),
            height: height.into(),
            center: center.get_center(),
            alpha: alpha.into(),
            jr: jr.into(),
            theta: theta.into(),
            jx: jr.into() * (theta.into() * PI / 180.).cos(),
            jy: jr.into() * (theta.into() * PI / 180.).sin(),
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

pub trait Magnet2D<T>: Magnet {
    fn get_field(&self, point: &T) -> Result<T, Box<dyn Error>>;
    fn get_center(&self) -> Point2;
    fn get_size(&self) -> Point2;
    fn get_j(self) -> Point2;
}

impl Magnet for Rectangle {}

impl Magnet2D<Point2> for Rectangle {
    fn get_field(&self, point: &Point2) -> Result<Point2, Box<dyn Error>> {
        get_field_rectangle(&self, point)
    }

    fn get_center(&self) -> Point2 {
        self.center
    }

    fn get_size(&self) -> Point2 {
        Point2::new(self.width, self.height)
    }

    fn get_j(self) -> Point2 {
        Point2::new(self.jx, self.jy)
    }
}

#[derive(Copy, Clone)]
pub struct Circle {
    pub radius: f64,
    pub center: Point2,
    pub alpha: f64,
    pub jr: f64,
    pub theta: f64,
    pub jx: f64,
    pub jy: f64,
}

impl Default for Circle {
    fn default() -> Self {
        Circle {
            radius: 1.0,
            center: Point2::new(0.0, 0.0),
            alpha: 0.0,
            jr: 1.0,
            theta: 0.0,
            jx: 0.0,
            jy: 1.0,
        }
    }
}

impl Circle {
    pub fn new<R, C, A, J, T>(radius: R, center: C, alpha: A, jr: J, theta: T) -> Circle
    where
        R: Into<f64> + Mul<Output = R> + Add<Output = R> + Copy,
        C: GetCenter2D,
        A: Into<f64> + Mul<Output = A> + Add<Output = A> + Copy,
        J: Into<f64> + Mul<Output = J> + Add<Output = J> + Copy,
        T: Into<f64> + Mul<Output = T> + Add<Output = T> + Copy,
    {
        Circle {
            radius: radius.into(),
            center: center.get_center(),
            alpha: alpha.into(),
            jr: jr.into(),
            theta: theta.into(),
            jx: jr.into() * (theta.into() * PI / 180.).cos(),
            jy: jr.into() * (theta.into() * PI / 180.).sin(),
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

impl Magnet2D<Point2> for Circle {
    fn get_field(&self, point: &Point2) -> Result<Point2, Box<dyn Error>> {
        get_field_circle(&self, point)
    }

    fn get_center(&self) -> Point2 {
        self.center
    }

    fn get_size(&self) -> Point2 {
        Point2::new(self.radius, self.radius)
    }

    fn get_j(self) -> Point2 {
        Point2::new(self.jx, self.jy)
    }
}
