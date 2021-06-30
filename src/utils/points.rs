/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! Points
//! Generic traits for all points
//!

/// General traits for all point types, including operation overloading
pub trait Points {
    type Output;
    /// Implelment addition
    fn add_p(&self, other: &Self) -> Self::Output;
    /// Subtraction
    fn sub_p(&self, other: &Self) -> Self::Output;
    fn mul_p(&self, other: &Self) -> Self::Output;
    fn div_p(&self, other: &Self) -> Self::Output;
    fn neg_p(&self) -> Self::Output;
    fn scale(&self, s: f64) -> Self::Output;
    fn round(&self) -> Self::Output;
}
