/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

use super::line_field::sheet_field;
use crate::{
    points::{Point2, Points, Points2},
    MagnetError,
};

#[derive(Clone, Debug)]
pub struct Line {
    pub length: f64,
    pub center: Point2,
    pub beta: f64,
    pub k: f64,
}

impl Default for Line {
    /// Default method for Line.
    ///
    /// Generates a line magnet  of length 1.0, centred at (0,0),
    /// with a surface current density equivalent to 1 tesla in z
    fn default() -> Self {
        Line {
            length: 1.0,
            center: Point2::default(),
            beta: 0.0,
            k: 1.0,
        }
    }
}

pub fn signed_area() {
    println!("");
}

fn unit_norm_vector(vertex_1: Point2, vertex_2: Point2) -> (Point2, f64) {
    let delta = vertex_1 - vertex_2;
    let norm = Point2::new(delta.y, -delta.x);
    (norm.unit(), norm.magnitude())
}

/// Returns the line midpoint
fn line_center(vertex_1: &Point2, vertex_2: &Point2) -> Point2 {
    (*vertex_1 + *vertex_2).scale(0.5)
}

/// Returns angle of line with respect to the y-axis
fn get_line_beta(unit_norm: &Point2) -> f64 {
    unit_norm.y.atan2(unit_norm.x)
}

fn get_field_line(center: &Point2, length: &f64, kr: &f64) -> Result<Point2, MagnetError> {
    sheet_field(&center.x, &center.y, length, kr)
}
