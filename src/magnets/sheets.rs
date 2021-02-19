/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
/// # Sheets Module
/// This consists of modules for calculating magnetic fields due to 
/// 

use std::fmt;
use std::ops::{Add, Mul};

use crate::utils::points2::{Point2, Points2};
use crate::PI;

pub trait Magnet {}
pub trait Magnet2: Magnet {}

#[derive(Copy, Clone)]
pub struct Rectangle {
    /// A 2D magnet
    /// 
    /// Arguments: 
    /// 
    /// ::default method generates a square magnet of size 1 x 1, centred at
    /// (0, 0), magnetised in y with a remnant magnetisation of 1 T
    /// # Example
    /// ```rust
    /// use magma::magnets::sheets::Rectangle;
    /// let magnet_1 = Rectangle::default();
    /// println!("Magnet 1:{}", magnet_1);
    /// let magnet_2 = Rectangle::new(1.0, 1.0, (0.0, -1.0 / 2.0), 0.0, 1.0, 90);
    /// println!("Magnet 2:{}", magnet_2);
    /// ```
    /// 
    pub width: f64,
    pub height: f64,
    pub center: Point2,
    pub alpha: f64,
    pub jr: f64,
    pub theta: f64,
    a: f64,
    b: f64,
    jx: f64,
    jy: f64,
}

impl Magnet for Rectangle {}
impl Magnet2 for Rectangle {}

// #[derive(Clone, Default)]
// pub struct PointArray2 {
//     pub x: Vec<f64>,
//     pub y: Vec<f64>,
// }

pub trait ReturnCenter2 {
    fn return_center2(&self) -> Point2;
}

impl ReturnCenter2 for Point2 {
    fn return_center2(&self) -> Point2 {
        *self
    }
}

// impl Into<f64> for (T, T) {
//     fn into(self) -> (f64, f64) {
//         (self.0 as f64, self.1 as f64)
//     }

// }

impl<T: Into<f64> + Copy, U: Into<f64> + Copy> ReturnCenter2 for (T, U) {
    fn return_center2(&self) -> Point2 {
        Point2 {
            x: self.0.into(),
            y: self.1.into(),
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            width: 1.0,
            height: 1.0,
            center: Point2::new(0.0, 0.0),
            alpha: 0.0,
            jr: 1.0,
            theta: 0.0,
            jx: 0.0,
            jy: 1.0,
            a: 0.5,
            b: 0.5,
        }
    }
}

impl Rectangle {
    pub fn new<W, H, A, J, T, C>(
        width: W,
        height: H,
        center: C,
        alpha: A,
        jr: J,
        theta: T,
    ) -> Rectangle
    where
        W: Into<f64> + Mul<Output = W> + Add<Output = W> + Copy,
        H: Into<f64> + Mul<Output = H> + Add<Output = H> + Copy,
        C: ReturnCenter2,
        A: Into<f64> + Mul<Output = A> + Add<Output = A> + Copy,
        J: Into<f64> + Mul<Output = J> + Add<Output = J> + Copy,
        T: Into<f64> + Mul<Output = T> + Add<Output = T> + Copy,
    {
        Rectangle {
            width: width.into(),
            height: height.into(),
            center: center.return_center2(),
            alpha: alpha.into(),
            jr: jr.into(),
            theta: theta.into(),
            jx: jr.into() * (theta.into() * PI / 180.).cos(),
            jy: jr.into() * (theta.into() * PI / 180.).sin(),
            a: width.into() / 2.0,
            b: height.into() / 2.0,
        }
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[h: {},\tw: {},\tc: {},\talpha:{}\tJ ({:.3}, {:.3})]",
            self.width, self.height, self.center, self.alpha, self.jx, self.jy
        )
    }
}

//
pub fn magnetic_field_x(magnet: &Rectangle, point: &Point2) -> Result<Point2, ()> {
    // println!("Magnet\n{}", magnet);
    if magnet.jx.abs() > 1e-6 {
        let b = Point2 {
            x: field_in_x_for_x_mag(point.x, point.y, magnet.a, magnet.b, magnet.jx),
            y: field_in_y_for_x_mag(point.x, point.y, magnet.a, magnet.b, magnet.jx),
        };
        Ok(b)
    } else {
        let b = Point2 { x: 0.0, y: 0.0 };
        Ok(b)
    }
}

