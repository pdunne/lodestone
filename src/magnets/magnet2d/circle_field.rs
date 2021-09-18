/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */
//! # Infinitely long bipolar rod (circle)
//!
//! Contains magnet field routines for calculating the magnetic field in 2D
//! for this structure
//!
use crate::magnets::magnet2d::Circle;
use crate::points::{Point2, Points2, PolarPoint, PolarPoints};
use crate::utils::conversions::vector_pol2cart;
use crate::{MagnetError, FP_CUTOFF, M2_PI};

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
    let polar_val = (*point - magnet.center).to_polar();

    let local_polar = polar_val - PolarPoint::new(0.0, magnet.phi.to_radians());

    let polar_field = get_polar_field_circle(magnet, &local_polar)?;

    let mut field = vector_pol2cart(&polar_field, &polar_val.phi());

    if magnet.alpha.to_radians().abs() > FP_CUTOFF {
        let reverse_angle = M2_PI - magnet.alpha.to_radians();
        field = field.rotate(&reverse_angle);
    }
    Ok(field)
}

#[cfg(test)]
mod tests {
    use super::{get_field_circle, get_polar_field_circle, Circle};
    use crate::points::{Point2, PolarPoint};
    use crate::utils::comparison::nearly_equal;
    use crate::utils::conversions::Angle;
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

        let comp_field = PolarPoint {
            rho: 0.0_f64,
            phi: 0.5_f64,
        };
        assert!(nearly_equal(field.rho, comp_field.rho));
        assert!(nearly_equal(field.phi, comp_field.phi));
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

        assert!(nearly_equal(field.rho, comp_field.rho));
        assert!(nearly_equal(field.phi, comp_field.phi));
    }

    #[test]
    fn surface_field_y_rot_0() {
        let magnet = Circle::new(
            1.0,
            (0.0, 0.0),
            Angle::Degrees(0.0),
            1.0,
            Angle::Degrees(90.0),
        );
        let point1 = Point2::new(0.0, 1.0);

        let field = get_field_circle(&magnet, &point1).unwrap();
        let comp_field = Point2::new(0.0, 0.5);
        assert!(nearly_equal(field.x, comp_field.x));
        assert!(nearly_equal(field.y, comp_field.y));
    }

    #[test]
    fn surface_field_y_rot_90() {
        let magnet = Circle::new(
            1.0,
            (0.0, 0.0),
            Angle::Degrees(90.0),
            1.0,
            Angle::Degrees(90.0),
        );
        let point1 = Point2::new(0.0, 1.0);

        let field = get_field_circle(&magnet, &point1).unwrap();
        let comp_field = Point2::new(0.5, 0.0);

        assert!(nearly_equal(field.x, comp_field.x));
        assert!(nearly_equal(field.y, comp_field.y));
    }

    #[test]
    fn surface_field_x_rot_90() {
        let magnet = Circle::new(
            1.0,
            (0.0, 0.0),
            Angle::Degrees(90.0),
            1.0,
            Angle::Degrees(0.0),
        );
        let point1 = Point2::new(0.0, 1.0);

        let field = get_field_circle(&magnet, &point1).unwrap();
        let comp_field = Point2::new(0.0, 0.5);
        assert!(nearly_equal(field.x, comp_field.x));
        assert!(nearly_equal(field.y, comp_field.y));
    }
}
