/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

//! The magnets module contains the 2D and 3D magnet objects
//! and their methods for calculating magnetic fields
//!
//! # Magnet Structs
//! List the different struct types here!
//! - List 1
//! - List
//!
//! # Methods

mod base;
mod magnet2d;
mod magnet3d;
pub use base::*;

pub use magnet2d::{
    get_field_2d, loop_field_2d, sheet_field, Circle, Magnet2D, MagnetTrait2D, PolyDimension,
    Polygon, Rectangle, Vertices,
};

pub use magnet3d::bulirsch;
// use crate::MagnetError;
