/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! Generic traits for all point objects i.e. Points (2D, 3D, cartesian, polar,
//! cylindrical, spherical), structs of
//! heap allocated vectors, stack allocated arrays
//!
//! Note that struct of arrays is implemented but unused
//!

// mod point_array2;
mod point_vec2;
mod points2;
mod points3;
mod polarpoints;
mod rotation_2d;

// make subroutines available from this module
// pub use point_array2::*;
pub use point_vec2::*;
pub use points2::*;
pub use points3::*;
pub use polarpoints::*;

/// General traits for all point types, including operation overloading
pub trait Points {
    /// Output type, usually set to Self
    type Output;
    /// Implelments addition
    fn add_p(&self, other: &Self) -> Self::Output;
    /// Implelments subtraction
    fn sub_p(&self, other: &Self) -> Self::Output;
    /// Implelments multiplication
    fn mul_p(&self, other: &Self) -> Self::Output;
    /// Implelments division
    fn div_p(&self, other: &Self) -> Self::Output;
    /// Implelments negation
    fn neg_p(&self) -> Self::Output;
    /// Implelments scaling by a float
    fn scale(&self, s: f64) -> Self::Output;
    /// Implelments round of all internal elements
    fn round(&self) -> Self::Output;
}

/// Calculates the norm of an x,y pair
pub fn internal_norm(x: &f64, y: &f64) -> (f64, f64) {
    let xy_mag = (x.powi(2) + y.powi(2)).sqrt();
    (x / xy_mag, y / xy_mag)
}
