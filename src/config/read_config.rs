/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! # Read
//!

use crate::{
    magnets::{Circle, Magnet2D, MagnetVec2D, Rectangle},
    points::{cart_prod_2d_vec, gen_line_2d, Point2, PointVec2},
    utils::conversions::Angle,
    MagnetError,
};
use serde_derive::{Deserialize, Serialize};

/// Stores settings for the grid of points to be generated, and the list of magnets
/// to calculate over.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configure {
    pub grid: GridKind2D,
    pub magnet: Vec<MagnetKind>,
}

/// Convenience enum containing 2D and 3D magnet types used for serializing/deserializing
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum MagnetKind {
    Circle(ReadCircle),
    Rectangle(ReadRectangle),
}

/// Stores input properties of a rectangular 2D magnet
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default = "default_rectangle")]
pub struct ReadRectangle {
    size: [f64; 2],
    center: [f64; 2],
    magnetisation: [f64; 2],
    mag_angle: String,
    alpha: f64,
    alpha_angle: String,
}

impl ReadRectangle {
    pub fn new(
        size: [f64; 2],
        center: [f64; 2],
        magnetisation: [f64; 2],
        mag_angle: String,
        alpha: f64,
        alpha_angle: String,
    ) -> Self {
        ReadRectangle {
            size,
            center,
            magnetisation,
            mag_angle,
            alpha,
            alpha_angle,
        }
    }

    pub fn default() -> Self {
        default_rectangle()
    }
}

fn default_rectangle() -> ReadRectangle {
    ReadRectangle {
        size: [1.0, 1.0],
        center: [0.0, 0.0],
        magnetisation: [1.0, 90.0],
        mag_angle: "degrees".to_string(),
        alpha: 0.0,
        alpha_angle: "degrees".to_string(),
    }
}

/// Stores input properties of a circular 2D magnet
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default = "default_circle")]
pub struct ReadCircle {
    size: f64,
    center: [f64; 2],
    magnetisation: [f64; 2],
    mag_angle: String,
    alpha: f64,
    alpha_angle: String,
}

impl ReadCircle {
    pub fn new(
        size: f64,
        center: [f64; 2],
        magnetisation: [f64; 2],
        mag_angle: String,
        alpha: f64,
        alpha_angle: String,
    ) -> Self {
        ReadCircle {
            size,
            center,
            magnetisation,
            mag_angle,
            alpha,
            alpha_angle,
        }
    }
    pub fn default() -> Self {
        default_circle()
    }
}

fn default_circle() -> ReadCircle {
    ReadCircle {
        size: 1.0,
        center: [0.0, 0.0],
        magnetisation: [1.0, 90.0],
        mag_angle: "degrees".to_string(),
        alpha: 0.0,
        alpha_angle: "degrees".to_string(),
    }
}

/// Enum for distinguishing grid types
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum GridKind2D {
    Point(ReadGrid0D),
    Line(ReadGrid1D),
    Grid(ReadGrid2D),
    Custom(ReadGridCustom),
    None,
}

/// Stores input properties of 2D grid of points (2D)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default = "default_grid2d")]
pub struct ReadGrid2D {
    start: [f64; 2],
    stop: [f64; 2],
    num_points: usize,
}

fn default_grid2d() -> ReadGrid2D {
    ReadGrid2D {
        start: [-2.0, -2.0],
        stop: [2.0, 2.0],
        num_points: 100,
    }
}

/// Stores input properties of a linear array of points (2D)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default = "default_line2d")]
pub struct ReadGrid1D {
    start: [f64; 2],
    stop: [f64; 2],
    num_points: usize,
}

fn default_line2d() -> ReadGrid1D {
    ReadGrid1D {
        start: [-2.0, -2.0],
        stop: [2.0, 2.0],
        num_points: 100,
    }
}

/// Stores input properties of a single point (2D)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default = "default_point2d")]
pub struct ReadGrid0D {
    point: [f64; 2],
}

fn default_point2d() -> ReadGrid0D {
    ReadGrid0D { point: [1.0, 1.0] }
}

/// Stores input properties of a custom grid of points (2D)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default = "default_custom_grid2d")]
pub struct ReadGridCustom {
    x: Vec<f64>,
    y: Vec<f64>,
}

fn default_custom_grid2d() -> ReadGridCustom {
    ReadGridCustom {
        x: vec![-1.0, 0.0, 1.0],
        y: vec![-1.0, 0.0, 1.0],
    }
}

