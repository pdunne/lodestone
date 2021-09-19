# Lodestone

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)
[![License: CC BY-SA 4.0](https://img.shields.io/badge/License-CC%20BY--SA%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-sa/4.0/)



This test library and binary implements routines for calculating magnetic
fields, written in Rust. A more complete Python version can be found on
[Github](https://github.com/pdunne/pymagnet), or
[PyPi](https://pypi.org/project/pymagnet/).

Download from [crates.io](https://crates.io/crates/magnet-rs).

## Features

This code uses analytical expressions to calculate the magnetic field due to
simple magnets. These include:

* 2D: rectangles, squares, arbitrary polygons

This binary reads a toml file containing the magnets and list of points to run the calculation over.
This is then saved to a JSON file with the following keys:

* `magnets`: an array of the magnets and their properties
* `points`: points where the field is calculated
* `field`: calculated magnetic field

## Example

Run the demo calculation:

```bash
magnet_rs -d
```

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
