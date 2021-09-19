# Magnet_RS

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)
[![License: CC BY-SA 4.0](https://img.shields.io/badge/License-CC%20BY--SA%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-sa/4.0/)
[![build status](https://github.com/pdunne/lodestone/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/pdunne/lodestone/actions/workflows/rust.yml)
[![crate](https://img.shields.io/crates/v/magnet_rs.svg)](https://crates.io/crates/magnet_rs)
[![documentation](https://docs.rs/magnet_rs/badge.svg)](https://docs.rs/magnet_rs)

Magnet_RS is a command line program to calculate the magnetic fields of any
object or objects in 2D and 3D; along with any induced forces and torques. This
uses the `lodestone_core` crate, which is a rewrite of a python package,
`pymagnet`:

[Github](https://github.com/pdunne/pymagnet), or
[PyPi](https://pypi.org/project/pymagnet/).

## Description

This binary reads a toml file containing the magnets and list of points to run
the calculation over. This is then saved to a JSON file with the following keys:

* `magnets`: an array of the magnets and their properties
* `points`: points where the field is calculated
* `field`: calculated magnetic field

Currently, only 2D features are available.

## Example

Run the demo calculation:

```bash
magnet_rs -d
```

which saves the computed field to `example_out.json`

The Python script in the data folder, `plot_example.py` will plot the resulting
json file.

### Reading input files

Save the following into `input.toml`

```toml
[[magnet]]
kind = "rectangle"
size = [1.0, 1.0]
center = [-1.0, -0.5]
magnetisation = [1.0, 90.0]
magAngle = "degrees"
alpha = 0.0
alphaAngle = "degrees"

[[magnet]]
kind = "rectangle"
size = [1.0, 1.0]
center = [1.0, -2.0]
magnetisation = [-1.0, 0.5]
magAngle = "degrees"
alpha = 0.0
alphaAngle = "degrees"


# Then define the type of grid for calculating over
[grid]
kind = "grid"
start = [-2.0, -2.0]
stop = [2.0, 2.0]
numPoints = 101
units = "mm" # NOTE: Units are not yet implemented 
```

then run the following to save the data in a JSON file:

```bash
magnet_rs -i input.toml -o out.json
```

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
