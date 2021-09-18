use crate::points::Point2;

/// Rotates a 2D point `Point2` about an angle `alpha`
pub fn rotate_point2(point: &Point2, alpha: &f64) -> Point2 {
    let x_rot = point.x * alpha.cos() - point.y * alpha.sin();
    let y_rot = point.x * alpha.sin() + point.y * alpha.cos();
    Point2::new(x_rot, y_rot)
}

/// Rotates a 2D point tuple`(x,y)` about an angle `alpha`
pub fn rotate_tuple2(point: (&f64, &f64), alpha: &f64) -> (f64, f64) {
    let x_rot = point.0 * alpha.cos() - point.1 * alpha.sin();
    let y_rot = point.0 * alpha.sin() + point.1 * alpha.cos();
    (x_rot, y_rot)
}

#[cfg(test)]
mod tests {
    use super::{rotate_point2, rotate_tuple2, Point2};
    use crate::utils::comparison::nearly_equal;

    #[test]
    fn test_rotate_point2_90() {
        let point = Point2::new(1.0, 1.0);
        let rot_point = rotate_point2(&point, &90.0_f64.to_radians());
        assert_eq!(-0.9999999999999999_f64, rot_point.x);
        assert_eq!(1.0, rot_point.y);
    }

    #[test]
    fn test_rotate_point2_180() {
        let point = Point2::new(1.0, 1.0);
        let rot_point = rotate_point2(&point, &180.0_f64.to_radians());
        assert_eq!(-1.0000000000000002_f64, rot_point.x);
        assert_eq!(-0.9999999999999999_f64, rot_point.y);
    }

    #[test]
    fn test_rotate_point2_270() {
        let point = Point2::new(1.0, 1.0);
        let rot_point = rotate_point2(&point, &270.0_f64.to_radians());
        assert_eq!(0.9999999999999998_f64, rot_point.x);
        assert_eq!(-1.0000000000000002_f64, rot_point.y);
    }

    #[test]
    fn test_rotate_tuple2_90() {
        let point = (&1.0, &1.0);
        let rot_point = rotate_tuple2(point, &90.0_f64.to_radians());
        assert_eq!(-0.9999999999999999_f64, rot_point.0);
        assert_eq!(1.0_f64, rot_point.1);
    }

    #[test]
    fn test_rotate_tuple2_180() {
        let point = (&1.0, &1.0);
        let rot_point = rotate_tuple2(point, &180.0_f64.to_radians());
        assert_eq!(-1.0000000000000002_f64, rot_point.0);
        assert_eq!(-0.9999999999999999_f64, rot_point.1);
    }

    #[test]
    fn test_rotate_tuple2_270() {
        let point = (&1.0, &1.0);
        let rot_point = rotate_tuple2(point, &270.0_f64.to_radians());
        assert_eq!(0.9999999999999998_f64, rot_point.0);
        assert_eq!(-1.0000000000000002_f64, rot_point.1);
    }

    #[test]
    fn test_rotate_tuple2_45_x_axis() {
        let point = (&1.0, &0.0);
        let rot_point = rotate_tuple2(point, &45.0_f64.to_radians());

        let rot_x = nearly_equal(1.0 / 2.0_f64.sqrt(), rot_point.0);
        let rot_y = nearly_equal(1.0 / 2.0_f64.sqrt(), rot_point.1);
        println!("Result: {:?}", rot_point);
        assert!(rot_x, "x false");
        assert!(rot_y, "y false");

        // assert_eq!(1.0 / 2.0_f64.sqrt(), rot_point.0);
        // assert_eq!(1.0 / 2.0_f64.sqrt(), rot_point.1);
    }

    #[test]
    fn test_rotate_tuple2_45_yaxis() {
        let point = (&0.0, &1.0);
        let alpha = -45.0_f64.to_radians();
        println!("Alpha : {}", alpha);
        let rot_point = rotate_tuple2(point, &alpha);

        let rot_x = nearly_equal(1.0 / 2.0_f64.sqrt(), rot_point.0);
        let rot_y = nearly_equal(1.0 / 2.0_f64.sqrt(), rot_point.1);
        println!("Result: {:?}", rot_point);
        assert!(rot_x, "x false");
        assert!(rot_y, "y false");

        // assert_eq!(1.0 / 2.0_f64.sqrt(), rot_point.0);
        // assert_eq!(1.0 / 2.0_f64.sqrt(), rot_point.1);
    }
}
