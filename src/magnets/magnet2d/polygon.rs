/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

#[allow(unused_imports)]
use crate::magnets::{GetCenter, GetField, Magnet};
use std::fmt;
use std::ops::{Add, Mul};
// PointVecs2, PolarPoint
use crate::points::{Point2, PointVec2};
use crate::PI;

#[derive(Clone, Debug)]
pub struct Polygon {
    pub center: Point2,
    pub alpha: f64,
    pub jr: f64,
    pub phi: f64,
    pub jx: f64,
    pub jy: f64,
    pub vertices: PointVec2,
    pub num_vertices: usize,
}

impl Default for Polygon {
    /// Default method for Polygon.
    ///
    /// Generates a circular magnet  of radius 1.0, centred at (0,0),
    /// with a magnetisation of 1 tesla in y
    fn default() -> Self {
        Polygon {
            center: Point2::new(0.0, 0.0),
            alpha: 0.0,
            jr: 1.0,
            phi: 90.0_f64.to_radians(),
            jx: 0.0,
            jy: 1.0,
            vertices: PointVec2::new(vec![-1.0, 1.0, 1.0, -1.0], vec![1.0, 1.0, -1.0, -1.0]),
            num_vertices: 4,
        }
    }
}

impl Polygon {
    /// Creates a new Polygon
    ///
    /// Vertices is an enum for generating either a regular polygon, or a custom polygon.
    ///
    /// For the former, `Vertices::
    pub fn new<C, A, J, P>(center: C, alpha: A, jr: J, phi: P, vertices: Vertices) -> Polygon
    where
        C: GetCenter<Point2>,
        A: Into<f64> + Mul<Output = A> + Add<Output = A> + Copy,
        J: Into<f64> + Mul<Output = J> + Add<Output = J> + Copy,
        P: Into<f64> + Mul<Output = P> + Add<Output = P> + Copy,
    {
        // let num_vertices = Vertices::Some.x().len();
        let (returned_vert, num_vertices) =
            return_vert_num_vert(&vertices, &center.center(), &alpha.into());

        Polygon {
            center: center.center(),
            alpha: alpha.into(),
            jr: jr.into(),
            phi: phi.into(),
            jx: jr.into() * phi.into().cos(),
            jy: jr.into() * phi.into().sin(),
            vertices: returned_vert,
            num_vertices: num_vertices,
        }
    }
}

impl fmt::Display for Polygon {
    /// Implements Display for Polygonal magnets.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[c: {},\talpha:{}\tJ ({:.3}, {:.3})\nNo. vertices: {}]",
            self.center,
            self.alpha,
            self.jr,
            self.phi.to_degrees(),
            self.num_vertices,
        )
    }
}

/// Enum containing the Apothem, side length, or radius of a regular polygon
pub enum PolyDimension {
    Apothem(f64),
    Radius(f64),
    Side(f64),
}

/// Enum to define if a polygon is regular or custom
pub enum Vertices {
    Regular(usize, PolyDimension),
    Custom(PointVec2),
}

/// Returns the radius of a regular polygon depending on which of three parameter
/// types are used:
///
/// - Apothem
/// - Side (length)
/// - Radius
fn get_radius(num_vertices: &usize, param: &PolyDimension) -> f64 {
    match param {
        PolyDimension::Apothem(v) => v / (PI / *num_vertices as f64).cos(),
        PolyDimension::Side(v) => v / 2.0 / (PI / *num_vertices as f64).sin(),
        PolyDimension::Radius(v) => *v,
    }
}

/// Offset angle needed for aligning generated vertices
fn offset_angle(num_vertices: &usize, alpha: &f64) -> f64 {
    if num_vertices % 2 == 0 {
        (PI / *num_vertices as f64) + alpha
    } else {
        (PI / *num_vertices as f64) + PI + alpha
    }
}

/// Returns the vertices of a regular polygon in a PointVec2 struct.
/// One of apotherm, side length, or radius must be defined using the PolyDimension enum
///
fn generate_polygon(
    num_vertices: &usize,
    center: &Point2,
    param: &PolyDimension,
    alpha: &f64,
) -> PointVec2 {
    assert!(*num_vertices > 2);
    let offset = offset_angle(num_vertices, &alpha);
    let radius = get_radius(num_vertices, param);
    let xv: Vec<f64> = (0..*num_vertices)
        .into_iter()
        .map(|k| center.x + radius * ((2.0 * PI * k as f64 / *num_vertices as f64) + offset).sin())
        .collect();
    let yv: Vec<f64> = (0..*num_vertices)
        .into_iter()
        .map(|k| center.y + radius * ((2.0 * PI * k as f64 / *num_vertices as f64) + offset).cos())
        .collect();

    PointVec2::new(xv, yv)
}

