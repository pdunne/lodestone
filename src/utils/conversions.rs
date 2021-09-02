use crate::points::{Point2, Points2, PolarPoint};
use serde_derive::{Deserialize, Serialize};

/// Angle enum for converting between radians and degrees
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum Angle {
    Degrees(f64),
    Radians(f64),
}

impl Angle {
    /// Converts radian angle to degrees as a float. If the angle is already in degrees,
    /// it returns itself
    pub fn to_degrees(self) -> f64 {
        match self {
            Angle::Radians(val) => val.to_degrees(),
            Angle::Degrees(val) => val,
        }
    }

    /// Converts degree angle to radians as a float. If the angle is already in radians,
    /// it returns itself
    pub fn to_radians(self) -> f64 {
        match self {
            Angle::Degrees(val) => val.to_radians(),
            Angle::Radians(val) => val,
        }
    }
}

/// Converts a cartesian coordinate to polar
pub fn cart2pol(point: &Point2) -> PolarPoint {
    let rho = point.magnitude();
    let phi = point.y.atan2(point.x);
    PolarPoint::new(rho, phi)
}

/// Converts a polar coordinate to cartesian
pub fn pol2cart(point: &PolarPoint) -> Point2 {
    let x = point.rho * point.phi.cos();
    let y = point.rho * point.phi.sin();
    Point2 { x, y }
}

/// Converts polar vectors to cartesian vectors
pub fn vector_pol2cart(vector: &PolarPoint, phi: &f64) -> Point2 {
    let cos_phi = phi.cos();
    let sin_phi = phi.sin();

    let vector_x = vector.rho * cos_phi - vector.phi * sin_phi;
    let vector_y = vector.rho * sin_phi + vector.phi * cos_phi;

    Point2 {
        x: vector_x,
        y: vector_y,
    }
}

/// Rotates a 2D point, `Point2` about a pivot point
pub fn rotate_around_pivot(&point: &Point2, phi: &f64, pivot: &Point2) -> Point2 {
    let cos_val = phi.cos();
    let sin_val = phi.sin();
    let x = point.x - pivot.x;
    let y = point.y - pivot.y;

    let x_rot = (x * cos_val - y * sin_val) + pivot.x;
    let y_rot = (x * sin_val + y * cos_val) + pivot.y;

    Point2 { x: x_rot, y: y_rot }
}

/// Rotates a 2D point, `Point2` about the origin
pub fn rotate_around_origin(&point: &Point2, phi: &f64) -> Point2 {
    let cos_val = phi.cos();
    let sin_val = phi.sin();
    let x = point.x;
    let y = point.y;

    let x_rot = x * cos_val - y * sin_val;
    let y_rot = x * sin_val + y * cos_val;

    Point2 { x: x_rot, y: y_rot }
}

// def cart2sph(x, y, z):
//     """Converts from cartesian to spherical coordinates
//
//     Args:
//         x (ndarray): x coordinates
//         y (ndarray): y coordinates
//         z (ndarray): z coordinates
//
//     Returns:
//         tuple: r, theta, phi
//     """
//     r = _np.sqrt(x ** 2 + y ** 2 + z ** 2)
//     phi = _np.arctan2(y, x)
//
//     # Hide the warning for situtations where there is a divide by zero.
//     # This returns a NaN in the array, which is ignored for plotting.
//     with _np.errstate(divide="ignore", invalid="ignore"):
//         theta = _np.arccos(z / r)
//     return (r, theta, phi)
//
//
// def sph2cart(r, theta, phi):
//     """Converts from spherical to cartesian coordinates
//
//     Args:
//         r (ndarray): radial coordinates
//         theta (ndarray): azimuthal angles
//         phi (ndarray): polar angle
//
//     Returns:
//         tuple: x,y,z
//     """
//     x = r * _np.sin(theta) * _np.cos(phi)
//     y = r * _np.sin(theta) * _np.sin(phi)
//     z = r * _np.cos(theta)
//     return x, y, z
//
//
// def vector_sph2cart(Br, Btheta, Bphi, theta, phi):
//     """Converts Vectors from spherical to cartesian coordinates
//
//     Args:
//         Br (ndarray): radial vector component
//         Btheta (ndarray): polar vector component
//         Bphi (ndarray): azimuthal vector component
//         theta (ndarray): azimuthal angles
//         phi (ndarray): polar angle
//
//     Returns:
//         tuple: Bx,By,Bz
//     """
//     Bx = (
//         Br * _np.sin(theta) * _np.cos(phi)
//         + Btheta * _np.cos(theta) * _np.cos(phi)
//         - Bphi * _np.sin(phi)
//     )
//
//     By = (
//         Br * _np.sin(theta) * _np.sin(phi)
//         + Btheta * _np.cos(theta) * _np.sin(phi)
//         + Bphi * _np.cos(phi)
//     )
//
//     Bz = Br * _np.cos(theta) - Btheta * _np.sin(theta)
//     return Bx, By, Bz
//
//
// def sphere_sph2cart(Br, Btheta, theta, phi):
//     """Converts magnetic field of a sphere from spherical to cartesian coordinates
//
//     Args:
//         Br (ndarray): radial vector component
//         Btheta (ndarray): polar vector component
//         theta (ndarray): azimuthal angles
//         phi (ndarray): polar angle
//
//     Returns:
//         tuple: Bx,By,Bz
//     """
//     Bx = Br * _np.sin(theta) * _np.cos(phi) + Btheta * _np.cos(theta) * _np.cos(phi)
//
//     By = Br * _np.sin(theta) * _np.sin(phi) + Btheta * _np.cos(theta) * _np.sin(phi)
//
//     Bz = Br * _np.cos(theta) - Btheta * _np.sin(theta)
//     return Bx, By, Bz

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PI, PI_2, PI_4};

    #[test]
    fn test_degrees_to_radians() {
        let angle = Angle::Degrees(90.0);
        assert_eq!(angle.to_radians(), PI_2);
    }

    #[test]
    fn test_degrees_to_degrees() {
        let angle = Angle::Degrees(32.0);
        assert_eq!(angle.to_degrees(), 32.0);
    }

    #[test]
    fn test_radians_to_degrees() {
        let angle = Angle::Radians(PI);
        assert_eq!(angle.to_degrees(), 180.0);
    }

    #[test]
    fn test_radians_to_radians() {
        let angle = Angle::Radians(PI_4);
        assert_eq!(angle.to_radians(), PI_4);
    }
}
