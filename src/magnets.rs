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

mod magnet2d;
mod magnet3d;

pub use magnet2d::{Circle, Magnet2D, Rectangle};

use crate::MagnetError;

/// Return center trait. It must implement the `center()` method
pub trait GetCenter<T> {
    /// Returns center method for any type
    fn center(&self) -> T;
}

/// Magnet Trait for standard methods for all magnet types
pub trait Magnet<POINT, CENTER, SIZE, MAG> {
    /// Returns the magnetic field at a point
    fn field(&self, point: &POINT) -> anyhow::Result<POINT, MagnetError>;

    /// Returns the magnet center
    fn center(&self) -> CENTER;

    /// Returns the magnet dimensions.
    ///
    /// Note: This returns a generic, an array `[f64;2]` for Rectangles,
    /// and f64 for Circles
    fn size(&self) -> SIZE;

    /// Returns the magnetisation vector
    fn magnetisation(self) -> MAG;

    /// Sets the magnet center to a point
    fn set_center(&mut self, point: CENTER);

    /// Sets the size the of the magnet.
    /// Generic method which can also change internal struct values
    fn set_size(&mut self, point: SIZE);

    /// Set the magnetisation  of the magnet using a Polar vector.
    /// i.e. magnitude and angle phi.
    ///
    /// This method also updates self.jx and self.jy
    fn set_magnetisation(&mut self, magnetisation: MAG);
}
