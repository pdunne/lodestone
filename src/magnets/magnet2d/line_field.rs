/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

use crate::utils::points2::Point2;
use crate::PI;

pub fn sheet_field(x: f64, y: f64, h: f64, kr: f64) -> Result<Point2, String> {
    let x_sq = x.powi(2);
    let y_sq = y.powi(2);
    let h_sq = h.powi(2);
    let prefac = kr / 4.0 / PI;

    let y_plus_h = y + h;
    let y_minus_h = y - h;

    let bx = prefac * ((x_sq + y_minus_h.powi(2)) / (x_sq + y_plus_h.powi(2))).ln();

    let by = 2.0 * prefac * (2.0 * h * x).atan2(x_sq + y_sq - h_sq);

    Ok(Point2::new(bx, by))
}
