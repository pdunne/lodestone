/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! # Magnet2D - Field calculations in 2D
//! This consists of modules for calculating magnetic fields due to magnetic
//! objects in 2D, including:
//!

mod circle;
mod rectangle;

mod circle_field;
mod rectangle_field;

pub use circle::*;
pub use rectangle::*;

use crate::magnets::Magnet;

/// Magnet2D Traits
pub trait Magnet2D<POINT, CENTER, SIZE, MAG>: Magnet<POINT, CENTER, SIZE, MAG> {}
