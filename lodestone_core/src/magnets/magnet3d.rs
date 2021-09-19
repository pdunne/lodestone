/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! # Magnet3D - Field calculations in 3D
/// This consists of modules for calculating magnetic fields due to
///
use serde_derive::{Deserialize, Serialize};

use super::MagnetTrait;

pub mod bulirsch;
mod prism;
mod solenoid;

pub use prism::*;
pub use solenoid::*;

/// Enum to store the different 3D magnet types.
///
/// This allows us to create a Vec<MagnetType2D>, and access the fields of the magnet
/// struct with a match routine.
///
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Magnet3D {
    Prism(Prism),
    Cylinder,
    Sphere,
    Custom,
}

/// Magnet2D Traits
pub trait MagnetTrait3D<POINT, CENTER, SIZE, MAG>: MagnetTrait<POINT, CENTER, SIZE, MAG> {}
