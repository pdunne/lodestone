/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

//! Lodestone python bindings crate
//!

use lodestone_core::{
    magnets::{GetField, Rectangle},
    points::Point2,
    utils::conversions::Angle,
};

// use lodestone_core::parse::SimResult;
use pyo3::prelude::*;

#[pyclass]
struct PyPoint2 {
    #[pyo3(get, set)]
    pub x: f64,
    #[pyo3(get, set)]
    pub y: f64,
}

#[pymethods]
impl PyPoint2 {
    #[new]
    pub fn new(x: f64, y: f64) -> PyPoint2 {
        PyPoint2 { x, y }
    }
    pub fn printer(&self) {
        println!("{} {}", self.x, self.y);
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn lodestone(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyPoint2>()?;
    m.add_function(wrap_pyfunction!(get_point, m)?)?;
    m.add_function(wrap_pyfunction!(rectangle_field, m)?)?;

    Ok(())
}

#[pyfunction]
fn get_point() -> PyResult<PyPoint2> {
    Ok(PyPoint2 { x: 1.1, y: 2.2 })
    // Ok(Point2::new(0.0, 0.0))
}

#[pyfunction]
fn rectangle_field(
    width: f64,
    height: f64,
    center: (f64, f64),
    alpha: f64,
    jr: f64,
    phi: f64,
    point: (f64, f64),
) -> PyResult<PyPoint2> {
    let magnet = Rectangle::new(
        width,
        height,
        Point2::new(center.0, center.1),
        Angle::Degrees(alpha),
        jr,
        Angle::Degrees(phi),
    );

    let point = Point2::new(point.0, point.0);
    let field = magnet.field(&point).unwrap();
    let wrapped_field = PyPoint2 {
        x: field.x,
        y: field.y,
    };
    Ok(wrapped_field)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_val = 5 - 1;
        assert_eq!(test_val, 4);
    }
}
