/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

//! Crate Level documentation in main

// #![allow(clippy::many_single_char_names)]
// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![warn(missing_docs)]

use anyhow::Result;
use magnet_rs::config::{Args, SimResult};
use magnet_rs::magnets::{loop_field_2d, MagnetType2D, Rectangle};
use magnet_rs::points::*;
use magnet_rs::utils::conversions::Angle;
fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    // magnet_test();

    let (magnet_list, points) = magnet_rs::config::parse_config_file(&infile)?;
    println!("Points:\n{:?}", points);

    let field = points.get_field(&magnet_list);

    let toml_test = toml::to_string(&field)?;
    println!("{}", toml_test);

    // let mag_toml = magnet_list[0].to;
    println!("Field:\n{:?}", field);

    // let sim_res = SimResult::new(magnet_list., points, field);

    Ok(())
}

fn magnet_test() {
    // Create Magnet Registry
    let mut magnet_list = Vec::<MagnetType2D>::new();

    // Create Magnets
    let m1 = Rectangle::new(
        1.0,
        1.0,
        (-0.5, -0.5),
        Angle::Degrees(0.0),
        1.0,
        Angle::Degrees(90.0),
    );
    magnet_list.push(MagnetType2D::Rectangle(m1));
    let m2 = Rectangle::new(
        1.0,
        1.0,
        (0.5, -0.5),
        Angle::Degrees(0.0),
        -1.0,
        Angle::Degrees(90.0),
    );
    magnet_list.push(MagnetType2D::Rectangle(m2));

    // Create Test Point
    let point = Point2::new(0.0, 0.01);

    // Get local field for all magnets in the registry
    let local_field = loop_field_2d(&magnet_list, &point).unwrap();
    println!("Total field is {} at point {}", local_field, point);
    assert_eq!(local_field.x, 1.357145077959237);
}
