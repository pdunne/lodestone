// #![warn(missing_docs)]
//! This test library and binary implements routines for calculating magnetic
//! fields, written in Rust. A more complete Python version can be found on
//! [Github](https://github.com/pdunne/pymagnet), or
//! [PyPi](https://pypi.org/project/pymagnet/)
//! 
//! # User friendly magnetic field calculations
//! This library consists of methods for calculating magnetic fields due
//! to simple objects in 2D and 3D.
//! 
//! 
//! 
//! # Calculation Method
//! ## Exact Analytical Methods
//! 
//! ## Iterative Method for Cylindrical Sources



pub mod magnets;
pub mod utils;

pub const NAN: f64 = f64::NAN;
pub const PI: f64 = std::f64::consts::PI;
