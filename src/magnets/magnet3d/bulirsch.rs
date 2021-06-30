/* This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at https://mozilla.org/MPL/2.0/.
Copyright 2021 Peter Dunne */

use core::f64;

use crate::{NAN, PI};
const ERRTOL: f64 = 1e-6;

/// Bulirsch's  complete elliptic integral
/// See NIST Handbook of Mathematical Functions, [http://dlmf.nist.gov/19.2](http://dlmf.nist.gov/19.2)
/// ```math
/// C\left(k_c, p, c,s \right) = \int_0 ^{\pi/2} \frac{ \left( c \cos^2 \varphi + s \sin^s \varphi  \right) d\varphi }
/// {\left( \cos^2 \varphi + p \sin^2 \varphi \right) \sqrt{ \cos^2 \varphi + k_c^2 \sin^2 \varphi  } }
/// ```
/// The three standard Legendre forms of the complete elliptic integrals
/// can be written using the generalised complete elliptic integral of
/// Bulirsch:
///
/// $`K(k) = C(k_c, 1, 1, 1)`$
///
/// $`E(K) = C(k_c, 1, 1, k_c^2)`$
///
/// $`\Pi(n, k) = C(k_c, n+1, 1, 1)`$
///
///
/// # Example
/// Here is an example of how to use it, for the special case of
/// $`k_c = 1`$ for the first complete elliptic integral, $`K(1)`$:
///
/// $` C\left(1, 1, 1, 1 \right)  = \pi/2 `$
///
/// ```rust
/// use magnet_rs::magnets::bulirsch::cel;
/// assert_eq!(cel(1.0, 1.0, 1.0, 1.0), std::f64::consts::FRAC_PI_2);
/// ```
pub fn cel(kc: f64, p: f64, c: f64, s: f64) -> f64 {
    if kc.abs() == 0.0 {
        NAN
    } else {
        let mut k = kc.abs();
        let mut pp = p;
        let mut cc = c;
        let mut ss = s;
        let mut em = 1.0;

        let mut kk = k;
        let mut f;
        let mut g;

        if p > 0.0 {
            pp = p.sqrt();
            ss = s / pp;
        } else {
            f = kc.powi(2);
            let mut q = 1.0 - f;
            g = 1.0 - pp;
            f -= pp;
            q *= ss - c * pp;
            pp = (f / g).sqrt();
            cc = (c - ss) / g;
            ss = -q / (g * g * pp) + cc * pp;
        }
        f = cc;
        cc += ss / pp;

        g = k / pp;

        ss = 2.0 * (ss + f * g);
        pp += g;
        g = em;
        em += k;

        while (g - k).abs() > g * ERRTOL {
            k = 2.0 * kk.sqrt();
            kk = k * em;
            f = cc;
            cc += ss / pp;
            g = kk / pp;
            ss = 2.0 * (ss + f * g);
            pp += g;
            g = em;
            em += k;
        }
        (PI / 2.0) * (ss + cc * em) / (em * (em + pp))
    }
}

#[cfg(test)]
mod tests {
    // use crate::utils::nearly_equal;
    use crate::magnets::magnet3d::bulirsch::cel;

    #[test]
    fn cel_all_ones() {
        assert_eq!(cel(1.0, 1.0, 1.0, 1.0), std::f64::consts::FRAC_PI_2);
    }
}