pub fn field_in_x_for_x_mag(x: f64, y: f64, a: f64, b: f64, j: f64) -> f64 {
    // when internals of atan2 = 1, then atan2  = PI/4
    // and thus when J = 1, Bxx = 0.5
    let x_sq = x.powi(2);
    let a_sq = a.powi(2);
    let a2 = 2.0 * a;
    let b_plus_y = b + y;
    let b_minus_y = b - y;
    let xsq_minus_asq = x_sq - a_sq;

    (j / (2.0 * PI))
        * ((a2 * b_plus_y).atan2(xsq_minus_asq + b_plus_y.powi(2))
            + (a2 * b_minus_y).atan2(xsq_minus_asq + b_minus_y.powi(2)))
}

pub fn field_in_y_for_x_mag(x: f64, y: f64, a: f64, b: f64, j: f64) -> f64 {
    // when internals of ln = 1, then ln  = 0
    // and thus Byx = 0
    let x_plus_a_sq = (x + a).powi(2);
    let x_minus_a_sq = (x - a).powi(2);

    let y_plus_b_sq = (y + b).powi(2);
    let y_minus_b_sq = (y - b).powi(2);

    (-j / (4.0 * PI))
        * (((x_minus_a_sq + y_minus_b_sq) / (x_plus_a_sq + y_minus_b_sq)).ln()
            - ((x_minus_a_sq + y_plus_b_sq) / (x_plus_a_sq + y_plus_b_sq)).ln())
}

pub fn magnetic_field_y(magnet: &Rectangle, point: &Point2) -> Result<Point2, ()> {
    // println!("Magnet\n{}", magnet);
    let b = Point2 {
        x: field_in_x_for_y_mag(point.x, point.y, magnet.a, magnet.b, magnet.jy),
        y: field_in_y_for_y_mag(point.x, point.y, magnet.a, magnet.b, magnet.jy),
    };
    Ok(b)
}

pub fn field_in_x_for_y_mag(x: f64, y: f64, a: f64, b: f64, j: f64) -> f64 {
    // when internals of ln = 1, then ln  = 0
    // and thus Bxy = 0

    (j / (4.0 * PI))
        * ((((x + a).powi(2) + (y - b).powi(2)) / ((x + a).powi(2) + (y + b).powi(2))).ln()
            - (((x - a).powi(2) + (y - b).powi(2)) / ((x - a).powi(2) + (y + b).powi(2))).ln())
}

pub fn field_in_y_for_y_mag(x: f64, y: f64, a: f64, b: f64, j: f64) -> f64 {
    // when internals of atan2 = 1, then atan2  = PI/4
    // and thus when J = 1, Byy = 0.5
    (j / (2.0 * PI))
        * (((2.0 * b * (x + a)).atan2((x + a).powi(2) + y.powi(2) - b.powi(2)))
            - ((2.0 * b * (x - a)).atan2((x - a).powi(2) + y.powi(2) - b.powi(2))))
}

pub fn magnetic_field(magnet: &Rectangle, point: &Point2) -> Result<Point2, ()> {
    let mut b = Point2::zero();
    // println!("Magnet\n{}", magnet);
    b += if magnet.jx.abs() > 1e-6 {
        Point2 {
            x: field_in_x_for_x_mag(point.x, point.y, magnet.a, magnet.b, magnet.jx),
            y: field_in_y_for_x_mag(point.x, point.y, magnet.a, magnet.b, magnet.jx),
        }
    } else {
        Point2 { x: 0.0, y: 0.0 }
    };

    b += if magnet.jy.abs() > 1e-6 {
        Point2 {
            x: field_in_x_for_y_mag(point.x, point.y, magnet.a, magnet.b, magnet.jy),
            y: field_in_y_for_y_mag(point.x, point.y, magnet.a, magnet.b, magnet.jy),
        }
    } else {
        Point2 { x: 0.0, y: 0.0 }
    };

    Ok(b)
}
