/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! # Magnet2D - Field calculations in 2D
//! This consists of modules for calculating magnetic fields due to magnetic
//! objects in 2D, including:
//!

mod circle;
mod field_loop_2d;
mod line;
mod line_field;
mod polygon;
mod rectangle;

mod circle_field;
mod rectangle_field;

pub use circle::*;
pub use field_loop_2d::*;
pub use line::*;
pub use line_field::*;
pub use polygon::*;
pub use rectangle::*;

use crate::MagnetError;
use crate::config::magnet2d_to_toml;
use crate::config::MagnetKind;
use crate::magnets::MagnetTrait;

/// Magnet2D Traits
pub trait MagnetTrait2D<POINT, CENTER, SIZE, MAG>: MagnetTrait<POINT, CENTER, SIZE, MAG> {}

use serde_derive::{Deserialize, Serialize};

/// Enum to store the different 2D magnet types.
///
/// This allows us to create a Vec<MagnetType2D>, and access the fields of the magnet
/// struct with a match routine.
///
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Magnet2D {
    Rectangle(Rectangle),
    Circle(Circle),
}

impl Magnet2D {
    /// Returns serialisable struct for saving to a TOML file
    pub fn to_toml_struct(&self) -> Result<MagnetKind, MagnetError> {
        magnet2d_to_toml(self)
    }
}
