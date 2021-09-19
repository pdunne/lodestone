/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

// #![allow(unused_imports)]

use super::line_field::sheet_field;
use crate::magnets::GetField;
use crate::{
    points::{Point2, PointVec2, Points, Points2},
    utils::conversions::Angle,
    MagnetError,
};
use crate::{FP_CUTOFF, M2_PI, PI};
use serde_derive::{Deserialize, Serialize};

/// Line struct for calculating the magnetic field due to a 2D infinite charge sheet.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Line {
    // lengths in application units (dimensionless)
    pub length: f64,
    // line center
    pub center: Point2,
    // angle between the line and y-axis
    pub beta: Angle,
    // Out-of-plane (z) surface current density in units T
    pub kr: f64,
}

/// Convenience type alias for a Vec of Line
// #[derive(Copy, Clone, Debug)]
pub type LineVec = Vec<Line>;

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
            kr: 1.0,
        }
    }
}

impl Line {
    pub fn new(length: f64, center: Point2, beta: Angle, kr: f64) -> Line {
        Line {
            length,
            center,
            beta,
            kr,
        }
    }
}

/// Returns the signed area of a polygon using the Shoelace Formula, giving
/// area < 0 for clockwise arrays of vertices.
pub fn generate_line_array(vertices: &PointVec2, jx: &f64, jy: &f64) -> (LineVec, f64, Point2) {
    let num_points = vertices.x.len();
    // assert!(num_points % 2 == 0);

    let mut line_array = LineVec::with_capacity(num_points / 2);
    let mut area = 0.0;
    // let mut j = 1;
    let mut centroid = Point2::zero();

    for i in 0..num_points {
        // To ensure j wraps around to get the zeroth element on the final iteration
        let mut j = i + 1;
        j %= num_points;
        let weight = (vertices.x[j] + vertices.x[i]) * (vertices.y[j] - vertices.y[i]);
        centroid.x += (vertices.x[j] + vertices.x[i]) * weight;
        centroid.y += (vertices.y[j] + vertices.y[i]) * weight;

        area += weight;

        let start_point = Point2::new(vertices.x[i], vertices.y[i]);
        let end_point = Point2::new(vertices.x[j], vertices.y[j]);
        let (unit_norm, length) = unit_norm_vector(&start_point, &end_point);

        let center: Point2 = line_center(&start_point, &end_point);
        let beta = get_line_beta(&unit_norm);
        let kr = jx * unit_norm.y - jy * unit_norm.x;
        // println!("C: {}, N: {}, B: {:?}, K: {}", center, unit_norm, beta, kr);
        line_array.push(Line::new(length, center, beta, kr));

        // j += 1;
    }

    // divide by 3
    centroid = centroid.scale(1.0 / area / 3.0);
    //TODO: check winding order of polygon, area > 0 for anti-clockwise ordering of points
    // if area > 0.0 {}
    // then change sign of beta and kr (*-1)

    (line_array, area * 0.5, centroid)
}

/// Returns the unit normal vector between two points, and the distance between them.
fn unit_norm_vector(start_point: &Point2, end_point: &Point2) -> (Point2, f64) {
    let delta = *end_point - *start_point;
    let norm = Point2::new(-delta.y, delta.x);
    let length = delta.magnitude();
    (norm.scale(1.0 / length), length)
}

/// Returns the line midpoint
fn line_center(vertex_1: &Point2, vertex_2: &Point2) -> Point2 {
    (*vertex_1 + *vertex_2).scale(0.5)
}

// /// Returns the line length
// fn line_length(vertex_1: &Point2, vertex_2: &Point2) -> f64 {
//     vertex_2.distance_from_point(vertex_1)
// }

/// Returns angle of line with respect to the y-axis
fn get_line_beta(unit_norm: &Point2) -> Angle {
    Angle::Radians(unit_norm.y.atan2(unit_norm.x))
}

