/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

//! Crate Level documentation in main

#![warn(missing_docs)]

mod args;
mod demo;
use anyhow::Result;
use args::Args;
use lodestone_core::parse::SimResult;

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
        demo,
    } = args;

    if demo {
        demo::run_demo()?
    } else {
        let (magnet_list, points) = lodestone_core::parse::parse_config_file(&infile)?;
        if !silent {
            println!("Number of magnets: {}", magnet_list.len());
            println!("Number of points: {}", points.x.len());
        }

        // Calculate the magnetic field
        let field = points.get_field(&magnet_list);
        let units = "mm".to_string();

        let mag_toml = lodestone_core::parse::gen_magnet_toml_2d(&magnet_list)?;

        if outfile.is_some() {
            let sim_res = SimResult::new(mag_toml, points, units, field);
            println!("Saving to {:#?}", outfile.as_ref().unwrap());
            lodestone_core::parse::save_results(&sim_res, &outfile.unwrap())?;
            println!("Done")
        }
    }
    Ok(())
}
