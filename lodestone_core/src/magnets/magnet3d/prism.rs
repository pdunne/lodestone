/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

// pub use super::rectangle_field::*;

// use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
// use std::fmt;
// use std::ops::{Add, Mul};

// use crate::magnets::{GetCenter, GetField, MagnetTrait};
use crate::points::Point3;
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
/// The default method generates a square magnet of size 1.0 x 1.0, x 1.0 centred at
/// (0.0, 0.0, 0.0), magnetised in z with a remnant magnetisation of 1.0 T
/// # Example
/// ```rust
/// use magnet_rs::magnets::Prism;
/// use magnet_rs::utils::conversions::Angle;
/// let magnet_1 = Rectangle::default();
/// println!("Magnet 1:{}", magnet_1);
/// let magnet_2 = Rectangle::new(1.0, 1.0, (0.0, -1.0 / 2.0), Angle::Degrees(0.0), 1.0, Angle::Degrees(90.0));
/// println!("Magnet 2:{}", magnet_2);
/// ```
///
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Prism {
    pub width: f64,
    pub height: f64,
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
