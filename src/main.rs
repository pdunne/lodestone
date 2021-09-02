/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

//! Crate Level documentation in main

// #![allow(clippy::many_single_char_names)]
// #![allow(unused_imports)]
// #![allow(dead_code)]
#![warn(missing_docs)]

use anyhow::Result;
use magnet_rs::config::{Args, SimResult};

// use std::fs::File;

// use pbr::ProgressBar;
fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    // Get list of magnets and points to run calculation on
    let (magnet_list, points) = magnet_rs::config::parse_config_file(&infile)?;

    // Calculate the magnetic field
    let field = points.get_field(&magnet_list);

    let mag_toml = magnet_rs::config::gen_magnet_toml_2d(&magnet_list)?;

    if !silent {
        // println!("Points:\n{:?}", points);
        // let toml_test = toml::to_string(&field)?;

        // println!("{}", toml_test);
        println!("Number of magnets: {}", magnet_list.len());
        println!("Number of points: {}", points.x.len());
        // let toml_test_mag = toml::to_string(&mag_toml)?;
        // println!("Magnets:\n{}", toml_test_mag);
        // println!("Field:\n{:?}", field);
    }

    if outfile.is_some() {
        let sim_res = SimResult::new(mag_toml, points, field);
        magnet_rs::config::save_results(&sim_res, &outfile.unwrap())?;
    }

    Ok(())
}

// fn magnet_test() {
//     // Create Magnet Registry
//     let mut magnet_list = MagnetVec2D::new();
//
//     // Create Magnets
//     let m1 = Rectangle::new(
//         1.0,
//         1.0,
//         (-0.5, -0.5),
//         Angle::Degrees(0.0),
//         1.0,
//         Angle::Degrees(90.0),
//     );
//     magnet_list.push(Magnet2D::Rectangle(m1));
//     let m2 = Rectangle::new(
//         1.0,
//         1.0,
//         (0.5, -0.5),
//         Angle::Degrees(0.0),
//         -1.0,
//         Angle::Degrees(90.0),
//     );
//     magnet_list.push(Magnet2D::Rectangle(m2));
//
//     // Create Test Point
//     let point = Point2::new(0.0, 0.01);
//
//     // Get local field for all magnets in the registry
//     let local_field = loop_field_2d(&magnet_list, &point).unwrap();
//     println!("Total field is {} at point {}", local_field, point);
//     assert_eq!(local_field.x, 1.357145077959237);
// }
