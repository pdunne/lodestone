/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

#![allow(unused_imports)]

use super::line_field::sheet_field;
use super::polygon::{return_vert_num_vert, PolyDimension, Vertices};
use crate::{
    points::{Point2, PointVec2, Points, Points2},
    utils::conversions::Angle,
    MagnetError,
};

#[derive(Clone, Debug)]
pub struct Line {
    pub length: f64,
    pub center: Point2,
    pub beta: Angle,
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
            beta: Angle::Radians(0.0),
            k: 1.0,
        }
    }
}

/// Returns the signed area of a polygon using the Shoelace Formula
pub fn signed_area(vertices: &PointVec2) -> f64 {
    let num_points = vertices.x.len();
    let mut j = 1;
    let mut area = 0.0;

    for i in 0..num_points {
        j = j % num_points;
        area += (vertices.x[j] - vertices.x[i]) * (vertices.y[j] + vertices.y[i]);
        j += 1;
    }
    println!("area {}", area * 0.5);
    area * 0.5
}

fn unit_norm_vector(vertex_1: &Point2, vertex_2: &Point2) -> (Point2, f64) {
    let delta = *vertex_1 - *vertex_2;
    let norm = Point2::new(delta.y, -delta.x);
    (norm.unit(), norm.magnitude())
}

/// Returns the line midpoint
fn line_center(vertex_1: &Point2, vertex_2: &Point2) -> Point2 {
    (*vertex_1 + *vertex_2).scale(0.5)
}

/// Returns angle of line with respect to the y-axis
fn get_line_beta(unit_norm: &Point2) -> Angle {
    Angle::Radians(unit_norm.y.atan2(unit_norm.x))
}

fn get_field_line(center: &Point2, length: &f64, kr: &f64) -> Result<Point2, MagnetError> {
    sheet_field(&center.x, &center.y, length, kr)
}

// def signed_area(polygon):
//         """Calculates signed area of a polygon
//
//         Args:
//             polygon (Polygon): Polygon instance
//
//         Returns:
//             float: signed area
//         """
//         j = 1
//         NP = polygon.num_vertices()
//         area = 0
//         norm = _np.empty([NP, 2])
//         center = _np.empty([NP, 2])
//         beta = _np.empty(NP)  # angle w.r.t. y axis
//         length = _np.empty(NP)
//
//         for i in range(NP):
//             j = j % NP
//             area += (polygon.vertices[j][0] - polygon.vertices[i][0]) * (
//                 polygon.vertices[j][1] + polygon.vertices[i][1]
//             )
//             norm[i, :], length[i] = LineUtils.unit_norm(
//                 polygon.vertices[i], polygon.vertices[j]
//             )
//             center[i, :] = LineUtils.line_center(
//                 polygon.vertices[i], polygon.vertices[j]
//             )
//             j += 1
//
//         # check winding order of polygon, area < 0 for clockwise ordering of points
//         if area < 0:
//             norm *= -1
//         beta[:] = _np.rad2deg(_np.arctan2(norm[:, 1], norm[:, 0]))
//
//         return area / 2.0, norm, beta, length, center

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_signed_area() {
        let param = PolyDimension::Side(2.0);
        let vertex_wrapper = Vertices::Regular(4, param);
        let (vertices, _) =
            return_vert_num_vert(&vertex_wrapper, &Point2::default(), &0.0).unwrap();

        let area = signed_area(&vertices);
        let comp_area = 4.0;
        assert_eq!(area, comp_area);
    }
}