fn get_field_line(magnet: &Line, point: &Point2) -> Result<Point2, MagnetError> {
    // println!("{:?}", magnet);
    let mut field = Point2::zero();
    // println!("input: {}", point);
    // Translate and rotate into local coordinates
    let mut local_point = *point - magnet.center;
    // println!("local: {}, input {}", local_point, point);
    let mut rotation_flag = false;
    if (magnet.beta.to_radians() % PI).abs() > FP_CUTOFF && magnet.kr.abs() > FP_CUTOFF {
        let reverse_angle = M2_PI - magnet.beta.to_radians();
        local_point = local_point.rotate(&reverse_angle);
        rotation_flag = true;
    }

    field += if magnet.kr.abs() > FP_CUTOFF {
        let local_field = sheet_field(
            &local_point.x,
            &local_point.y,
            &(magnet.length / 2.0),
            &magnet.kr,
        );
        match local_field {
            Ok(value) => value,
            Err(_e) => Point2 { x: 0.0, y: 0.0 },
        }
    } else {
        Point2 { x: 0.0, y: 0.0 }
    };

    if rotation_flag {
        let reverse_alpha = magnet.beta.to_radians();
        field = field.rotate(&reverse_alpha);
    }
    Ok(field)
}

impl GetField<&Point2, Result<Point2, MagnetError>> for Line {
    /// Returns the magnetic field of a line magnet at a Point2 struct {x,y}
    fn field(&self, point: &Point2) -> Result<Point2, MagnetError> {
        get_field_line(self, point)
    }
}

impl GetField<&(f64, f64), Result<(f64, f64), MagnetError>> for Line {
    /// Returns the magnetic field of a line magnet at a 2-element tuple (x,y)
    fn field(&self, point: &(f64, f64)) -> Result<(f64, f64), MagnetError> {
        let field_vec = get_field_line(
            self,
            &Point2 {
                x: point.0,
                y: point.1,
            },
        )?;
        Ok((field_vec.x, field_vec.y))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::magnets::magnet2d::polygon::{generate_vertices_wrapper, PolyDimension, Vertices};
    use crate::utils::comparison::nearly_equal;
    // use crate::M2_PI;

    #[test]
    fn test_default_line() {
        let line = Line::default();
        println!("{:?}", line);
        assert!(true);
    }

    #[test]
    fn test_get_line_center_vertical() {
        let start_point = Point2::new(0.5, 0.5);
        let end_point = Point2::new(0.5, -0.5);
        let center = line_center(&start_point, &end_point);

        assert_eq!(center, Point2::new(0.5, 0.0));
    }

    #[test]
    fn test_get_line_center_horizontal() {
        let start_point = Point2::new(-0.5, 0.5);
        let end_point = Point2::new(0.5, 0.5);
        let center = line_center(&start_point, &end_point);

        assert_eq!(center, Point2::new(0.0, 0.5));
    }

    //FIXME:Check test value
    #[test]
    fn test_default_get_field() {
        let line = Line::default();
        let point = Point2::new(0.5, 0.0);
        let field = line.field(&point).unwrap();
        // let comp_field = (1.0_f64).atan2(0.5 * 0.5 - 1.0) / M2_PI;
        assert_eq!(Point2::new(0.0, 0.25), field);
    }

    #[test]
    fn test_generate_line_array() {
        let param = PolyDimension::Side(2.0);
        let vertex_wrapper = Vertices::Regular(4, param);
        let vertices =
            generate_vertices_wrapper(vertex_wrapper, &Point2::default(), &Angle::Radians(0.0))
                .unwrap();
        // println!("{:?}", vertices);
        let (_line_array, area, centroid) = generate_line_array(&vertices, &0.0, &1.0);
        let comp_area = -4.0;

        let mut comp_line_array = LineVec::with_capacity(4);
        let comp_line = Line::new(1.0, Point2::new(0.0, 1.0), Angle::Degrees(90.0), 1.0);
        comp_line_array.push(comp_line);

        assert_eq!(area, comp_area, "Compare areas");

        let centroid_comp = nearly_equal(0.0, centroid.x) && nearly_equal(0.0, centroid.y);
        assert!(centroid_comp, "Centroid should be zero");
        println!("{:?}", _line_array);
        // assert_eq?!(comp_line_array, line_array);
    }
}
