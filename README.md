# Lodestone

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)
[![License: CC BY-SA 4.0](https://img.shields.io/badge/License-CC%20BY--SA%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-sa/4.0/)
[![build status](https://github.com/pdunne/lodestone/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/pdunne/lodestone/actions/workflows/rust.yml)
[![crate](https://img.shields.io/crates/v/lodestone_core.svg)](https://crates.io/crates/lodestone_core)
[![documentation](https://docs.rs/lodestone_core/badge.svg)](https://docs.rs/lodestone_core)

Lodestone is a colletion of packages to calculate magnetic fields of
any object or objects in 2D and 3D; along with any induced forces and torques. This consists of:

* `lodestone_core` a Rust library for handling all calculations and file parsing
* `lodestone_python` a planned wrapper to expose `lodestone_core` as a native module in Python
* `magnet_rs` a command line tool to calculate  

This is a rewrite of a Python package `pymagnet`:
[Github](https://github.com/pdunne/pymagnet), or
[PyPi](https://pypi.org/project/pymagnet/).

## Features

This code uses analytical expressions to calculate the magnetic field due to
simple magnets. These include:

* 2D: rectangles, squares, arbitrary polygons
* 3D: cuboids, cylinders, solenoids, spheres, arbitrary polyhedra

## Licensing

Source code licensed under the [Mozilla Public License Version 2.0](https://www.mozilla.org/en-US/MPL/2.0/)

Documentation is licensed under a Creative Commons Attribution-ShareAlike 4.0 International [(CC BY-SA 4.0)](https://creativecommons.org/licenses/by-sa/4.0/) license.

This is a human-readable summary of (and not a substitute for) the license, adapted from [CS50x](https://cs50.harvard.edu/x/2021/license/). Official translations of this license are available in other languages.

**You are free to:**

* **Share** — copy and redistribute the material in any medium or format.
* **Adapt** — remix, transform, and build upon the material.

**Under the following terms:**

* **Attribution** — You must give appropriate credit, provide a link to the license, and indicate if changes were made. You may do so in any reasonable manner, but not in any way that suggests the licensor endorses you or your use.
* **ShareAlike** — If you remix, transform, or build upon the material, you must distribute your contributions under the same license as the original
* No additional restrictions — You may not apply legal terms or technological measures that legally restrict others from doing anything the license permits.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
