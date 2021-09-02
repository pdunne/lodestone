/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

use crate::points::Point2;
use crate::{MagnetError, M4_PI};

/// Returns magnetic field due to an infinite current sheet
pub fn sheet_field(x: &f64, y: &f64, h: &f64, kr: &f64) -> Result<Point2, MagnetError> {
    let x_sq = x.powi(2);
    let y_sq = y.powi(2);
    let h_sq = h.powi(2);
    let prefac = kr / M4_PI;

    let y_plus_h = y + h;
    let y_minus_h = y - h;

    let bx = prefac * ((x_sq + y_minus_h.powi(2)) / (x_sq + y_plus_h.powi(2))).ln();

    let by = 2.0 * prefac * (2.0 * h * x).atan2(x_sq + y_sq - h_sq);

    Ok(Point2::new(bx, by))
}

// pub fn signed_area_2d(Vec<()>)

#[cfg(test)]
mod tests {
    use crate::magnets::sheet_field;
    use crate::points::Point2;
    // use crate::utils::comparison::nearly_equal;
    use crate::PI;

    #[test]
    fn test_mid_point() {
        let point = Point2::new(0.0, 0.0);
        let h = 1.0;
        let kr = 1.0;
        let field = sheet_field(&point.x, &point.y, &h, &kr).unwrap();

        let comp_field = Point2::new(0.0, 0.5);
        assert_eq!(field, comp_field);
    }

    #[test]
    fn test_offset_right() {
        let point = Point2::new(0.5, 0.0);
        let h = 1.0;
        let kr = 1.0;
        let field = sheet_field(&point.x, &point.y, &h, &kr).unwrap();
        let comp_field = Point2::new(0.0, 2.0_f64.atan2(1.0) / PI);
        assert_eq!(field, comp_field);
    }
}
