use crate::utils::points2::{Point2, Points2, PolarPoint};

/// Converts a cartesian coordinate to polar
pub fn cart2pol(point: Point2) -> PolarPoint {
    let rho = point.magnitude();
    let phi = point.y.atan2(point.x);
    PolarPoint::new(rho, phi)
}

/// Converts a polar coordinate to cartesian
pub fn pol2cart(point: PolarPoint) -> Point2 {
    let x = point.rho * point.phi.cos();
    let y = point.rho * point.phi.sin();
    Point2 { x: x, y: y }
}

/// Converts polar vectors to cartesian vectors
pub fn vector_pol2cart(vector: PolarPoint, phi: f64) -> Point2 {
    let vector_x = vector.rho * phi.cos() - vector.phi * phi.sin();
    let vector_y = vector.rho * phi.sin() + vector.phi * phi.cos();

    Point2 {
        x: vector_x,
        y: vector_y,
    }
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
