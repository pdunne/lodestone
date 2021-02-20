/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![warn(missing_docs)]

//! magnet_rs test binary

use magnet_rs::magnets::sheets::{magnetic_field, magnetic_field_x, magnetic_field_y, Rectangle};
use magnet_rs::utils::points2::{Point2, Points2};
use magnet_rs::PI;

fn main() {
    println!("=====f64====");
    let p1 = Point2 { x: 1.0, y: 2.0 };
    let p2 = Point2 { x: 3.0, y: 4.0 };
    let d1 = p1.dot(&p2);
    println!("Dot Product {}", d1);

    let p1 = Point2::new(1, 2);
    println!("p1 {}", p1);

    let w = 1;
    let h = w as f64 * 1.0;
    let jr = 2.0 * PI;

    let m1 = Rectangle::default();
    let m2 = Rectangle::new(w, h, (0, -h / 2.0), 0, jr, 45);
    let m3 = Rectangle::new(w, h, Point2::new(0., -h / 2.0), 0, jr, 90);
    let m4 = Rectangle::new(w, h, Point2::new(0., -h / 2.0), 0, jr, 0);

    println!("m1 {}", m1);
    println!("m2 {}", m2);
    println!("m3 {}", m3);
    println!("m4 {}", m4);

    let p3 = Point2 { x: 0., y: 0.0 };
    let b = magnetic_field_x(&m2, &p3).unwrap();
    println!("B: {} at {}", b, p3);

    let b = magnetic_field_y(&m2, &p3).unwrap();
    println!("B: {} at {}", b, p3);

    let b = magnetic_field(&m2, &p3).unwrap();
    println!("B: {} at {}", b, p3);

    let b = magnetic_field(&m3, &p3).unwrap();
    println!("B: {} at {}", b, p3);

    let b = magnetic_field(&m4, &p3).unwrap();
    println!("B: {} at {}", b, p3);
    let test: f64 = 0.0_f64.atan2(0.0);
    println!("atan2 {}", test);
}
