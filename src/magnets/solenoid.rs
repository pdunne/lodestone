use crate::magnets::burlisch::cel;
use crate::PI;
// use crate::magnets::

pub fn solenoid_field(a: f64, b: f64, rho: f64, z: f64, b_0: f64) -> (f64, f64) {
    /* Calculate the magnetic field
        due to a solenoid at any point
        returns Bz,Br
    */
    let zp = z + b;
    let zn = z - b;

    let rho_a = rho + a;
    let rho_na = rho - a;

    let alpha_p = a / (zp.powi(2) + (rho_a).powi(2)).sqrt();
    let alpha_n = a / (zn.powi(2) + (rho_a).powi(2)).sqrt();

    let beta_p = zp / (zp.powi(2) + (rho_a).powi(2)).sqrt();
    let beta_n = zn / (zn.powi(2) + (rho_a).powi(2)).sqrt();

    let gamma = (rho_na) / (rho_a);

    let kp = ((zp.powi(2) + (rho_na).powi(2)) / (zp.powi(2) + (rho_a).powi(2))).sqrt();

    let kn = ((zn.powi(2) + (rho_na).powi(2)) / (zn.powi(2) + (rho_a).powi(2))).sqrt();

    let b_r = (b_0 / PI) * (alpha_p * cel(kp, 1.0, 1.0, -1.0) - alpha_n * cel(kn, 1.0, 1.0, -1.0));

    let b_z = ((b_0 / PI) * a / (rho_a))
        * (beta_p * cel(kp, gamma.powi(2), 1.0, gamma)
            - beta_n * cel(kn, gamma.powi(2), 1.0, gamma));

    (b_z, b_r)
}
