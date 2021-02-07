use crate::routines::burlich;
use crate::PI;
use pyo3::prelude::*;

// use point::

#[pyclass]
struct Point2 {
    #[pyo3(get)]
    x: f64,
    #[pyo3(get)]
    y: f64,
}

#[pymethods]
impl Point2 {
    #[new]
    pub fn new(x: f64, y: f64) -> Self {
        Point2 { x, y }
    }

    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    #[staticmethod]
    pub fn return_pi() -> f64 {
        PI
    }

    pub fn return_cel(&self) -> f64 {
        burlich::cel(self.x, self.y, 1.0, 1.0)
    }
}

#[pymodule]
fn magma(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point2>()?;

    Ok(())
}
