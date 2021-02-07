use crate::{NAN, PI};
const ERRTOL: f64 = 1e-6;

pub fn cel(kc: f64, p: f64, c: f64, s: f64) -> f64 {
    /* Burlich's complete elliptic integral
    See NIST Handbook of Mathematical Functions, http://dlmf.nist.gov/19.2
    */

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
