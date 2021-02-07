/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
#![allow(dead_code)]

// extern crate num_traits;

pub mod magnets;
use magnets::{DotProduct, Point2};

const NAN: f64 = f64::NAN;
const PI: f64 = std::f64::consts::PI;

fn main() {
    println!("=====f64====");
    let p1 = Point2 { x: 1.0, y: 2.0 };
    let p2 = Point2 { x: 3.0, y: 4.0 };
    let d1 = p1.dot(&p2);
    println!("Dot Product {}", d1);

    println!("p1 + p2 {}", p1 + p2);
    println!("p1 - p2 {}", p1 - p2);
    println!("p1 * p2 {}", p1 * p2);
    println!("p1 / p3 {}", p1 / p2);

    println!("=====u32====");
    let p1 = Point2 { x: 1_u32, y: 2_u32 };
    let p2 = Point2 { x: 3_u32, y: 4_u32 };
    let d1 = p1.dot(&p2);
    println!("Dot Product {}", d1);
    println!("p1 + p2 {}", p1 + p2);
    println!("p1 * p2 {}", p1 * p2);
    println!("p1 / p3 {}", p1 / p2);

    println!("=====i32====");
    let p1 = Point2 { x: 1_i32, y: 2_i32 };
    let p2 = Point2 { x: 3_i32, y: 4_i32 };
    let d1 = p1.dot(&p2);
    println!("Dot Product {}", d1);
    println!("p1 + p2 {}", p1 + p2);
    println!("p1 - p2 {}", p1 - p2);
    println!("p1 * p2 {}", p1 * p2);
    println!("p1 / p3 {}", p1 / p2);
}
