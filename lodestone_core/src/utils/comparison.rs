//! # Comparisons
//!
//! Some simple comparison functions needed to for verifying results
use crate::ERR_CUTOFF;

/// Helper function to determine if two floats are equal within a relative
/// tolerance of 1e-12
///
/// Taken from [https://floating-point-gui.de/errors/comparison/](https://floating-point-gui.de/errors/comparison/)
#[allow(clippy::float_cmp)]
pub fn nearly_equal(a: f64, b: f64) -> bool {
    // let ulp = f64::MIN_POSITIVE;
    let minimum_val = ERR_CUTOFF;
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b {
        // Handle infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < minimum_val {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        // diff < (f64::EPSILON * f64::MIN_POSITIVE)
        diff < minimum_val
    } else {
        // Use relative error.
        // (diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON
        (diff / f64::min(abs_a + abs_b, f64::MAX)) < minimum_val
    }
}

pub fn nearly_equal_array(va: &[f64], vb: &[f64]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| nearly_equal(*a,*b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearly_equal() {
        let a = 2.0;
        let b = 10.0 / 5.0;
        assert!(nearly_equal(a, b));
    }

    #[test]
    fn test_nearly_equal_array() {
        let a = vec![1.0, 2.3, 3.004, 4.5];
        let b = vec![1.0, 2.3, 3.004, 4.5];
        assert!(nearly_equal_array(&a, &b));
    }
}
