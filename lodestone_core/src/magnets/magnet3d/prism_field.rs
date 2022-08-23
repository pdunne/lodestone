/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

//! # Infinitely long bipolar rod (circle)
//!
//! Contains magnet field routines for calculating the magnetic field in 2D
//! for this structure
//!

#![allow(clippy::many_single_char_names)]
use crate::magnets::magnet3d::Prism;
use crate::points::Point3;
use crate::{MagnetError, I_4PI};

/// Returns the x component of the magnetic field for a prism magnetised in x
fn get_field_x_mag_x(magnet: &Prism, point: &Point3) -> f64 {
    let x = point.x;
    let y = point.y;
    let z = point.z;

    let a = magnet.a;
    let b = magnet.b;
    let c = magnet.c;
    let jr = magnet.jx;

    -(jr * I_4PI)
        * (_f1(a, b, c, -x, y, z)
            + _f1(a, b, c, -x, y, -z)
            + _f1(a, b, c, -x, -y, z)
            + _f1(a, b, c, -x, -y, -z)
            + _f1(a, b, c, x, y, z)
            + _f1(a, b, c, x, y, -z)
            + _f1(a, b, c, x, -y, z)
            + _f1(a, b, c, x, -y, -z))
}

/// Returns the y component of the magnetic field for a prism magnetised in x
fn get_field_y_mag_x(magnet: &Prism, point: &Point3) -> f64 {
    let x = point.x;
    let y = point.y;
    let z = point.z;

    let a = magnet.a;
    let b = magnet.b;
    let c = magnet.c;
    let jr = magnet.jx;
    let top = _f2(a, b, c, -x, -y, z) * _f2(a, b, c, x, y, z);
    let bottom = _f2(a, b, c, -x, y, z) * _f2(a, b, c, x, -y, z);
    (jr * I_4PI) * (top / bottom).ln()
}

/// Returns the z component of the magnetic field for a prism magnetised in x
fn get_field_z_mag_x(magnet: &Prism, point: &Point3) -> f64 {
    let x = point.x;
    let y = point.y;
    let z = point.z;

    let a = magnet.a;
    let b = magnet.b;
    let c = magnet.c;
    let jr = magnet.jx;

    let top = _f2(a, c, b, -x, -z, y) * _f2(a, c, b, x, z, y);
    let bottom = _f2(a, c, b, -x, z, y) * _f2(a, c, b, x, -z, y);

    (jr * I_4PI) * (top / bottom).ln()
}

/// Returns the magnetic field vector at a point due to a prism magnetised in x
fn prism_field_x(magnet: &Prism, point: &Point3) -> Result<Point3, MagnetError> {
    let field = Point3 {
        x: get_field_x_mag_x(magnet, point),
        y: get_field_y_mag_x(magnet, point),
        z: get_field_z_mag_x(magnet, point),
    };
    Ok(field)
}

/// Returns the magnetic field vector at a point due to a prism magnetised in x
fn prism_field_y(magnet: &Prism, point: &Point3) -> Result<Point3, MagnetError> {
    let field = Point3 {
        x: get_field_x_mag_x(magnet, point),
        y: get_field_y_mag_x(magnet, point),
        z: get_field_z_mag_x(magnet, point),
    };
    Ok(field)
}

/// Internal function F1 used for calculating magnetic fields due to cuboids
fn _f1(a: f64, b: f64, c: f64, x: f64, y: f64, z: f64) -> f64 {
    let ax = a + x;
    let by = b + y;
    let cz = c + z;

    let top = by * cz;
    let bottom = ax * (ax.powi(2) + by.powi(2) + cz.powi(2)).sqrt();

    let result = top.atan2(bottom);
    if result.is_finite() {
        result
    } else {
        0.0
    }
}

/// Internal function F2 used for calculating magnetic fields due to cuboids
fn _f2(a: f64, b: f64, c: f64, x: f64, y: f64, z: f64) -> f64 {
    let ax_sq = (a + x).powi(2);
    let by_sq = (b + y).powi(2);
    let cz_sq = (c + z).powi(2);
    let cmz_sq = (c - z).powi(2);

    let top = (ax_sq + by_sq + cmz_sq).sqrt() + c - x;
    let bottom = (ax_sq + by_sq + cz_sq).sqrt() - c - z;

    // Check to avoid singularities
    if bottom.is_finite() {
        top / bottom
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        magnets::MagnetTrait, points::SphericalPoint, utils::comparison::nearly_equal, PI_2,
    };

    #[test]
    fn test_f1() {
        let point = Point3::new(0.0, 0.0, 1.0);

        let val = _f1(1.0, 1.0, 1.0, point.x, point.y, point.z);
        let comp_val = 2.0_f64.atan2((1.0 + 1.0 + 2.0_f64.powi(2)).sqrt());
        assert!(nearly_equal(comp_val, val));
    }

    #[test]
    fn test_f2() {
        let point = Point3::new(1.0, 1.0, 1.0);
        let val = _f2(1.0, 1.0, 1.0, point.x, point.y, point.z);
        let comp_val = 2.0_f64.sqrt() / (3.0_f64.sqrt() - 1.0);
        assert!(nearly_equal(comp_val, val));
    }

    //TODO:Complete
    #[test]
    fn test_get_field_x_mag_x() {
        let magnet = Prism::default().with_magnetisation(SphericalPoint::new(1.0, 0.0, PI_2));
        let point = Point3::new(1.0, 0.0, 0.0);
        let val = get_field_x_mag_x(&magnet, &point);
        println!("Magnet is: {}", magnet);
        println!("val is: {}", val);
        println!("J is {} {} {}", magnet.jx, magnet.jy, magnet.jz);
        assert!(true)
    }
}
