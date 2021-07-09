/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! # Points
//! Generic traits for all points
//!

mod point_array2;
mod points2;
mod points3;
mod polarpoints;

// make subroutines available from this module
pub use point_array2::*;
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
