/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

use crate::magnets::magnet2d::{generate_line_array, LineVec};
use crate::magnets::{GetCenter, GetField};
use crate::utils::conversions::Angle;

use crate::points::{Point2, PointVec2, Points2};
use crate::{MagnetError, PI};

use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Polygon {
    pub center: Point2,
    pub alpha: Angle,
    pub jr: f64,
    pub phi: Angle,
    pub jx: f64,
    pub jy: f64,
    pub vertices: PointVec2,
    pub num_vertices: usize,
    pub line_array: LineVec,
}

impl Default for Polygon {
    /// Default method for Polygon.
    ///
    /// Generates a circular magnet  of radius 1.0, centred at (0,0),
    /// with a magnetisation of 1 tesla in y
    fn default() -> Self {
        let vertices = PointVec2::new(vec![1.0, 1.0, -1.0, -1.0], vec![1.0, -1.0, -1.0, 1.0]);
        let (line_array, _, _) = generate_line_array(&vertices, &0.0, &1.0);

        Polygon {
            center: Point2::new(0.0, 0.0),
            alpha: Angle::Degrees(0.0),
            jr: 1.0,
            phi: Angle::Degrees(90.0),
            jx: 0.0,
            jy: 1.0,
            vertices,
            num_vertices: 4,
            line_array,
        }
    }
}

impl Polygon {
    /// Creates a new Polygon
    ///
    /// Vertices is an enum for generating either a regular polygon, or a custom polygon.
    ///
    /// For the former, `Vertices::
    pub fn new<C>(center: C, alpha: Angle, jr: f64, phi: Angle, vertices: Vertices) -> Polygon
    where
        C: GetCenter<Point2>,
    {
        let phi_rad = phi.to_radians();
        let returned_vert = generate_vertices_wrapper(vertices, &center.center(), &alpha).unwrap();
        let num_vertices = returned_vert.x.len();
        let jx = jr * phi_rad.cos();
        let jy = jr * phi_rad.sin();
        let (line_array, _, _) = generate_line_array(&returned_vert, &jx, &jy);

        Polygon {
            center: center.center(),
            alpha,
            jr,
            phi,
            jx,
            jy,
            vertices: returned_vert,
            num_vertices,
            line_array,
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
            self.alpha.to_degrees(),
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
fn offset_angle(num_vertices: &usize, alpha: &Angle) -> Angle {
    if num_vertices % 2 == 0 {
        Angle::Radians((PI / *num_vertices as f64) + alpha.to_radians())
    } else {
        Angle::Radians((PI / *num_vertices as f64) + PI + alpha.to_radians())
    }
}

/// Returns the vertices of a regular polygon in a PointVec2 struct.
/// One of apotherm, side length, or radius must be defined using the PolyDimension enum
///
fn regular_vertices(
    num_vertices: &usize,
    center: &Point2,
    param: &PolyDimension,
    alpha: &Angle,
) -> Result<PointVec2, MagnetError> {
    if *num_vertices < 3 {
        Err(MagnetError::PolygonSideError())
    } else {
        // assert!(
        //     *num_vertices > 2,
        //     "Error: There must be at least 3 vertices"
        // );
        let offset = offset_angle(num_vertices, alpha).to_radians();
        let radius = get_radius(num_vertices, param);
        let xv: Vec<f64> = (0..*num_vertices)
            .into_iter()
            .map(|k| {
                center.x + radius * ((2.0 * PI * k as f64 / *num_vertices as f64) + offset).sin()
            })
            .collect();
        let yv: Vec<f64> = (0..*num_vertices)
            .into_iter()
            .map(|k| {
                center.y + radius * ((2.0 * PI * k as f64 / *num_vertices as f64) + offset).cos()
            })
            .collect();

        Ok(PointVec2::new(xv, yv))
    }
}

/// Returns the vertices of a polygon
pub fn generate_vertices_wrapper(
    vertex_wrapper: Vertices,
    center: &Point2,
    alpha: &Angle,
) -> Result<PointVec2, MagnetError> {
    match vertex_wrapper {
        Vertices::Regular(val, param) => Ok(regular_vertices(&val, center, &param, alpha)?),

        Vertices::Custom(val) => Ok(val),
    }
}

// Return the magnetic field at a point due to a polygon
fn get_field_polygon(magnet: &Polygon, point: &Point2) -> Result<Point2, MagnetError> {
    let mut field = Point2::zero();

    // Translate and rotate into local coordinates
    // let local_point = *point - magnet.center;

    // if magnet.alpha.to_radians().abs() > FP_CUTOFF {
    //     let reverse_alpha = M2_PI - magnet.alpha.to_radians();
    //     local_point = local_point.rotate(&reverse_alpha);
    // }

    for line in &magnet.line_array {
        field += line.field(point)?;
    }

    // if magnet.alpha.to_radians().abs() > FP_CUTOFF {
    //     field = field.rotate(&magnet.alpha.to_radians());
    // }

    Ok(field)
}

impl GetField<&Point2, Result<Point2, MagnetError>> for Polygon {
    /// Returns the magnetic field of a polygon magnet at a Point2 struct {x,y}
    fn field(&self, point: &Point2) -> Result<Point2, MagnetError> {
        get_field_polygon(self, point)
    }
}

impl GetField<&(f64, f64), Result<(f64, f64), MagnetError>> for Polygon {
    /// Returns the magnetic field of a line magnet at a 2-element tuple (x,y)
    fn field(&self, point: &(f64, f64)) -> Result<(f64, f64), MagnetError> {
        let field_vec = get_field_polygon(
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
    use crate::PI_2;

    #[test]
    fn test_regular_vertices_fail() {
        let vertex = regular_vertices(
            &2,
            &Point2::zero(),
            &PolyDimension::Side(1.0),
            &Angle::Degrees(0.0),
        );
        assert!(vertex.is_err());
    }

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
        let vertices =
            generate_vertices_wrapper(vertex_wrapper, &Point2::default(), &Angle::Radians(0.0))
                .unwrap();
        let num_vertices = vertices.x.len();
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
        let vertices =
            generate_vertices_wrapper(vertex_wrapper, &Point2::default(), &Angle::Radians(0.0))
                .unwrap();
        let num_vertices = vertices.x.len();
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
        let vertices =
            generate_vertices_wrapper(vertex_wrapper, &Point2::default(), &Angle::Radians(0.0))
                .unwrap();
        let num_vertices = vertices.x.len();

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
            Angle::Radians(0.0),
            1.0,
            Angle::Radians(PI_2),
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

        let magnet = Polygon::new(
            (0.0, 0.0),
            Angle::Radians(0.0),
            1.0,
            Angle::Radians(PI_2),
            vertex_wrapper,
        );

        println!("{}", magnet);
        let comp_vert = PointVec2 {
            x: vec![1.0, 1.0, -1.0, -1.0],
            y: vec![1.0, -1.0, -1.0, 1.0],
        };

        assert_eq!(magnet.vertices, comp_vert);
        assert_eq!(magnet.jr, 1.0);
    }
}
