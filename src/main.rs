/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
#![allow(dead_code)]
// #![warn(missing_docs)]

//! magnet_rs test binary

// use std::fs::read;
use std::usize;

use magnet_rs::magnets::magnet2d::circle_field::{get_field_circle, get_polar_field_circle};
use magnet_rs::magnets::magnet2d::Circle;
use magnet_rs::magnets::magnet2d::{Magnet2D, Rectangle};
use magnet_rs::utils::comparison::nearly_equal;
use magnet_rs::utils::conversions::vector_pol2cart;
use magnet_rs::utils::points2::PolarPoints;
use magnet_rs::utils::points2::{Point2, Points2, PolarPoint};
use magnet_rs::{PI, PI_2};

fn main() -> anyhow::Result<()> {
    // magnet_tests();

    Ok(())
}



fn const_gen_test (){


    


}



fn magnet_tests() {
    println!("=====f64====");
    let p1 = Point2 { x: 1.0, y: 2.0 };
    let p2 = Point2 { x: 3.0, y: 4.0 };
    let d1 = p1.dot(&p2);
    println!("Dot Product {}", d1);

    let p1 = Point2::new(1, 2);
    println!("p1 {}", p1);

    let w = 1;
    let h = w as f64 * 1.0;
    let jr = 1.0;

    let m1 = Rectangle::default();
    let m2 = Rectangle::new(w, h, (0, -h / 2.0), 0, jr, 45);
    let m3 = Rectangle::new(w, h, Point2::new(0., -h / 2.0), 0, jr, 90);
    let m4 = Rectangle::new(w, h, Point2::new(0., -h / 2.0), 0, jr, 0);

    println!("m1 {}", m1);
    println!("m2 {}", m2);
    println!("m3 {}", m3);
    println!("m4 {}", m4);

    println!("m4 center is {}", m4.center());

    let p3 = Point2 { x: 0.0, y: 0.0 };

    let b = m1.field(&p3).unwrap();
    println!("B: {} at {}", b, p3);

    let b = m2.field(&p3).unwrap();
    println!("B: {} at {}", b, p3);

    let b = m3.field(&p3).unwrap();
    println!("B: {} at {}", b, p3);

    let b = m4.field(&p3).unwrap();
    println!("B: {} at {}", b, p3);

    surface_field_y();
    println!("Smallest f64 is: {:e}", f64::MIN_POSITIVE);

    let m1 = Circle::default();
    let m2 = Circle::new(1.0, Point2 { x: 0.0, y: -1.0 }, 0.0, 1.0, 0.0);
    println!("m1 {}", m1);
    println!("m2 {}", m2);

    println!("m1 center is {}", m1.center());

    println!("m2 center is {}", m2.center());

    let p3 = Point2 { x: 0.0, y: 0.0 };

    let b = m2.field(&p3).unwrap();
    println!("B: {} at {}", b, p3);

    let b = m2.field(&p3).unwrap();
    println!("B: {} at {}", b, p3);

    let step: usize = 5;
    let stop: usize = 0;
    let start: usize = 10;
    for i in (stop..=start).step_by(step) {
        print!("{} ", i);
    }
    println!(" ");
    for i in (stop..=start).step_by(step).map(|x| x * x) {
        print!("{} ", i);
    }
    println!(" ");
}

fn surface_field_y() {
    let magnet = Circle::default();
    println!("Magnet is: {}", magnet);

    let point1 = PolarPoint {
        rho: magnet.radius,
        phi: PI_2,
    };

    println!("point1 is: {}", point1);

    let field = get_polar_field_circle(&magnet, &point1).unwrap();
    // println!(field);
    let comp_field = PolarPoint {
        rho: 0.0_f64,
        phi: 0.5_f64,
    };
    // let result =
    //     nearly_equal(field.rho(), comp_field.rho()) && nearly_equal(field.phi(), comp_field.phi());

    // println!("field: {}", field);
    println!("comp_field: {}", comp_field);

    // println!("Comparison: {}", result);
    // println!("Diff_rho: {:e}", field.rho() - comp_field.rho());

    let rot_field = vector_pol2cart(&field, point1.phi());
    let rot_comp_field = vector_pol2cart(&comp_field, point1.phi());
    println!("ROTATED:");
    println!("comp_field: {}", rot_field);
    println!("Comparison: {}", rot_comp_field);

    println!("Point 1 in cartesian {}", point1.to_cartesian());
    let test_field = get_field_circle(&magnet, &point1.to_cartesian()).unwrap();
    println!("test_field: {}", test_field);

    println!("TOP");
    let point2 = Point2::new(0.0, magnet.radius);
    let test_field = get_field_circle(&magnet, &point2).unwrap();
    println!("test_field: {}", test_field);

    println!("RIGHT");
    let point2 = Point2::new(magnet.radius, 0.0);
    let test_field = get_field_circle(&magnet, &point2).unwrap();
    println!("test_field: {}", test_field);

    println!("BOTTOM");
    let point2 = Point2::new(0.0, -magnet.radius);
    let test_field = get_field_circle(&magnet, &point2).unwrap();
    println!("test_field: {}", test_field);

    println!("LEFT");
    let point2 = Point2::new(-magnet.radius, 0.0);
    let test_field = get_field_circle(&magnet, &point2).unwrap();
    println!("test_field: {}", test_field);
}