/// Returns a tuple of the Vertices and number of vertices
fn return_vert_num_vert(
    vertex_wrapper: &Vertices,
    center: &Point2,
    // param: &PolyDimension,
    alpha: &f64,
) -> (PointVec2, usize) {
    match vertex_wrapper {
        Vertices::Regular(val, param) => (generate_polygon(val, center, param, alpha), *val),
        // TODO: Replace clone with something smarter
        Vertices::Custom(val) => (val.clone(), val.x.len()),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::PI_2;

    #[test]
    fn test_polygon_default() {
        let magnet = Polygon::default();
        println!("{}", magnet);

        assert!(true);
    }

    #[test]
    fn test_radius_apothem_square() {
        let param = PolyDimension::Apothem(2.0);
        let num_vertices = 4;
        let radius = get_radius(&num_vertices, &param);
        assert_eq!(radius, 2.0 / (PI / 4.0).cos());
    }

    #[test]
    fn test_radius_side_hexagon() {
        let param = PolyDimension::Side(1.0);
        let num_vertices = 6;
        let radius = get_radius(&num_vertices, &param);
        assert_eq!(radius, 1.0 / 2.0 / (PI / 6.0).sin());
    }

    #[test]
    fn test_radius_radius_septagon() {
        let param = PolyDimension::Radius(2.0);
        let num_vertices = 7;
        let radius = get_radius(&num_vertices, &param);
        assert_eq!(radius, 2.0);
    }

    #[test]
    fn test_gen_poly_square() {
        let param = PolyDimension::Side(2.0);
        let vertex_wrapper = Vertices::Regular(4, param);
        let (vertices, num_vertices) =
            return_vert_num_vert(&vertex_wrapper, &Point2::default(), &0.0);
        let comp_vert = PointVec2 {
            x: vec![1.0, 1.0000000000000002, -1.0, -1.0000000000000002],
            y: vec![1.0000000000000002, -1.0, -1.0000000000000002, 1.0],
        };
        assert_eq!(vertices, comp_vert);
        assert_eq!(num_vertices, 4)
    }

    #[test]
    fn test_gen_poly_hex() {
        let param = PolyDimension::Side(3.0);
        let vertex_wrapper = Vertices::Regular(6, param);
        let (vertices, num_vertices) =
            return_vert_num_vert(&vertex_wrapper, &Point2::default(), &0.0);
        let comp_vert = PointVec2 {
            x: vec![
                1.5,
                3.0000000000000004,
                1.5000000000000013,
                -1.4999999999999996,
                -3.0000000000000004,
                -1.4999999999999991,
            ],
            y: vec![
                2.5980762113533165,
                0.00000000000000018369701987210302,
                -2.598076211353316,
                -2.598076211353317,
                -0.000000000000000551091059616309,
                2.598076211353317,
            ],
        };
        assert_eq!(vertices, comp_vert);
        assert_eq!(num_vertices, 6)
    }

    #[test]
    fn test_custom_poly() {
        let vertex_wrapper = Vertices::Custom(PointVec2 {
            x: vec![1.0, 1.0, -1.0, -1.0],
            y: vec![1.0, -1.0, -1.0, 1.0],
        });
        let (vertices, num_vertices) =
            return_vert_num_vert(&vertex_wrapper, &Point2::default(), &0.0);
        let comp_vert = PointVec2 {
            x: vec![1.0, 1.0, -1.0, -1.0],
            y: vec![1.0, -1.0, -1.0, 1.0],
        };
        assert_eq!(vertices, comp_vert);
        assert_eq!(num_vertices, comp_vert.x.len())
    }

    #[test]
    fn test_polygon_new_square() {
        let magnet = Polygon::new(
            (0.0, 0.0),
            0.0,
            1.0,
            PI_2,
            Vertices::Regular(4, PolyDimension::Side(2.0)),
        );

        println!("{}", magnet);
        let comp_vert = PointVec2 {
            x: vec![1.0, 1.0000000000000002, -1.0, -1.0000000000000002],
            y: vec![1.0000000000000002, -1.0, -1.0000000000000002, 1.0],
        };

        assert_eq!(magnet.vertices, comp_vert);
        assert_eq!(magnet.jr, 1.0);
    }

    #[test]
    fn test_polygon_custom() {
        let vertex_wrapper = Vertices::Custom(PointVec2 {
            x: vec![1.0, 1.0, -1.0, -1.0],
            y: vec![1.0, -1.0, -1.0, 1.0],
        });

        let magnet = Polygon::new((0.0, 0.0), 0.0, 1.0, PI_2, vertex_wrapper);

        println!("{}", magnet);
        let comp_vert = PointVec2 {
            x: vec![1.0, 1.0, -1.0, -1.0],
            y: vec![1.0, -1.0, -1.0, 1.0],
        };

        assert_eq!(magnet.vertices, comp_vert);
        assert_eq!(magnet.jr, 1.0);
    }
}