/// Reads in a configuration TOML file and returns a Vec of 2D magnets, and the
/// points to calculate over
pub fn parse_config_file(infile: &str) -> Result<(MagnetVec2D, PointVec2), MagnetError> {
    let config = read_config_file(infile)?;
    let magnet_list = generate_magnets(config.magnet)?;
    let points = generate_points(config.grid)?;

    Ok((magnet_list, points))
}

/// Reads in a configuration TOML file and returns a structured Config
pub fn read_config_file(infile: &str) -> Result<Configure, MagnetError> {
    let config_text = std::fs::read_to_string(infile)?;

    Ok(toml::from_str(&config_text)?)
}

/// Generates the points
pub fn generate_points(grid: GridKind2D) -> Result<PointVec2, MagnetError> {
    let points = match grid {
        GridKind2D::Point(val) => PointVec2::new(vec![val.point[0]], vec![val.point[1]]),
        GridKind2D::Line(val) => gen_line_2d(
            &Point2::new(val.start[0], val.start[1]),
            &Point2::new(val.stop[0], val.stop[1]),
            &val.num_points,
        ),
        GridKind2D::Grid(val) => cart_prod_2d_vec(
            &Point2::new(val.start[0], val.start[1]),
            &Point2::new(val.stop[0], val.stop[1]),
            &val.num_points,
        ),
        GridKind2D::Custom(val) => PointVec2::new(val.x, val.y),
        GridKind2D::None => PointVec2::default(),
    };

    Ok(points)
}

pub fn generate_magnets(magnets: Vec<MagnetKind>) -> Result<MagnetVec2D, MagnetError> {
    let mut magnet_list = MagnetVec2D::new();
    for mag in magnets {
        magnet_list.push(match mag {
            MagnetKind::Circle(val) => Magnet2D::Circle(Circle::new(
                val.size,
                (val.center[0], val.center[1]),
                match val.alpha_angle.as_str() {
                    "degrees" => Angle::Degrees(val.alpha),
                    "radians" => Angle::Radians(val.alpha),
                    _ => Angle::Degrees(val.alpha),
                },
                val.magnetisation[0],
                Angle::Degrees(val.magnetisation[1]),
            )),
            MagnetKind::Rectangle(val) => Magnet2D::Rectangle(Rectangle::new(
                val.size[0],
                val.size[1],
                (val.center[0], val.center[1]),
                match val.alpha_angle.as_str() {
                    "degrees" => Angle::Degrees(val.alpha),
                    "radians" => Angle::Radians(val.alpha),
                    _ => Angle::Degrees(val.alpha),
                },
                val.magnetisation[0],
                match val.mag_angle.as_str() {
                    "degrees" => Angle::Degrees(val.magnetisation[1]),
                    "radians" => Angle::Radians(val.magnetisation[1]),
                    _ => Angle::Degrees(val.magnetisation[1]),
                },
            )),
        });
    }

    Ok(magnet_list)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_read_toml() {
        let config_text = r#"[[magnet]]
kind = "rectangle"
size = [1.0, 1.0]
center = [-0.5, -0.5]
magnetisation = [1.0, 90.0]
magAngle = "degrees"
alpha = 0.0
alphaAngle = "degrees"


[[magnet]]
kind = "rectangle"
size = [1.0, 1.0]
center = [0.5, -0.5]
magnetisation = [-1.0, 90.0]
magAngle = "degrees"
alpha = 0.0
alphaAngle = "degrees"

[grid]
kind = "line"
start = [0.0, -1.01]
stop = [0.0, 0.01]
numPoints = 2"#;
        let config: Configure = toml::from_str(&config_text).unwrap();
        let magnet_list = generate_magnets(config.magnet).unwrap();
        let points = generate_points(config.grid).unwrap();

        let mut magnet_list_vec = MagnetVec2D::new();

        // Create Magnets
        let m1 = Rectangle::new(
            1.0,
            1.0,
            (-0.5, -0.5),
            Angle::Degrees(0.0),
            1.0,
            Angle::Degrees(90.0),
        );
        magnet_list_vec.push(Magnet2D::Rectangle(m1));
        let m2 = Rectangle::new(
            1.0,
            1.0,
            (0.5, -0.5),
            Angle::Degrees(0.0),
            -1.0,
            Angle::Degrees(90.0),
        );
        magnet_list_vec.push(Magnet2D::Rectangle(m2));
        let point_vec = PointVec2::new(vec![0.0, 0.0], vec![-1.01, 0.010000000000000009]);

        assert_eq!(points, point_vec);
        assert_eq!(magnet_list[0], magnet_list_vec[0]);
        assert_eq!(magnet_list[1], magnet_list_vec[1]);
    }
}
