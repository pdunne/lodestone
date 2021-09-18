/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

//! Crate Level documentation in main

#![warn(missing_docs)]

use anyhow::Result;
use magnet_rs::config::{Args, SimResult};

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;

    // Get list of magnets and points to run calculation on
    let (magnet_list, points) = magnet_rs::config::parse_config_file(&infile)?;

    if !silent {
        println!("Number of magnets: {}", magnet_list.len());
        println!("Number of points: {}", points.x.len());
    }

    // Calculate the magnetic field
    let field = points.get_field(&magnet_list);
    let units = "mm".to_string();

    let mag_toml = magnet_rs::config::gen_magnet_toml_2d(&magnet_list)?;

    if outfile.is_some() {
        let sim_res = SimResult::new(mag_toml, points, units, field);
        println!("Saving to {:#?}", outfile.as_ref().unwrap());
        magnet_rs::config::save_results(&sim_res, &outfile.unwrap())?;
        println!("Done")
    }

    Ok(())
}
