/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! Contains magnet field routines for calculating the magnetic field due to an
//! infinitely long bipolar rod (circle)
//!
//!
use crate::magnets::magnet2d::Circle;
use crate::points::{Point2, Points2, PolarPoint, PolarPoints};
use crate::utils::conversions::vector_pol2cart;
use crate::MagnetError;

/// Calculates the 2D magnetic field of an infintely long bipolar rod (circle)
///
/// The field is returned in a `PolarPoint` struct
pub fn get_polar_field_circle(
    magnet: &Circle,
    point: &PolarPoint,
) -> Result<PolarPoint, MagnetError> {
    let prefac = magnet.jr * (magnet.radius / point.rho).powi(2) / 2.0;
    let b_rho = prefac * point.phi.cos();
    let b_phi = prefac * point.phi.sin();
    let field = PolarPoint::new(b_rho, b_phi);

    Ok(field)
}

pub fn get_field_circle(magnet: &Circle, point: &Point2) -> Result<Point2, MagnetError> {
    let polar_val = point.to_polar();
    let polar_field = get_polar_field_circle(&magnet, &polar_val).unwrap();

    let field = vector_pol2cart(&polar_field, polar_val.phi());

    Ok(field)
}

#[cfg(test)]
mod tests {
    use crate::magnets::magnet2d::circle_field::get_polar_field_circle;
    use crate::magnets::magnet2d::Circle;
    use crate::points::PolarPoint;
    use crate::utils::comparison::nearly_equal;
    use crate::{PI_2, PI_4};

    #[test]
    fn surface_field_x() {
        let magnet = Circle::default();
        let point1 = PolarPoint {
            rho: magnet.radius,
            phi: 0.0,
        };
        let field = get_polar_field_circle(&magnet, &point1).unwrap();
        assert_eq!(PolarPoint { rho: 0.5, phi: 0.0 }, field);
    }

    #[test]
    fn surface_field_y() {
        let magnet = Circle::default();
        let point1 = PolarPoint {
            rho: magnet.radius,
            phi: PI_2,
        };
        let field = get_polar_field_circle(&magnet, &point1).unwrap();
        // println!(field);
        let comp_field = PolarPoint {
            rho: 0.0_f64,
            phi: 0.5_f64,
        };
        let result =
            nearly_equal(field.rho, comp_field.rho) && nearly_equal(field.phi, comp_field.phi);

        assert!(result);
    }

    #[test]
    fn surface_field_45() {
        let magnet = Circle::default();
        let point1 = PolarPoint {
            rho: magnet.radius,
            phi: PI_4,
        };
        let field = get_polar_field_circle(&magnet, &point1).unwrap();

        let comp_field = PolarPoint {
            rho: 0.5_f64 / 2.0_f64.sqrt(),
            phi: 0.5_f64 / 2.0_f64.sqrt(),
        };

        let result =
            nearly_equal(field.rho, comp_field.rho) && nearly_equal(field.phi, comp_field.phi);

        assert!(result);
    }
}
