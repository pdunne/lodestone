use crate::{DotProduct, Point2, };
use crate::{NAN, PI};

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub center: Point2<f64>,
}

pub fn field_in_x_for_x(x: f64, y: f64, a: f64, b: f64, J: f64) -> f64 {
    let x_sq = x.powi(2);
    let a_sq = a.powi(2);
    let a2 = 2.0 * a;
    let b_plus_y = b + y;
    let b_minus_y = b - y;
    let xsq_min_asq = x_sq - a_sq;

    (J / (2.0 * PI))
        * ((a2 * b_plus_y).atan2(xsq_min_asq + b_plus_y.powi(2))
            + (a2 * b_minus_y).atan2(xsq_min_asq + b_minus_y.powi(2)))
}


pub fn field_in_y_for_x(x: f64, y: f64, a: f64, b: f64, J: f64) -> f64 {
    let x_sq = x.powi(2);
    let a_sq = a.powi(2);
    let a2 = 2.0 * a;
    let b_plus_y = b + y;
    let b_minus_y = b - y;
    let xsq_min_asq = x_sq - a_sq;

    (J / (2.0 * PI))
        * ((a2 * b_plus_y).atan2(xsq_min_asq + b_plus_y.powi(2))
            + (a2 * b_minus_y).atan2(xsq_min_asq + b_minus_y.powi(2)))
}