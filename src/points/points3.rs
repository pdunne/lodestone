// use crate::utils::points::Points;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Point3 {
        Point3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

impl fmt::Display for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// Traits:
pub trait Points3 {
    type Output;
    fn add_p(&self, other: &Self) -> Self::Output;
    fn sub_p(&self, other: &Self) -> Self::Output;
    fn mul_p(&self, other: &Self) -> Self::Output;
    fn div_p(&self, other: &Self) -> Self::Output;
    fn neg_p(&self) -> Self::Output;
    fn scale(&self, s: f64) -> Self::Output;
    fn round(&self) -> Self::Output;

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;

    fn with_x(&self, x: f64) -> Self::Output;
    fn with_y(&self, y: f64) -> Self::Output;
    fn with_z(&self, z: f64) -> Self::Output;

    fn magnitude(&self) -> f64;
    fn magnitude_squared(&self) -> f64;
    fn distance_from_origin(&self) -> f64;
    fn distance_from_point(&self, other: &Self) -> f64;

    fn dot(&self, other: &Self) -> f64;
    fn unit(&self) -> Self::Output;
    fn zero() -> Self::Output;
    fn identity() -> Self::Output;
    fn i_hat() -> Self::Output;
    fn j_hat() -> Self::Output;
    fn k_hat() -> Self::Output;
}

impl Points3 for Point3 {
    type Output = Point3;

    fn add_p(&self, other: &Self) -> Point3 {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn sub_p(&self, other: &Self) -> Point3 {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn mul_p(&self, other: &Self) -> Point3 {
        Point3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    fn div_p(&self, other: &Self) -> Point3 {
        Point3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }

    fn neg_p(&self) -> Point3 {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    fn scale(&self, s: f64) -> Point3 {
        Point3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    fn round(&self) -> Point3 {
        Point3 {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn with_x(&self, x: f64) -> Point3 {
        Point3 {
            x,
            y: self.y,
            z: self.z,
        }
    }

    fn with_y(&self, y: f64) -> Point3 {
        Point3 {
            x: self.x,
            y,
            z: self.z,
        }
    }

    fn with_z(&self, y: f64) -> Point3 {
        Point3 {
            x: self.x,
            y,
            z: self.z,
        }
    }

    fn magnitude_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    fn distance_from_origin(&self) -> f64 {
        self.magnitude()
    }

    fn distance_from_point(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn unit(&self) -> Point3 {
        self.scale(1.0 / self.magnitude())
    }

    fn zero() -> Point3 {
        Point3 {
            x: 0.0_f64,
            y: 0.0_f64,
            z: 0.0_f64,
        }
    }

    fn identity() -> Point3 {
        Point3 {
            x: 1.0_f64,
            y: 1.0_f64,
            z: 1.0_f64,
        }
    }

    fn i_hat() -> Point3 {
        Point3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn j_hat() -> Point3 {
        Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    fn k_hat() -> Point3 {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.add_p(&other)
    }
}

impl Sub for Point3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.sub_p(&other)
    }
}

impl Mul for Point3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.mul_p(&other)
    }
}

impl Div for Point3 {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.div_p(&other)
    }
}

impl Neg for Point3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.neg_p()
    }
}

#[cfg(test)]
mod tests {
    use crate::points::{Point3, Points3};
    use crate::utils::comparison::nearly_equal;

    #[test]
    fn sum_points() {
        let sum = Point3::i_hat() + Point3::j_hat() + Point3::k_hat();
        assert_eq!(
            Point3 {
                x: 1.0,
                y: 1.0,
                z: 1.0
            },
            sum
        );
    }

    #[test]
    fn sub_points() {
        let p1 = Point3 {
            x: 2.0,
            y: 4.0,
            z: 5.2,
        };
        let p2 = Point3 {
            x: 0.3,
            y: 1.5,
            z: 3.1,
        };
        let sub = p1 - p2;
        assert_eq!(
            Point3 {
                x: 1.7,
                y: 2.5,
                z: 2.1
            },
            sub
        );
    }

    #[test]
    fn mul_points() {
        let p1 = Point3 {
            x: 2.0,
            y: 4.0,
            z: 5.2,
        };
        let p2 = Point3 {
            x: 0.3,
            y: 1.5,
            z: 3.1,
        };
        let result = p1 * p2;
        assert_eq!(
            Point3 {
                x: 0.6,
                y: 6.0,
                z: 16.12
            },
            result
        );
    }

    #[test]
    fn div_points() {
        let p1 = Point3 {
            x: 2.0,
            y: 4.0,
            z: 5.0,
        };
        let p2 = Point3 {
            x: 0.2,
            y: 1.6,
            z: 2.0,
        };
        let result = p1 / p2;
        assert_eq!(
            Point3 {
                x: 10.0,
                y: 2.5,
                z: 2.5
            },
            result
        );
    }

    #[test]
    fn neg_point() {
        let p1 = Point3 {
            x: 2.0,
            y: 4.0,
            z: 5.0,
        };
        let result = -p1;
        assert_eq!(
            Point3 {
                x: -2.0,
                y: -4.0,
                z: -5.0
            },
            result
        );
    }

    #[test]
    fn scale_point() {
        let p1 = Point3 {
            x: 2.0,
            y: 4.0,
            z: 5.0,
        };
        let result = p1.scale(3.0);
        assert_eq!(
            Point3 {
                x: 6.0,
                y: 12.0,
                z: 15.0
            },
            result
        );
    }

    #[test]
    fn round_point() {
        let p1 = Point3 {
            x: 2.33,
            y: 4.55,
            z: -5.13,
        };
        let result = p1.round();
        assert_eq!(
            Point3 {
                x: 2.0,
                y: 5.0,
                z: -5.0
            },
            result
        );
    }

    #[test]
    fn unit_magnitude_squared() {
        let mag_squared = Point3::i_hat().magnitude_squared();
        assert_eq!(1., mag_squared);
    }

    #[test]
    fn magnitude_point() {
        let p1 = Point3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let result = p1.magnitude();
        assert_eq!(50_f64.sqrt(), result);
    }

    #[test]
    fn distance_from_origin_point() {
        let p1 = Point3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let result = p1.distance_from_origin();
        assert_eq!(50_f64.sqrt(), result);
    }

    #[test]
    fn distance_from_point_test() {
        let p1 = Point3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let p2 = Point3 {
            x: 4.5,
            y: -3.2,
            z: 7.1,
        };
        let result = p1.distance_from_point(&p2);
        assert_eq!(58.5_f64.sqrt(), result);
    }

    #[test]
    fn dot_product() {
        let p1 = Point3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let p2 = Point3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let result = p1.dot(&p2);
        assert_eq!(26.0, result);
    }

    #[test]
    fn unit_vector() {
        let p1 = Point3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let norm_p1 = p1.unit();

        let result = nearly_equal(norm_p1.x, 3.0 / 50_f64.sqrt())
            && nearly_equal(norm_p1.y, 4.0 / 50_f64.sqrt())
            && nearly_equal(norm_p1.z, 5.0 / 50_f64.sqrt());
        assert!(result);
    }
}
